use ::core::cmp::Ordering;
use debug_unsafe::slice::SliceGetter;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
#[strum(const_into_str)]
// #[strum(ascii_case_insensitive)]
pub enum Script {
    #[strum(serialize = "Adlm")]
    Adlam,
    Ahom,
    AnatolianHieroglyphs,
    #[strum(serialize = "Arab")]
    Arabic,
    #[strum(serialize = "Armn")]
    Armenian,
    Avestan,
    Balinese,
    Bamum,
    BassaVah,
    #[strum(serialize = "Batk")]
    Batak,
    #[strum(serialize = "Beng")]
    Bengali,
    Bhaiksuki,
    Bopomofo,
    Brahmi,
    Braille,
    #[strum(serialize = "Bugi")]
    Buginese,
    Buhid,
    CanadianAboriginal,
    Carian,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    Chorasmian,
    Common,
    Coptic,
    #[strum(serialize = "Xsux")]
    Cuneiform,
    Cypriot,
    #[strum(serialize = "Cpmn")]
    CyproMinoan,
    #[strum(serialize = "Cyrl")]
    Cyrillic,
    #[strum(serialize = "Dsrt")]
    Deseret,
    #[strum(serialize = "Deva")]
    Devanagari,
    DivesAkuru,
    Dogra,
    #[strum(serialize = "Dupl")]
    Duployan,
    #[strum(serialize = "Egyp")]
    EgyptianHieroglyphs,
    Elbasan,
    #[strum(serialize = "Elym")]
    Elymaic,
    #[strum(serialize = "Ethi")]
    Ethiopic,
    Garay,
    #[strum(serialize = "Geor")]
    Georgian,
    Glagolitic,
    #[strum(serialize = "Goth")]
    Gothic,
    Grantha,
    #[strum(serialize = "Grek")]
    Greek,
    #[strum(serialize = "Gujr")]
    Gujarati,
    GunjalaGondi,
    #[strum(serialize = "Guru")]
    Gurmukhi,
    #[strum(serialize = "Gukh")]
    GurungKhema,
    #[strum(serialize = "Hani")]
    Han,
    #[strum(serialize = "Hang")]
    Hangul,
    #[strum(serialize = "Rohg")]
    HanifiRohingya,
    Hanunoo,
    Hatran,
    #[strum(serialize = "Hebr")]
    Hebrew,
    Hiragana,
    ImperialAramaic,
    Inherited,
    InscriptionalPahlavi,
    InscriptionalParthian,
    Javanese,
    Kaithi,
    #[strum(serialize = "Knda")]
    Kannada,
    Katakana,
    Kawi,
    #[strum(serialize = "Kali")]
    KayahLi,
    #[strum(serialize = "Khar")]
    Kharoshthi,
    #[strum(serialize = "Kits")]
    KhitanSmallScript,
    #[strum(serialize = "Khmr")]
    Khmer,
    Khojki,
    Khudawadi,
    KiratRai,
    #[strum(serialize = "Laoo")]
    Lao,
    #[strum(serialize = "Latn")]
    Latin,
    #[strum(serialize = "Lepc")]
    Lepcha,
    Limbu,
    #[strum(serialize = "Lina")]
    LinearA,
    #[strum(serialize = "Linb")]
    LinearB,
    #[strum(serialize = "Lisu")]
    Lisu,
    #[strum(serialize = "Lyci")]
    Lycian,
    #[strum(serialize = "Lydi")]
    Lydian,
    Mahajani,
    #[strum(serialize = "Maka")]
    Makasar,
    #[strum(serialize = "Mlym")]
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    MasaramGondi,
    Medefaidrin,
    #[strum(serialize = "Mtei")]
    MeeteiMayek,
    #[strum(serialize = "Mend")]
    MendeKikakui,
    #[strum(serialize = "Merc")]
    MeroiticCursive,
    #[strum(serialize = "Mero")]
    MeroiticHieroglyphs,
    Miao,
    Modi,
    Mongolian,
    #[strum(serialize = "Mroo")]
    Mro,
    #[strum(serialize = "Mult")]
    Multani,
    #[strum(serialize = "Mymr")]
    Myanmar,
    Nabataean,
    #[strum(serialize = "Nagm")]
    NagMundari,
    Nandinagari,
    #[strum(serialize = "Newa")]
    Newa,
    NewTaiLue,
    #[strum(serialize = "Nkoo")]
    Nko,
    Nushu,
    NyiakengPuachueHmong,
    Ogham,
    #[strum(serialize = "Olck")]
    OlChiki,
    #[strum(serialize = "Hung")]
    OldHungarian,
    #[strum(serialize = "Ital")]
    OldItalic,
    #[strum(serialize = "Narb")]
    OldNorthArabian,
    #[strum(serialize = "Perm")]
    OldPermic,
    #[strum(serialize = "Xpeo")]
    OldPersian,
    #[strum(serialize = "Sogo")]
    OldSogdian,
    #[strum(serialize = "Sarb")]
    OldSouthArabian,
    #[strum(serialize = "Orkh")]
    OldTurkic,
    #[strum(serialize = "Ougr")]
    OldUyghur,
    #[strum(serialize = "Onao")]
    OlOnal,
    #[strum(serialize = "Orya")]
    Oriya,
    #[strum(serialize = "Osge")]
    Osage,
    Osmanya,
    PahawhHmong,
    Palmyrene,
    #[strum(serialize = "Pauc")]
    PauCinHau,
    PhagsPa,
    #[strum(serialize = "Phnx")]
    Phoenician,
    PsalterPahlavi,
    #[strum(serialize = "Rjng")]
    Rejang,
    Runic,
    Samaritan,
    #[strum(serialize = "Saur")]
    Saurashtra,
    Sharada,
    #[strum(serialize = "Shaw")]
    Shavian,
    Siddham,
    #[strum(serialize = "Sgnw")]
    SignWriting,
    #[strum(serialize = "Sinh")]
    Sinhala,
    #[strum(serialize = "Sogd")]
    Sogdian,
    #[strum(serialize = "Sora")]
    SoraSompeng,
    Soyombo,
    Sundanese,
    #[strum(serialize = "Sunu")]
    Sunuwar,
    #[strum(serialize = "Sylo")]
    SylotiNagri,
    Syriac,
    Tagalog,
    Tagbanwa,
    #[strum(serialize = "Tale")]
    TaiLe,
    TaiTham,
    TaiViet,
    Takri,
    #[strum(serialize = "Taml")]
    Tamil,
    Tangsa,
    #[strum(serialize = "Tang")]
    Tangut,
    #[strum(serialize = "Telu")]
    Telugu,
    Thaana,
    #[strum(serialize = "Thai")]
    Thai,
    #[strum(serialize = "Tibt")]
    Tibetan,
    #[strum(serialize = "Tfng")]
    Tifinagh,
    Tirhuta,
    Todhri,
    #[strum(serialize = "Toto")]
    Toto,
    #[strum(serialize = "Tutg")]
    TuluTigalari,
    #[strum(serialize = "Ugar")]
    Ugaritic,
    #[strum(serialize = "Vaii")]
    Vai,
    Vithkuqi,
    #[strum(serialize = "Wcho")]
    Wancho,
    WarangCiti,
    #[strum(serialize = "Yezi")]
    Yezidi,
    #[strum(serialize = "Yiii")]
    Yi,
    ZanabazarSquare,
}

use super::ucd::BY_NAME;

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
    script: Script,
}
const RANGE_SCRIPT_DEFAULT: RangeScript = RangeScript {
    range_start: char::MAX,
    range_end: char::MAX,
    script: unsafe { ::core::mem::transmute::<u8, Script>(0) },
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

impl Script {
    pub fn find(ch: char) -> Self {
        CHAR_RANGES_SORTED
            .binary_search_by(|ra| compare(ra, ch))
            .ok()
            .map(|i| CHAR_RANGES_SORTED.get_safe_unchecked(i).script)
            // Some unused `Common` ranges in `ucd` are commented out, so it defaults to `Common`
            .unwrap_or(Self::Common)
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
    use super::CHAR_RANGES_SORTED;

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
}
