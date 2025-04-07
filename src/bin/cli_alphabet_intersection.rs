use ::std::{
    fs,
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};
use alphabet_detector::{
    ch_norm_iter, read_iter::ReadCharsChunks, script_char_to_slangs, slang_arr_default, CharData,
    Script, ScriptLanguage,
};
use clap::Parser;

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
            let thread_lang = match ScriptLanguage::from_str(&file_name) {
                Some(l) => l,
                _ => {
                    let Some(l) = ScriptLanguage::from_str(file_name.split('_').next().unwrap())
                    else {
                        panic!("*{}* Not found lang", file_name);
                    };
                    l
                }
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

            let thread_script = <Option<Script>>::from(thread_lang);
            let thread_langs = thread_script
                .map(|s| script_char_to_slangs(s, char::default()))
                .unwrap_or_default();
            // TODO: rm this filter
            /* if !matches!(thread_lang, ScriptLanguage::English) {
                return;
            } */
            // TODO: rm this filter
            if thread_script != Some(Script::Latin) {
                return;
            }

            println!("*{}* {:?} started", file_name, thread_lang);

            let file = BufReader::new(File::open(path.path()).expect("open failed"));
            let ch_iter = file.chars_chunks(b'\n').map(|v| (0, v.unwrap()));

            let mut langs_count = slang_arr_default::<usize>();
            let mut found_chars: ahash::AHashMap<char, usize> = Default::default();
            let mut not_found_chars: ahash::AHashMap<ScriptLanguage, ahash::AHashMap<char, usize>> =
                Default::default();
            for CharData { script, ch, .. } in ch_norm_iter::from_ch_iter(ch_iter) {
                let langs = script_char_to_slangs(script, ch);
                let mut has_lang = false;
                for &l in langs {
                    has_lang |= l == thread_lang;
                    langs_count[l as usize] += 1;
                }

                if has_lang {
                    if langs != script_char_to_slangs(script, char::default()) {
                        *found_chars.entry(ch).or_default() += 1;
                    }
                } else {
                    for &l in langs {
                        *not_found_chars.entry(l).or_default().entry(ch).or_default() += 1;
                    }
                }
            }

            let lang_count = langs_count[thread_lang as usize];
            let lang_count_max = langs_count.iter().fold(1, |acc, cnt| acc.max(*cnt));
            /* if lang_count_max == lang_count {
                println!("*{}* {:?} AWESOME!", file_name, thread_lang);
                return;
            } */

            let count_diff = lang_count_max - lang_count;
            let count_percent_diff = (count_diff * 100) as f64 / lang_count_max as f64;
            /* if count_percent_diff < 0.1 {
                println!(
                    "*{}* {:?} within error bounds {}",
                    file_name, thread_lang, count_percent_diff
                );
                return;
            } */

            let mut found_chars_new: ahash::AHashMap<char, usize> = Default::default();
            for (ch, cnt) in found_chars {
                let ch_lowered = ch.to_lowercase().next().unwrap();
                let v = found_chars_new.entry(ch_lowered).or_default();
                *v += cnt;
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

            let top_langs: ahash::AHashSet<ScriptLanguage> = langs_count
                .into_iter()
                .enumerate()
                .filter(|(_, cnt)| *cnt > lang_count)
                .map(|(l, _)| ScriptLanguage::from_usize_unchecked(l))
                .collect();

            let top_lang_chars: ahash::AHashMap<_, _> = not_found_chars
                .iter()
                .filter(|(_, c)| !c.is_empty())
                .map(|(l, c)| (ScriptLanguage::from_usize_unchecked(*l), c))
                .filter(|(l, _)| top_langs.contains(l))
                .map(|(l, c)| {
                    (l, {
                        let mut res = c.into_iter().collect::<Vec<_>>();
                        res.sort_by(|a, b| b.1.cmp(&a.1));
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

            let not_found_chars: ahash::AHashSet<_> = not_found_chars
                .into_iter()
                .filter(|(_, c)| !c.is_empty())
                .map(|(l, c)| (ScriptLanguage::from_usize_unchecked(l), c))
                .filter(|(l, _)| thread_langs.contains(l))
                .map(|(_, c)| c.into_iter())
                .flatten()
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
