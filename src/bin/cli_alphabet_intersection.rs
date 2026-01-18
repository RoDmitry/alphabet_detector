#![feature(string_into_chars)]

use ::std::{
    fs,
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};
use ahash::{AHashMap, AHashSet};
use alphabet_detector::{
    ch_norm, reader::ReadChunks, script_char_to_slangs, slang_arr_default, slang_arr_default_nc,
    ucd::BY_NAME, CharData, Language, Script, ScriptLanguage, ScriptLanguageArr, UcdScript,
};
use clap::Parser;
use debug_unsafe::slice::SliceGetter;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short = 'd', required = true)]
    dir: String,
}

const THREADS: usize = 8;
fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(&args.dir).unwrap();
    let pool = threadpool::ThreadPool::new(THREADS);
    let langs_seen = Arc::new(Mutex::new(slang_arr_default::<bool>()));

    for path in paths {
        let langs_seen = langs_seen.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            println!("*{}* New", file_name);
            let Some(thread_lang) = ScriptLanguage::from_str(&file_name) else {
                panic!("*{}* Not found lang", file_name);
            };
            {
                let mut guard = langs_seen.lock().unwrap();
                let lang_seen = guard.get_mut(thread_lang as usize).unwrap();
                if *lang_seen {
                    drop(guard);
                    panic!("*{}* Have already seen lang: {:?}", file_name, thread_lang);
                }
                *lang_seen = true;
            }

            let unknown_lang =
                ScriptLanguage::from_parts((Language::Unknown, Script::from(thread_lang)));
            let thread_script = UcdScript::from(thread_lang);
            let thread_langs = ScriptLanguage::all_with_script(thread_script);
            // TODO: rm this filter
            /* if !matches!(thread_lang, ScriptLanguage::English) {
                return;
            } */
            // TODO: rm this filter
            /* if thread_script != UcdScript::Latin {
                return;
            } */

            println!("*{}* {:?} started", file_name, thread_lang);

            let mut lang_chars = AHashSet::new();
            for &(script, ranges) in BY_NAME {
                if script != thread_script {
                    continue;
                }
                for range in ranges {
                    for ch in range.0..=range.1 {
                        let ch_langs = script_char_to_slangs(thread_script, ch);
                        if (unknown_lang.is_some() || ch_langs != thread_langs)
                            && ch_langs.contains(&thread_lang)
                        {
                            lang_chars.insert(ch);
                        }
                    }
                }
                break;
            }

            let file = BufReader::new(File::open(path.path()).expect("open failed"));
            let ch_iter = file
                .chunks(b'\n')
                .flat_map(|s| s.unwrap().into_chars().map(|c| (0, c)));

            let mut langs_count = slang_arr_default::<usize>();
            let mut found_chars: AHashMap<char, usize> = Default::default();
            let mut not_found_chars: ScriptLanguageArr<AHashMap<char, usize>> =
                slang_arr_default_nc();
            let mut prev_char_script = UcdScript::Common;
            for CharData { mut script, ch, .. } in ch_norm::from_ch_ind(ch_iter) {
                lang_chars.remove(&ch.to_lowercase().next().unwrap());
                lang_chars.remove(&ch.to_uppercase().next().unwrap());

                if script == UcdScript::Inherited {
                    script = prev_char_script;
                } else {
                    prev_char_script = script;
                }

                let langs = script_char_to_slangs(script, ch);
                let mut has_lang = false;
                for &l in langs {
                    has_lang |= l == thread_lang;
                    langs_count[l as usize] += 1;
                }

                if has_lang {
                    if langs != ScriptLanguage::all_with_script(script) {
                        *found_chars.entry(ch).or_default() += 1;
                    }
                } else {
                    for &l in langs {
                        *not_found_chars
                            .get_safe_unchecked_mut(l as usize)
                            .entry(ch)
                            .or_default() += 1;
                    }
                }
            }

            if !lang_chars.is_empty() {
                println!(
                    "*{}* {:?} unused chars found: {:?}",
                    file_name, thread_lang, lang_chars
                );
            }

            let lang_count = langs_count[thread_lang as usize];
            let lang_count_max = langs_count.iter().fold(1, |acc, cnt| acc.max(*cnt));
            /* if lang_count_max == lang_count {
                println!("*{}* {:?} AWESOME!", file_name, thread_lang);
                return;
            } */

            let count_diff = lang_count_max - lang_count;
            let count_percent_diff = (count_diff * 100) as f64 / lang_count_max as f64;
            if count_percent_diff == 0.0 {
                println!(
                    "*{}* {:?} within error bounds {}",
                    file_name, thread_lang, count_percent_diff
                );
                return;
            }

            let mut found_chars_new: AHashMap<char, usize> = Default::default();
            for (ch, cnt) in found_chars {
                let ch_lowered = ch.to_lowercase().next().unwrap();
                *found_chars_new.entry(ch_lowered).or_default() += cnt;
            }
            let mut found_chars_new = found_chars_new.into_iter().collect::<Vec<_>>();
            found_chars_new.sort_by(|a, b| a.1.cmp(&b.1));
            found_chars_new.truncate(10);
            println!(
                "*{}* {:?} found: {:?}",
                file_name, thread_lang, found_chars_new
            );

            /* let mut langs: Vec<(ScriptLanguage, usize)> = langs_count
                .iter()
                .enumerate()
                .filter(|(_, c)| **c > 0)
                .map(|(l, cnt)| (ScriptLanguage::from(l), *cnt))
                .collect();
            langs.sort_by(|a, b| b.1.cmp(&a.1));
            println!("*{}* {:?} langs {:?}", file_name, thread_lang, langs); */

            let top_langs: AHashSet<ScriptLanguage> = langs_count
                .into_iter()
                .enumerate()
                .filter(|(_, cnt)| *cnt > lang_count)
                .map(|(l, _)| unsafe { ScriptLanguage::transmute_from_usize(l) })
                .collect();

            let top_lang_chars: AHashMap<_, _> = not_found_chars
                .iter()
                .enumerate()
                .filter(|(_, c)| !c.is_empty())
                .map(|(l, c)| (unsafe { ScriptLanguage::transmute_from_usize(l) }, c))
                .filter(|(l, _)| top_langs.contains(l))
                .map(|(l, c)| {
                    (l, {
                        let mut res = c.into_iter().collect::<Vec<_>>();
                        res.sort_by(|a, b| b.1.cmp(a.1));
                        res.truncate(10);
                        res
                    })
                })
                .take(2)
                .collect();
            println!(
                "*{}* {:?} top: {:?}",
                file_name, thread_lang, top_lang_chars
            );

            let not_found_chars: AHashSet<_> = not_found_chars
                .into_iter()
                .enumerate()
                .filter(|(_, c)| !c.is_empty())
                .map(|(l, c)| (unsafe { ScriptLanguage::transmute_from_usize(l) }, c))
                .filter(|(l, _)| Some(l) == unknown_lang.as_ref() || thread_langs.contains(l))
                .flat_map(|(_, c)| c.into_iter())
                .collect();
            let mut not_found_chars: Vec<_> = not_found_chars
                .into_iter()
                // .filter(|(_, cnt)| *cnt > (lang_count_max / 15000))
                .collect();
            not_found_chars.sort_by(|a, b| b.1.cmp(&a.1));

            println!(
                "*{}* {:?} {} ({}/{}) {:?}",
                file_name,
                thread_lang,
                count_percent_diff,
                count_diff,
                lang_count_max,
                not_found_chars
            );
        });
    }

    pool.join();
}
