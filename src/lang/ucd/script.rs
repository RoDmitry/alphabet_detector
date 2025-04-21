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
pub enum UcdScript {
    #[strum(serialize = "Adlm")]
    Adlam,
    #[strum(serialize = "Ahom")]
    Ahom,
    #[strum(serialize = "Hluw")]
    AnatolianHieroglyphs,
    #[strum(serialize = "Arab")]
    Arabic,
    #[strum(serialize = "Armn")]
    Armenian,
    #[strum(serialize = "Avst")]
    Avestan,
    #[strum(serialize = "Bali")]
    Balinese,
    #[strum(serialize = "Bamu")]
    Bamum,
    #[strum(serialize = "Bass")]
    BassaVah,
    #[strum(serialize = "Batk")]
    Batak,
    #[strum(serialize = "Beng")]
    Bengali,
    #[strum(serialize = "Bhks")]
    Bhaiksuki,
    #[strum(serialize = "Bopo")]
    Bopomofo,
    #[strum(serialize = "Brah")]
    Brahmi,
    #[strum(serialize = "Brai")]
    Braille,
    #[strum(serialize = "Bugi")]
    Buginese,
    #[strum(serialize = "Buhd")]
    Buhid,
    #[strum(serialize = "Cans")]
    CanadianAboriginal,
    #[strum(serialize = "Cari")]
    Carian,
    #[strum(serialize = "Aghb")]
    CaucasianAlbanian,
    #[strum(serialize = "Cakm")]
    Chakma,
    #[strum(serialize = "Cham")]
    Cham,
    #[strum(serialize = "Cher")]
    Cherokee,
    #[strum(serialize = "Chrs")]
    Chorasmian,
    #[strum(serialize = "Zyyy")]
    Common,
    #[strum(serialize = "Copt")]
    Coptic,
    #[strum(serialize = "Xsux")]
    Cuneiform,
    #[strum(serialize = "Cprt")]
    Cypriot,
    #[strum(serialize = "Cpmn")]
    CyproMinoan,
    #[strum(serialize = "Cyrl")]
    Cyrillic,
    #[strum(serialize = "Dsrt")]
    Deseret,
    #[strum(serialize = "Deva")]
    Devanagari,
    #[strum(serialize = "Diak")]
    DivesAkuru,
    #[strum(serialize = "Dogr")]
    Dogra,
    #[strum(serialize = "Dupl")]
    Duployan,
    #[strum(serialize = "Egyp")]
    EgyptianHieroglyphs,
    #[strum(serialize = "Elba")]
    Elbasan,
    #[strum(serialize = "Elym")]
    Elymaic,
    #[strum(serialize = "Ethi")]
    Ethiopic,
    #[strum(serialize = "Gara")]
    Garay,
    #[strum(serialize = "Geor")]
    Georgian,
    #[strum(serialize = "Glag")]
    Glagolitic,
    #[strum(serialize = "Goth")]
    Gothic,
    #[strum(serialize = "Gran")]
    Grantha,
    #[strum(serialize = "Grek")]
    Greek,
    #[strum(serialize = "Gujr")]
    Gujarati,
    #[strum(serialize = "Gong")]
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
    #[strum(serialize = "Hano")]
    Hanunoo,
    #[strum(serialize = "Hatr")]
    Hatran,
    #[strum(serialize = "Hebr")]
    Hebrew,
    #[strum(serialize = "Hira")]
    Hiragana,
    #[strum(serialize = "Armi")]
    ImperialAramaic,
    #[strum(serialize = "Zinh")]
    Inherited,
    #[strum(serialize = "Phli")]
    InscriptionalPahlavi,
    #[strum(serialize = "Prti")]
    InscriptionalParthian,
    #[strum(serialize = "Java")]
    Javanese,
    #[strum(serialize = "Kthi")]
    Kaithi,
    #[strum(serialize = "Knda")]
    Kannada,
    #[strum(serialize = "Kana")]
    Katakana,
    #[strum(serialize = "Kawi")]
    Kawi,
    #[strum(serialize = "Kali")]
    KayahLi,
    #[strum(serialize = "Khar")]
    Kharoshthi,
    #[strum(serialize = "Kits")]
    KhitanSmallScript,
    #[strum(serialize = "Khmr")]
    Khmer,
    #[strum(serialize = "Khoj")]
    Khojki,
    #[strum(serialize = "Sind")]
    Khudawadi,
    #[strum(serialize = "Krai")]
    KiratRai,
    #[strum(serialize = "Laoo")]
    Lao,
    #[strum(serialize = "Latn")]
    Latin,
    #[strum(serialize = "Lepc")]
    Lepcha,
    #[strum(serialize = "Limb")]
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
    #[strum(serialize = "Mahj")]
    Mahajani,
    #[strum(serialize = "Maka")]
    Makasar,
    #[strum(serialize = "Mlym")]
    Malayalam,
    #[strum(serialize = "Mand")]
    Mandaic,
    #[strum(serialize = "Mani")]
    Manichaean,
    #[strum(serialize = "Marc")]
    Marchen,
    #[strum(serialize = "Gonm")]
    MasaramGondi,
    #[strum(serialize = "Medf")]
    Medefaidrin,
    #[strum(serialize = "Mtei")]
    MeeteiMayek,
    #[strum(serialize = "Mend")]
    MendeKikakui,
    #[strum(serialize = "Merc")]
    MeroiticCursive,
    #[strum(serialize = "Mero")]
    MeroiticHieroglyphs,
    #[strum(serialize = "Plrd")]
    Miao,
    #[strum(serialize = "Modi")]
    Modi,
    #[strum(serialize = "Mong")]
    Mongolian,
    #[strum(serialize = "Mroo")]
    Mro,
    #[strum(serialize = "Mult")]
    Multani,
    #[strum(serialize = "Mymr")]
    Myanmar,
    #[strum(serialize = "Nbat")]
    Nabataean,
    #[strum(serialize = "Nagm")]
    NagMundari,
    #[strum(serialize = "Nand")]
    Nandinagari,
    #[strum(serialize = "Newa")]
    Newa,
    #[strum(serialize = "Talu")]
    NewTaiLue,
    #[strum(serialize = "Nkoo")]
    Nko,
    #[strum(serialize = "Nshu")]
    Nushu,
    #[strum(serialize = "Hmnp")]
    NyiakengPuachueHmong,
    #[strum(serialize = "Ogam")]
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
    #[strum(serialize = "Osma")]
    Osmanya,
    #[strum(serialize = "Hmng")]
    PahawhHmong,
    #[strum(serialize = "Palm")]
    Palmyrene,
    #[strum(serialize = "Pauc")]
    PauCinHau,
    #[strum(serialize = "Phag")]
    PhagsPa,
    #[strum(serialize = "Phnx")]
    Phoenician,
    #[strum(serialize = "Phlp")]
    PsalterPahlavi,
    #[strum(serialize = "Rjng")]
    Rejang,
    #[strum(serialize = "Runr")]
    Runic,
    #[strum(serialize = "Samr")]
    Samaritan,
    #[strum(serialize = "Saur")]
    Saurashtra,
    #[strum(serialize = "Shrd")]
    Sharada,
    #[strum(serialize = "Shaw")]
    Shavian,
    #[strum(serialize = "Sidd")]
    Siddham,
    #[strum(serialize = "Sgnw")]
    SignWriting,
    #[strum(serialize = "Sinh")]
    Sinhala,
    #[strum(serialize = "Sogd")]
    Sogdian,
    #[strum(serialize = "Sora")]
    SoraSompeng,
    #[strum(serialize = "Soyo")]
    Soyombo,
    #[strum(serialize = "Sund")]
    Sundanese,
    #[strum(serialize = "Sunu")]
    Sunuwar,
    #[strum(serialize = "Sylo")]
    SylotiNagri,
    #[strum(serialize = "Syrc")]
    Syriac,
    #[strum(serialize = "Tglg")]
    Tagalog,
    #[strum(serialize = "Tagb")]
    Tagbanwa,
    #[strum(serialize = "Tale")]
    TaiLe,
    #[strum(serialize = "Lana")]
    TaiTham,
    #[strum(serialize = "Tavt")]
    TaiViet,
    #[strum(serialize = "Takr")]
    Takri,
    #[strum(serialize = "Taml")]
    Tamil,
    #[strum(serialize = "Tnsa")]
    Tangsa,
    #[strum(serialize = "Tang")]
    Tangut,
    #[strum(serialize = "Telu")]
    Telugu,
    #[strum(serialize = "Thaa")]
    Thaana,
    #[strum(serialize = "Thai")]
    Thai,
    #[strum(serialize = "Tibt")]
    Tibetan,
    #[strum(serialize = "Tfng")]
    Tifinagh,
    #[strum(serialize = "Tirh")]
    Tirhuta,
    #[strum(serialize = "Todr")]
    Todhri,
    #[strum(serialize = "Toto")]
    Toto,
    #[strum(serialize = "Tutg")]
    TuluTigalari,
    #[strum(serialize = "Ugar")]
    Ugaritic,
    #[strum(serialize = "Vaii")]
    Vai,
    #[strum(serialize = "Vith")]
    Vithkuqi,
    #[strum(serialize = "Wcho")]
    Wancho,
    #[strum(serialize = "Wara")]
    WarangCiti,
    #[strum(serialize = "Yezi")]
    Yezidi,
    #[strum(serialize = "Yiii")]
    Yi,
    #[strum(serialize = "Zanb")]
    ZanabazarSquare,
}

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
