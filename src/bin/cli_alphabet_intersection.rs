use ::std::fs;
use ::std::fs::File;
use ::std::io::BufReader;
use ::std::str::FromStr;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::std::{thread, time::Duration};
use alphabet_detector::read_iter::ReadCharsChunks;
use alphabet_detector::{
    ch_norm_iter, lang_arr_default, script_char_to_langs, str_to_langs, Language,
};
use cap::Cap;
use clap::Parser;

#[global_allocator]
static ALLOCATOR: Cap<::std::alloc::System> = Cap::new(::std::alloc::System, usize::MAX);

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short = 'i', required = true)]
    inp: String,

    /// List of iso-639-1 language codes
    #[arg(
        short,
        long,
        help = "comma seperated list of iso-639-1 codes of languages to detect, if not specified, all supported language will be used. Setting this improves accuracy and resource usage.",
        num_args = 1,
        value_delimiter = ',',
        required = false
    )]
    languages: Vec<String>,

    #[arg(
        short = 'n',
        long,
        help = "Classify language per line, this only works if text is not supplied directly as an argument"
    )]
    per_line: bool,

    #[arg(short = 'L', long, help = "List supported languages")]
    list: bool,

    #[arg(
        short,
        long,
        help = "Show all confidence values (entire probability distribution), rather than just the winning score. Does not work with --multi"
    )]
    all: bool,

    #[arg(short = 'q', long, help = "Quick/low accuracy mode")]
    quick: bool,

    #[arg(
        short = 'm',
        long,
        help = "Classify multiple languages in mixed texts, will return matches along with UTF-8 byte offsets. Can not be combined with line mode."
    )]
    multi: bool,

    #[arg(
        short = 'c',
        long,
        help = "Confidence threshold, only output results with at least this confidence value (0.0-1.0)"
    )]
    confidence: Option<f64>,

    #[arg(
        short = 'M',
        long,
        help = "Minimum text length (without regard for whitespace, punctuation or numerals!). Shorter fragments will be classified as 'unknown'"
    )]
    minlength: Option<u8>,

    #[arg(short = 'p', help = "preload models")]
    preload: bool,

    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    #[arg(required = false)]
    text: Vec<String>,
}

const THREADS: usize = 8;
const MEM_LIMIT_SLEEP: usize = 6 * 1024 * 1024 * 1024;
fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(&args.inp).unwrap();
    let pool = threadpool::ThreadPool::new(THREADS);
    let langs_seen = Arc::new(Mutex::new(lang_arr_default::<bool>()));

    for path in paths {
        let langs_seen = langs_seen.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            println!("*{}* New", file_name);
            while ALLOCATOR.allocated() > MEM_LIMIT_SLEEP {
                println!(
                    "*{}* Mem allocated: {}MB Sleeping...",
                    file_name,
                    ALLOCATOR.allocated() / (1024 * 1024)
                );
                let time = Duration::from_secs(30);
                thread::sleep(time);
            }
            /* println!(
                "*{}* Mem allocated: {}MB",
                file_name,
                ALLOCATOR.allocated() / (1024 * 1024)
            ); */
            {
                let [thread_lang, alph] = file_name.split('_').collect::<Vec<_>>()[..] else {
                    unreachable!()
                };
                let thread_lang = match Language::from_str(&file_name) {
                    Ok(l) => l,
                    _ => {
                        let Ok(l) = Language::from_str(thread_lang) else {
                            panic!("*{}* Not found lang: {}", file_name, thread_lang);
                        };
                        l
                    }
                };
                {
                    let mut guard = langs_seen.lock().unwrap();
                    let lang_seen = guard.get_mut(thread_lang as usize).unwrap();
                    if *lang_seen {
                        drop(guard);
                        panic!("*{}* Have already seen lang: {}", file_name, thread_lang);
                    }
                    *lang_seen = true;
                }

                let thread_langs = str_to_langs(alph);
                if !thread_langs.contains(&thread_lang) {
                    panic!(
                        "*{}* Not found lang: {thread_lang} in {:?}",
                        file_name, thread_langs
                    );
                };
                // TODO: rm this filter
                /* if !matches!(thread_lang, Language::English) {
                    return;
                } */
                // TODO: rm this filter
                if alph != "Latn" {
                    // if alph != "Arab" {
                    return;
                }

                // let model_name = thread_lang.to_string();
                // let mod_dir = stringcase::snake_case(&model_name);

                println!("*{}* {} started", file_name, thread_lang);

                // let text = fs::read_to_string(path.path()).unwrap();
                let file = BufReader::new(File::open(path.path()).expect("open failed"));
                let ch_iter = file.chars_chunks(b'\n').map(|v| (0, v.unwrap()));

                let mut res = lang_arr_default::<usize>();
                let mut res2: ahash::AHashMap<Language, ahash::AHashMap<char, usize>> =
                    Default::default();
                for (script, _, ch) in ch_norm_iter::from_ch_iter(ch_iter) {
                    let langs = script
                        .map(|s| script_char_to_langs(s, ch))
                        .unwrap_or_default();
                    let mut has_lang = false;
                    for &l in langs {
                        has_lang |= l == thread_lang;
                        res[l as usize] += 1;
                    }
                    if !has_lang {
                        for &l in langs {
                            *res2.entry(l).or_default().entry(ch).or_default() += 1;
                        }
                    }
                }

                let res_count = res[thread_lang as usize];
                let lang_count_max = res.iter().fold(1, |acc, cnt| acc.max(*cnt));
                if lang_count_max == res_count {
                    println!("*{}* {} GOOD!", file_name, thread_lang);
                    return;
                }

                let count_diff = lang_count_max - res_count;
                let count_percent = (count_diff * 100) as f64 / lang_count_max as f64;
                if count_percent < 0.5 {
                    println!(
                        "*{}* {} within error bounds {}",
                        file_name, thread_lang, count_percent
                    );
                    return;
                }

                /* let res: Vec<(Language, usize)> = res
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c > 0)
                    .map(|(l, cnt)| (Language::from(l), cnt))
                    .collect();

                let top_langs: ahash::AHashSet<Language> = res
                    .iter()
                    .filter(|(_, cnt)| *cnt == lang_count_max)
                    .map(|(l, _)| *l)
                    .collect(); */
                /* let top_langs: ahash::AHashSet<Language> = res
                .into_iter()
                .enumerate()
                .filter(|(_, cnt)| *cnt == lang_count_max)
                .map(|(l, _)| Language::from(l))
                .collect(); */
                /* if top_langs.contains(&thread_lang) {
                    println!("*{}* {} GOOD!", file_name, thread_lang);
                    return;
                } */
                // res.sort_by(|a, b| b.1.cmp(&a.1));
                // println!("*{}* {} {:?}", file_name, thread_lang, res);

                let res2: ahash::AHashSet<_> = res2
                    .into_iter()
                    .filter(|(_, c)| !c.is_empty())
                    .map(|(l, c)| (Language::from(l), c))
                    .filter(|(l, _)| thread_langs.contains(l))
                    .map(|(_, c)| c.into_iter())
                    .flatten()
                    .collect();
                let mut res2: Vec<_> = res2.into_iter().collect();
                res2.sort_by(|a, b| b.1.cmp(&a.1));

                println!(
                    "*{}* {} {} ({}/{}) {:?}",
                    file_name, thread_lang, count_percent, count_diff, lang_count_max, res2
                );
            }

            /* println!(
                "*{}* malloc_trim {:?} {:?}MB",
                file_name,
                unsafe { libc::malloc_trim(0) },
                ALLOCATOR.allocated() / (1024 * 1024)
            ); */
        });
    }

    pool.join();
}
