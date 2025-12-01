use ::core::cmp::Ordering;
use alphabet_detector_macros::Script;
use debug_unsafe::slice::SliceGetter;
use strum_macros::EnumIter;

/// Int representation is unstable and can be changed anytime.
/// Code representation (const
/// [`into_code`](enum.UcdScript.html#method.into_code)/[`from_code`](enum.UcdScript.html#method.from_code))
/// or string representation (const
/// [`into_str`](enum.UcdScript.html#method.into_str)/[`from_str`](enum.UcdScript.html#method.from_str))
/// are more stable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, Script)]
pub enum UcdScript {
    #[scr(short = "Adlm", code = 166)]
    Adlam,
    #[scr(short = "Ahom", code = 338)]
    Ahom,
    #[scr(short = "Hluw", code = 80)]
    AnatolianHieroglyphs,
    #[scr(short = "Arab", code = 160)]
    Arabic,
    #[scr(short = "Armn", code = 230)]
    Armenian,
    #[scr(short = "Avst", code = 134)]
    Avestan,
    #[scr(short = "Bali", code = 360)]
    Balinese,
    #[scr(short = "Bamu", code = 435)]
    Bamum,
    #[scr(short = "Bass", code = 259)]
    BassaVah,
    #[scr(short = "Batk", code = 365)]
    Batak,
    #[scr(short = "Beng", code = 325)]
    Bengali,
    #[scr(short = "Berf", code = 258)]
    BeriaErfe,
    #[scr(short = "Bhks", code = 334)]
    Bhaiksuki,
    #[scr(short = "Bopo", code = 285)]
    Bopomofo,
    #[scr(short = "Brah", code = 300)]
    Brahmi,
    #[scr(short = "Brai", code = 570)]
    Braille,
    #[scr(short = "Bugi", code = 367)]
    Buginese,
    #[scr(short = "Buhd", code = 372)]
    Buhid,
    #[scr(short = "Cans", code = 440)]
    CanadianAboriginal,
    #[scr(short = "Cari", code = 201)]
    Carian,
    #[scr(short = "Aghb", code = 239)]
    CaucasianAlbanian,
    #[scr(short = "Cakm", code = 349)]
    Chakma,
    #[scr(short = "Cham", code = 358)]
    Cham,
    #[scr(short = "Cher", code = 445)]
    Cherokee,
    #[scr(short = "Chrs", code = 109)]
    Chorasmian,
    #[scr(short = "Zyyy", code = 998)]
    Common,
    #[scr(short = "Copt", code = 204)]
    Coptic,
    #[scr(short = "Xsux", code = 20)]
    Cuneiform,
    #[scr(short = "Cprt", code = 403)]
    Cypriot,
    #[scr(short = "Cpmn", code = 402)]
    CyproMinoan,
    #[scr(short = "Cyrl", code = 220)]
    Cyrillic,
    #[scr(short = "Dsrt", code = 250)]
    Deseret,
    #[scr(short = "Deva", code = 315)]
    Devanagari,
    #[scr(short = "Diak", code = 342)]
    DivesAkuru,
    #[scr(short = "Dogr", code = 328)]
    Dogra,
    #[scr(short = "Dupl", code = 755)]
    Duployan,
    #[scr(short = "Egyp", code = 50)]
    EgyptianHieroglyphs,
    #[scr(short = "Elba", code = 226)]
    Elbasan,
    #[scr(short = "Elym", code = 128)]
    Elymaic,
    #[scr(short = "Ethi", code = 430)]
    Ethiopic,
    #[scr(short = "Gara", code = 164)]
    Garay,
    #[scr(short = "Geor", code = 240)]
    Georgian,
    #[scr(short = "Glag", code = 225)]
    Glagolitic,
    #[scr(short = "Goth", code = 206)]
    Gothic,
    #[scr(short = "Gran", code = 343)]
    Grantha,
    #[scr(short = "Grek", code = 200)]
    Greek,
    #[scr(short = "Gujr", code = 320)]
    Gujarati,
    #[scr(short = "Gong", code = 312)]
    GunjalaGondi,
    #[scr(short = "Guru", code = 310)]
    Gurmukhi,
    #[scr(short = "Gukh", code = 397)]
    GurungKhema,
    #[scr(short = "Hani", code = 500)]
    Han,
    #[scr(short = "Hang", code = 286)]
    Hangul,
    #[scr(short = "Rohg", code = 167)]
    HanifiRohingya,
    #[scr(short = "Hano", code = 371)]
    Hanunoo,
    #[scr(short = "Hatr", code = 127)]
    Hatran,
    #[scr(short = "Hebr", code = 125)]
    Hebrew,
    #[scr(short = "Hira", code = 410)]
    Hiragana,
    #[scr(short = "Armi", code = 124)]
    ImperialAramaic,
    #[scr(short = "Zinh", code = 994)]
    Inherited,
    #[scr(short = "Phli", code = 131)]
    InscriptionalPahlavi,
    #[scr(short = "Prti", code = 130)]
    InscriptionalParthian,
    #[scr(short = "Java", code = 361)]
    Javanese,
    #[scr(short = "Kthi", code = 317)]
    Kaithi,
    #[scr(short = "Knda", code = 345)]
    Kannada,
    #[scr(short = "Kana", code = 411)]
    Katakana,
    #[scr(short = "Kawi", code = 368)]
    Kawi,
    #[scr(short = "Kali", code = 357)]
    KayahLi,
    #[scr(short = "Khar", code = 305)]
    Kharoshthi,
    #[scr(short = "Kits", code = 288)]
    KhitanSmallScript,
    #[scr(short = "Khmr", code = 355)]
    Khmer,
    #[scr(short = "Khoj", code = 322)]
    Khojki,
    #[scr(short = "Sind", code = 318)]
    Khudawadi,
    #[scr(short = "Krai", code = 396)]
    KiratRai,
    #[scr(short = "Laoo", code = 356)]
    Lao,
    #[scr(short = "Latn", code = 215)]
    Latin,
    #[scr(short = "Lepc", code = 335)]
    Lepcha,
    #[scr(short = "Limb", code = 336)]
    Limbu,
    #[scr(short = "Lina", code = 400)]
    LinearA,
    #[scr(short = "Linb", code = 401)]
    LinearB,
    #[scr(short = "Lisu", code = 399)]
    Lisu,
    #[scr(short = "Lyci", code = 202)]
    Lycian,
    #[scr(short = "Lydi", code = 116)]
    Lydian,
    #[scr(short = "Mahj", code = 314)]
    Mahajani,
    #[scr(short = "Maka", code = 366)]
    Makasar,
    #[scr(short = "Mlym", code = 347)]
    Malayalam,
    #[scr(short = "Mand", code = 140)]
    Mandaic,
    #[scr(short = "Mani", code = 139)]
    Manichaean,
    #[scr(short = "Marc", code = 332)]
    Marchen,
    #[scr(short = "Gonm", code = 313)]
    MasaramGondi,
    #[scr(short = "Medf", code = 265)]
    Medefaidrin,
    #[scr(short = "Mtei", code = 337)]
    MeeteiMayek,
    #[scr(short = "Mend", code = 438)]
    MendeKikakui,
    #[scr(short = "Merc", code = 101)]
    MeroiticCursive,
    #[scr(short = "Mero", code = 100)]
    MeroiticHieroglyphs,
    #[scr(short = "Plrd", code = 282)]
    Miao,
    #[scr(short = "Modi", code = 324)]
    Modi,
    #[scr(short = "Mong", code = 145)]
    Mongolian,
    #[scr(short = "Mroo", code = 264)]
    Mro,
    #[scr(short = "Mult", code = 323)]
    Multani,
    #[scr(short = "Mymr", code = 350)]
    Myanmar,
    #[scr(short = "Nbat", code = 159)]
    Nabataean,
    #[scr(short = "Nagm", code = 295)]
    NagMundari,
    #[scr(short = "Nand", code = 311)]
    Nandinagari,
    #[scr(short = "Newa", code = 333)]
    Newa,
    #[scr(short = "Talu", code = 354)]
    NewTaiLue,
    #[scr(short = "Nkoo", code = 165)]
    Nko,
    #[scr(short = "Nshu", code = 499)]
    Nushu,
    #[scr(short = "Hmnp", code = 451)]
    NyiakengPuachueHmong,
    #[scr(short = "Ogam", code = 212)]
    Ogham,
    #[scr(short = "Olck", code = 261)]
    OlChiki,
    #[scr(short = "Hung", code = 176)]
    OldHungarian,
    #[scr(short = "Ital", code = 210)]
    OldItalic,
    #[scr(short = "Narb", code = 106)]
    OldNorthArabian,
    #[scr(short = "Perm", code = 227)]
    OldPermic,
    #[scr(short = "Xpeo", code = 30)]
    OldPersian,
    #[scr(short = "Sogo", code = 142)]
    OldSogdian,
    #[scr(short = "Sarb", code = 105)]
    OldSouthArabian,
    #[scr(short = "Orkh", code = 175)]
    OldTurkic,
    #[scr(short = "Ougr", code = 143)]
    OldUyghur,
    #[scr(short = "Onao", code = 296)]
    OlOnal,
    #[scr(short = "Orya", code = 327)]
    Oriya,
    #[scr(short = "Osge", code = 219)]
    Osage,
    #[scr(short = "Osma", code = 260)]
    Osmanya,
    #[scr(short = "Hmng", code = 450)]
    PahawhHmong,
    #[scr(short = "Palm", code = 126)]
    Palmyrene,
    #[scr(short = "Pauc", code = 263)]
    PauCinHau,
    #[scr(short = "Phag", code = 331)]
    PhagsPa,
    #[scr(short = "Phnx", code = 115)]
    Phoenician,
    #[scr(short = "Phlp", code = 132)]
    PsalterPahlavi,
    #[scr(short = "Rjng", code = 363)]
    Rejang,
    #[scr(short = "Runr", code = 211)]
    Runic,
    #[scr(short = "Samr", code = 123)]
    Samaritan,
    #[scr(short = "Saur", code = 344)]
    Saurashtra,
    #[scr(short = "Shrd", code = 319)]
    Sharada,
    #[scr(short = "Shaw", code = 281)]
    Shavian,
    #[scr(short = "Sidd", code = 302)]
    Siddham,
    #[scr(short = "Sidt", code = 180)]
    Sidetic,
    #[scr(short = "Sgnw", code = 95)]
    SignWriting,
    #[scr(short = "Sinh", code = 348)]
    Sinhala,
    #[scr(short = "Sogd", code = 141)]
    Sogdian,
    #[scr(short = "Sora", code = 398)]
    SoraSompeng,
    #[scr(short = "Soyo", code = 329)]
    Soyombo,
    #[scr(short = "Sund", code = 362)]
    Sundanese,
    #[scr(short = "Sunu", code = 274)]
    Sunuwar,
    #[scr(short = "Sylo", code = 316)]
    SylotiNagri,
    #[scr(short = "Syrc", code = 135)]
    Syriac,
    #[scr(short = "Tglg", code = 370)]
    Tagalog,
    #[scr(short = "Tagb", code = 373)]
    Tagbanwa,
    #[scr(short = "Tale", code = 353)]
    TaiLe,
    #[scr(short = "Lana", code = 351)]
    TaiTham,
    #[scr(short = "Tavt", code = 359)]
    TaiViet,
    #[scr(short = "Tayo", code = 380)]
    TaiYo,
    #[scr(short = "Takr", code = 321)]
    Takri,
    #[scr(short = "Taml", code = 346)]
    Tamil,
    #[scr(short = "Tnsa", code = 275)]
    Tangsa,
    #[scr(short = "Tang", code = 520)]
    Tangut,
    #[scr(short = "Telu", code = 340)]
    Telugu,
    #[scr(short = "Thaa", code = 170)]
    Thaana,
    #[scr(short = "Thai", code = 352)]
    Thai,
    #[scr(short = "Tibt", code = 330)]
    Tibetan,
    #[scr(short = "Tfng", code = 120)]
    Tifinagh,
    #[scr(short = "Tirh", code = 326)]
    Tirhuta,
    #[scr(short = "Todr", code = 229)]
    Todhri,
    #[scr(short = "Tols", code = 299)]
    TolongSiki,
    #[scr(short = "Toto", code = 294)]
    Toto,
    #[scr(short = "Tutg", code = 341)]
    TuluTigalari,
    #[scr(short = "Ugar", code = 40)]
    Ugaritic,
    #[scr(short = "Vaii", code = 470)]
    Vai,
    #[scr(short = "Vith", code = 228)]
    Vithkuqi,
    #[scr(short = "Wcho", code = 283)]
    Wancho,
    #[scr(short = "Wara", code = 262)]
    WarangCiti,
    #[scr(short = "Yezi", code = 192)]
    Yezidi,
    #[scr(short = "Yiii", code = 460)]
    Yi,
    #[scr(short = "Zanb", code = 339)]
    ZanabazarSquare,
}

impl_try_from!(UcdScript, u16, u16 i16 u32 i32 usize isize u64 i64 u128 i128);
impl_serde!(UcdScript, "UcdScript");

use super::BY_NAME;

const fn char_ranges_count() -> usize {
    let mut i = 0;
    let mut cnt = 0;
    while i < BY_NAME.len() {
        cnt += BY_NAME[i].1.len();
        i += 1;
    }

    cnt
}
const LEN: usize = char_ranges_count();

#[derive(Clone, Copy, Debug)]
struct RangeScript {
    range_start: char,
    range_end: char,
    script: UcdScript,
}
const RANGE_SCRIPT_DEFAULT: RangeScript = RangeScript {
    range_start: char::MAX,
    range_end: char::MAX,
    script: unsafe { ::core::mem::transmute::<u8, UcdScript>(0) },
};

/* #[const_trait]
trait ConstDefault {
    fn default<const RUNTIME: bool>() -> Self;
}
impl const ConstDefault for RangeScript {
    fn default() -> Self {
        RangeScript {
            range_start: char::MAX,
            range_end: char::MAX,
            script: Script::Latin,
        }
    }
} */

const fn char_ranges_array_sorted() -> [RangeScript; LEN] {
    let mut res: [RangeScript; LEN] = [RANGE_SCRIPT_DEFAULT; LEN];

    // foreach BY_NAME
    let mut i = 0;
    while i < BY_NAME.len() {
        let (script, ranges) = BY_NAME[i];
        // foreach charset
        let mut j = 0;
        while j < ranges.len() {
            let range = ranges[j];
            // looking for insertion
            let mut ins = 0;
            while ins < LEN {
                let mut prev = res[ins];
                if range.0 < prev.range_start {
                    res[ins] = RangeScript {
                        range_start: range.0,
                        range_end: range.1,
                        script,
                    };
                    if prev.range_start == char::MAX {
                        break;
                    }
                    // shifts all elements right
                    let mut next_ins = ins + 1;
                    while next_ins < LEN {
                        let next = res[next_ins];
                        res[next_ins] = prev;
                        if next.range_start == char::MAX {
                            break;
                        }
                        prev = next;
                        next_ins += 1;
                    }
                    break;
                }
                ins += 1;
            }
            j += 1;
        }
        i += 1;
    }

    res
}
const CHAR_RANGES_SORTED: [RangeScript; LEN] = char_ranges_array_sorted();

/* #[test]
fn print_char_ranges_sorted() {
    panic!("{:?}", CHAR_RANGES_SORTED);
} */

trait BSearch<T> {
    fn binary_search_by_char(&self, ch: char) -> UcdScript;
}
// better than native `binary_search_by` because of `match` with an early exit,
// which gives better optimization with branches instead of `cmovae`.
impl BSearch<RangeScript> for [RangeScript] {
    #[inline(always)]
    fn binary_search_by_char(&self, ch: char) -> UcdScript {
        let mut size = self.len();
        let mut base = 0_usize;

        while size > 1 {
            let half = size >> 1;
            let mid = base + half;

            let res = self.get_safe_unchecked(mid);
            let cmp = compare(res, ch);

            base = match cmp {
                Ordering::Less => mid,
                Ordering::Equal => return res.script,
                Ordering::Greater => base,
            };
            // base = ::core::hint::select_unpredictable(cmp == Ordering::Greater, base, mid);
            size -= half;
        }

        let res = self.get_safe_unchecked(base);
        let cmp = compare(res, ch);
        if cmp == Ordering::Equal {
            res.script
        } else {
            // Some unused `Common` ranges in `ucd` are commented out, so it defaults to `Common`.
            UcdScript::Common
        }
    }
}

#[inline(always)]
fn compare(ra: &RangeScript, ch: char) -> Ordering {
    if ch < ra.range_start {
        Ordering::Greater
    } else if ch > ra.range_end {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

impl UcdScript {
    pub fn find(ch: char) -> Self {
        CHAR_RANGES_SORTED.binary_search_by_char(ch)
        /* CHAR_RANGES_SORTED
        .binary_search_by(|ra| compare(ra, ch))
        .ok()
        .map(|i| CHAR_RANGES_SORTED.get_safe_unchecked(i).script)
        // Some unused `Common` ranges in `ucd` are commented out, so it defaults to `Common`.
        .unwrap_or(Self::Common) */
    }
}

/* pub(crate) fn find_script(ch: char) -> Option<Script> {
    crate::script::BY_SCRIPT
        .iter()
        .find(|(_, chars)| chars.iter().any(|cs| ch > cs.0 || ch <= cs.1))
        .map(|v| v.0)
}

pub(crate) fn script_same(script: Script, ch: char) -> bool {
    crate::script::BY_SCRIPT
        .iter()
        .find(|(a, _)| a == &script)
        .map(|(_, chars)| chars.iter().any(|cs| ch > cs.0 || ch <= cs.1))
        .unwrap_or_default()
} */
/* const fn insertion_sort<const N: usize>(mut arr: [u32; N]) -> [u32; N] {
    let mut i = 1;
    while i < N {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }

    arr
} */

#[cfg(test)]
mod tests {
    use super::{BSearch, RangeScript, UcdScript, CHAR_RANGES_SORTED};

    #[test]
    fn test_char_ranges_sorted() {
        let mut prev = *CHAR_RANGES_SORTED.first().unwrap();
        for range in CHAR_RANGES_SORTED.into_iter().skip(1) {
            assert_ne!(range.range_start, char::MAX, "range_start is default");
            assert_ne!(range.range_end, char::MAX, "range_end is default");
            assert!(
                range.range_start <= range.range_end,
                "range_start > range_end\n{range:?}"
            );
            assert!(
                prev.range_end < range.range_start,
                "prev.range_end >= range.range_start\nprev: {prev:?}\nrange: {range:?}"
            );
            prev = range;
        }
    }

    #[test]
    fn test_bsearch() {
        let input = [RangeScript {
            range_start: 'a',
            range_end: 'z',
            script: UcdScript::Latin,
        }];
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Latin);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
            RangeScript {
                range_start: 'g',
                range_end: 'h',
                script: UcdScript::Arabic,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('c'), UcdScript::Ahom);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(
            input.binary_search_by_char('f'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('g'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('h'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        let input = [
            RangeScript {
                range_start: 'a',
                range_end: 'b',
                script: UcdScript::Adlam,
            },
            RangeScript {
                range_start: 'c',
                range_end: 'd',
                script: UcdScript::Ahom,
            },
            RangeScript {
                range_start: 'e',
                range_end: 'f',
                script: UcdScript::AnatolianHieroglyphs,
            },
            RangeScript {
                range_start: 'g',
                range_end: 'h',
                script: UcdScript::Arabic,
            },
            RangeScript {
                range_start: 'i',
                range_end: 'j',
                script: UcdScript::Armenian,
            },
        ];
        assert_eq!(input.binary_search_by_char('a'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('b'), UcdScript::Adlam);
        assert_eq!(input.binary_search_by_char('c'), UcdScript::Ahom);
        assert_eq!(input.binary_search_by_char('d'), UcdScript::Ahom);
        assert_eq!(
            input.binary_search_by_char('e'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(
            input.binary_search_by_char('f'),
            UcdScript::AnatolianHieroglyphs
        );
        assert_eq!(input.binary_search_by_char('g'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('h'), UcdScript::Arabic);
        assert_eq!(input.binary_search_by_char('i'), UcdScript::Armenian);
        assert_eq!(input.binary_search_by_char('j'), UcdScript::Armenian);
        assert_eq!(input.binary_search_by_char('x'), UcdScript::Common);

        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char('Ύ'),
            UcdScript::Greek
        );
        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char('Σ'),
            UcdScript::Greek
        );
        assert_eq!(
            CHAR_RANGES_SORTED.binary_search_by_char(char::MAX),
            UcdScript::Common
        );
    }
}
