use super::{script_char_to_langs, Script};
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use ::std::fmt::{Debug, Display, Formatter, Result};
use ahash::AHashSet;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    EnumCountMacro,
    EnumIter,
    EnumString,
    // strum_macros::Display,
)]
// todo: remove this UPPERCASE?
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
// #[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, rename_all = "UPPERCASE")
)]
#[non_exhaustive]
#[repr(usize)]
pub enum Language {
    #[strum(serialize = "ace_Latn")]
    Acehnese, // Latin
    #[strum(serialize = "ace_Arab")]
    AcehneseJawi,
    #[strum(serialize = "afr")]
    Afrikaans,
    Ahom,
    Akkadian,
    AlbanianHistorical,
    #[strum(serialize = "als")]
    AlbanianTosk, // Latin
    AlbanianToskVithkuqi,
    #[strum(serialize = "amh")]
    Amharic,
    AncientGreek,
    Angkola,
    #[strum(serialize = "arb")]
    Arabic,
    #[strum(serialize = "arz")]
    ArabicEgyptian,
    #[strum(serialize = "acm")]
    ArabicMesopotamian,
    #[strum(serialize = "ary")]
    ArabicMoroccan,
    #[strum(serialize = "ars")]
    ArabicNajdi,
    #[strum(serialize = "apc")]
    ArabicNorthLevantine,
    #[strum(serialize = "acq")]
    ArabicSouthernYemeni,
    #[strum(serialize = "ajp")]
    ArabicSouthLevantine,
    #[strum(serialize = "aeb")]
    ArabicTunisian,
    AramaicHatran,
    AramaicImperial,
    AramaicMandaic,
    AramaicNabataean,
    AramaicPalmyrene,
    AramaicSyriac,
    #[strum(serialize = "hye")]
    Armenian,
    #[strum(serialize = "asm")]
    Assamese,
    #[strum(serialize = "ast")]
    Asturian,
    Avestan,
    #[strum(serialize = "awa")]
    Awadhi,
    #[strum(serialize = "ayr")]
    AymaraCentral,
    #[strum(serialize = "azj")]
    AzerbaijaniNorth,
    #[strum(serialize = "azb")]
    AzerbaijaniSouth,
    #[strum(serialize = "ban")]
    Balinese, // Latin
    BalineseBalinese,
    #[strum(serialize = "bam")]
    Bambara,
    Bamum,
    #[strum(serialize = "bjn_Latn")]
    Banjar, // Latin
    #[strum(serialize = "bjn_Arab")]
    BanjarJawi,
    Bantawa,
    #[strum(serialize = "bak")]
    Bashkir,
    #[strum(serialize = "eus")]
    Basque,
    Bassa,
    #[strum(serialize = "bel")]
    Belarusian,
    #[strum(serialize = "bem")]
    Bemba,
    #[strum(serialize = "ben")]
    Bengali,
    Berber,
    Bhaiksuki,
    #[strum(serialize = "bho")]
    Bhojpuri, // Devanagari
    BhojpuriKaithi,
    BishnupriyaManipuri,
    #[deprecated]
    Bokmal, // todo: rm, renamed to NorwegianBokmal
    #[strum(serialize = "bos")]
    Bosnian,
    Braille, // Any language adapted to Braille
    BuddhistMarchen,
    #[strum(serialize = "bug")]
    Buginese, // Latin
    BugineseBuginese,
    Buhid,
    #[strum(serialize = "bul")]
    Bulgarian,
    #[strum(serialize = "mya")]
    Burmese,
    Carian,
    #[strum(serialize = "cat")]
    Catalan,
    CaucasianAlbanian,
    #[strum(serialize = "ceb")]
    Cebuano,
    Chakma,
    Cham,
    Cherokee,
    #[strum(serialize = "hne")]
    Chhattisgarhi,
    #[strum(serialize = "zho_Hans")]
    ChineseSimplified,
    #[strum(serialize = "zho_Hant")]
    ChineseTraditional,
    #[strum(serialize = "yue_Hant")]
    ChineseCantoneseTraditional,
    ChineseMandarinBopomofo,
    ChineseTuhua,
    #[strum(serialize = "cjk")]
    Chokwe,
    Chorasmian,
    Coptic,
    Cree,
    #[strum(serialize = "hat")]
    CreoleHaitian,
    #[strum(serialize = "hrv")]
    Croatian,
    CyproMinoan, // used in ancient Cyprus
    #[strum(serialize = "ces")]
    Czech,
    #[strum(serialize = "dan")]
    Danish,
    #[strum(serialize = "dik")]
    DinkaSouthwestern,
    Dogri,
    DogriDevanagari,
    DogriPersoArabic,
    DogriTakri,
    #[strum(serialize = "nld")]
    Dutch,
    #[strum(serialize = "dyu")]
    Dyula,
    #[strum(serialize = "dzo")]
    Dzongkha,
    EgyptianHieroglyphs,
    Elymaic,
    #[strum(serialize = "eng")]
    English,
    EnglishDuployan, //Shorthand systems for English
    EnglishMormon,
    EnglishPhonetic,
    #[strum(serialize = "epo")]
    Esperanto,
    #[strum(serialize = "est")]
    Estonian,
    Etruscan,
    #[strum(serialize = "ewe")]
    Ewe,
    #[strum(serialize = "fao")]
    Faroese,
    #[strum(serialize = "fij")]
    Fijian,
    #[strum(serialize = "fin")]
    Finnish,
    #[strum(serialize = "fon")]
    Fon,
    #[strum(serialize = "fra")]
    French,
    FrenchDuployan,
    #[strum(serialize = "fur")]
    Friulian,
    Fulani,
    #[strum(serialize = "fuv")]
    FulfuldeNigerian,
    #[strum(serialize = "gla")]
    GaelicScottish,
    #[strum(serialize = "glg")]
    Galician,
    #[strum(serialize = "lug")]
    Ganda,
    Gandhari,
    Geez,
    #[strum(serialize = "kat")]
    Georgian,
    #[strum(serialize = "deu")]
    German,
    GondiGunjala,
    GondiMasaram,
    Gothic,
    #[strum(serialize = "ell")]
    Greek,
    #[strum(serialize = "grn")]
    Guarani,
    #[strum(serialize = "guj")]
    Gujarati,
    Gurung,
    Hanunoo,
    #[strum(serialize = "hau")]
    Hausa,
    Hawaiian,
    #[strum(serialize = "heb")]
    Hebrew,
    HebrewSamaritan,
    #[strum(serialize = "hin")]
    Hindi, // Devanagari
    HindiKaithi,
    HindiMahajani,
    Hittite,
    Hmong,
    Ho,
    #[strum(serialize = "hun")]
    Hungarian,
    #[strum(serialize = "isl")]
    Icelandic,
    #[strum(serialize = "ibo")]
    Igbo,
    #[strum(serialize = "ilo")]
    Ilocano,
    #[strum(serialize = "ind")]
    Indonesian,
    Inuktitut,
    #[strum(serialize = "gle")]
    Irish,
    #[strum(serialize = "ita")]
    Italian,
    #[strum(serialize = "jpn")]
    Japanese,
    #[strum(serialize = "jav")]
    Javanese, // Latin
    JavaneseJavanese,
    #[strum(serialize = "kac")]
    Jingpho,
    #[strum(serialize = "kbp")]
    Kabiye,
    #[strum(serialize = "kea")]
    Kabuverdianu,
    #[strum(serialize = "kab")]
    Kabyle,
    #[strum(serialize = "kam")]
    Kamba,
    #[strum(serialize = "kan")]
    Kannada,
    KannadaTuluTigalari,
    #[strum(serialize = "knc_Latn")]
    KanuriCentral,
    #[strum(serialize = "knc_Arab")]
    KanuriCentralAjami,
    Karo,
    #[strum(serialize = "kas_Arab")]
    Kashmiri, // Arabic
    #[strum(serialize = "kas_Deva")]
    KashmiriDevanagari,
    KashmiriSharada,
    KashmiriTakri,
    KayahLi,
    #[strum(serialize = "kaz")]
    Kazakh,
    Khitan,
    #[strum(serialize = "khm")]
    Khmer,
    Khoja,
    #[strum(serialize = "kon")]
    Kikongo,
    #[strum(serialize = "kik")]
    Kikuyu,
    #[strum(serialize = "kmb")]
    Kimbundu,
    #[strum(serialize = "kin")]
    Kinyarwanda,
    #[strum(serialize = "kor")]
    Korean,
    #[strum(serialize = "ckb")]
    KurdishCentral, // Arabic
    #[strum(serialize = "kmr")]
    KurdishNorthern, // Latin
    KurdishSouthern, // Arabic
    KurdishYezidi,
    #[strum(serialize = "kir")]
    Kyrgyz,
    #[strum(serialize = "lao")]
    Lao,
    LaoTaiTham,
    #[strum(serialize = "ltg")]
    Latgalian,
    Latin,
    #[strum(serialize = "lvs")]
    Latvian,
    Lepcha,
    #[strum(serialize = "lij")]
    Ligurian,
    Limbu,
    #[strum(serialize = "lim")]
    Limburgish,
    #[strum(serialize = "lin")]
    Lingala,
    Lisu,
    #[strum(serialize = "lit")]
    Lithuanian,
    #[strum(serialize = "lmo")]
    Lombard,
    #[strum(serialize = "lua")]
    LubaKasai,
    #[strum(serialize = "luo")]
    Luo,
    Luwian,
    #[strum(serialize = "ltz")]
    Luxembourgish,
    Lycian,
    Lydian,
    #[strum(serialize = "mkd")]
    Macedonian,
    #[strum(serialize = "mag")]
    Magahi, // Devanagari
    MagahiKaithi,
    #[strum(serialize = "mai")]
    Maithili, // Devanagari
    MaithiliKaithi,
    MaithiliTirhuta,
    Makasar,
    Makassarese,
    #[strum(serialize = "zsm")]
    Malay,
    #[strum(serialize = "mal")]
    Malayalam,
    MaldivianDhivehi,
    #[strum(serialize = "plt")]
    MalgasyPlateau,
    #[strum(serialize = "mlt")]
    Maltese,
    Mandaic,
    Mandailing,
    Mande,
    ManipuriMeetei,
    #[strum(serialize = "mri")]
    Maori,
    #[strum(serialize = "mar")]
    Marathi, // Devanagari
    MarathiBrahmi,
    MarathiModi,
    Marwari,
    Medefaidrin,
    #[strum(serialize = "mni")]
    Meitei,
    Mende,
    Meroitic,
    MiddlePersianInscriptionalPahlavi,
    MiddlePersianManichaean,
    MiddlePersianPsalterPahlavi,
    #[strum(serialize = "min")]
    Minangkabau,
    Minoan,
    #[strum(serialize = "lus")]
    Mizo,
    #[strum(serialize = "khk")]
    MongolianHalh, // Cyrillic
    MongolianHalhMongolian,
    MongolianHalhPhagsPa,
    MongolianHalhSoyombo,
    MongolianHalhZanabazarSquare,
    #[strum(serialize = "mos")]
    Mossi,
    Mro,
    Mundari,
    MycenaeanGreek,
    #[strum(serialize = "npi")]
    Nepali,
    Newari,
    NorthernThai,
    #[strum(serialize = "nob")]
    NorwegianBokmal,
    #[strum(serialize = "nno")]
    NorwegianNynorsk,
    #[strum(serialize = "nus")]
    Nuer,
    #[strum(serialize = "nya")]
    Nyanja,
    #[deprecated]
    Nynorsk, // todo: rm, renamed to NorwegianNynorsk
    #[strum(serialize = "oci")]
    Occitan,
    #[strum(serialize = "ory")]
    Odia,
    Ojibwe,
    OldChurchSlavonic,
    OldChurchSlavonicGlagolitic,
    OldEnglish,
    OldHungarian,
    OldIrish,
    OldJavanese,
    OldKomi,
    OldNorse,
    OldNorthArabian,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    Oromo,
    #[strum(serialize = "gaz")]
    OromoWestCentral,
    Osage,
    Oscan,
    Pakpak,
    #[strum(serialize = "pag")]
    Pangasinan,
    #[strum(serialize = "pap")]
    Papiamento,
    Parthian,
    Pashto,
    #[strum(serialize = "pbt")]
    PashtoSouthern,
    #[deprecated]
    Persian, // todo: rm
    #[strum(serialize = "prs")]
    PersianDari, // Afghani
    #[strum(serialize = "pes")]
    PersianWestern, // Iranian
    Phoenician,
    #[strum(serialize = "pol")]
    Polish,
    #[strum(serialize = "por")]
    Portuguese,
    Prakrit,
    Pular,
    #[strum(serialize = "pan")]
    PunjabiEastern, // Gurmukhi
    PunjabiEasternMahajani,
    PunjabiEasternShahmukhi,
    #[strum(serialize = "quy")]
    QuechuaAyacucho,
    Rejang,
    Rohingya,
    #[strum(serialize = "ron")]
    Romanian,
    #[strum(serialize = "run")]
    Rundi,
    #[strum(serialize = "rus")]
    Russian,
    #[strum(serialize = "smo")]
    Samoan,
    #[strum(serialize = "sag")]
    Sango,
    #[strum(serialize = "san")]
    Sanskrit, // Devanagari
    SanskritBrahmi,
    SanskritGrantha,
    SanskritKawi,
    SanskritNandinagari,
    SanskritSharada,
    SanskritSiddham,
    SanskritSoyombo,
    SanskritTuluTigalari,
    SanskritZanabazarSquare,
    #[strum(serialize = "sat")]
    Santali,
    Saraiki,
    #[strum(serialize = "srd")]
    Sardinian,
    Saurashtra,
    #[strum(serialize = "nso")]
    Sepedi,
    #[strum(serialize = "srp")]
    Serbian,
    #[strum(serialize = "sot")]
    Sesotho,
    #[strum(serialize = "shn")]
    Shan,
    #[strum(serialize = "sna")]
    Shona,
    #[strum(serialize = "scn")]
    Sicilian,
    Signlanguages,
    #[strum(serialize = "szl")]
    Silesian,
    Simalungun,
    #[strum(serialize = "snd")]
    Sindhi, // Arabic
    SindhiDevanagari,
    SindhiKhojki,
    SindhiKhudawadi,
    #[strum(serialize = "sin")]
    Sinhala,
    #[strum(serialize = "slk")]
    Slovak,
    #[strum(serialize = "slv")]
    Slovene,
    Sogdian,
    SogdianManichaean,
    #[strum(serialize = "som")]
    Somali, // Latin
    SomaliOsmanya,
    Sora,
    #[strum(serialize = "spa")]
    Spanish,
    Sumerian,
    #[strum(serialize = "sun")]
    Sundanese, // Latin
    SundaneseSundanese,
    Sunuwar,
    #[strum(serialize = "swh")]
    Swahili,
    #[strum(serialize = "ssw")]
    Swati,
    #[strum(serialize = "swe")]
    Swedish,
    Sylheti,
    Syriac,
    #[strum(serialize = "tgl")]
    Tagalog, // Latin
    TagalogTagalog,
    Tagbanwa,
    TaiDam,
    TaiDon,
    TaiLe,
    TaiLue,
    #[strum(serialize = "tgk")]
    Tajik,
    #[strum(serialize = "taq_Latn")]
    TamasheqLatin,
    #[strum(serialize = "taq_Tfng")]
    TamasheqTifinagh,
    #[strum(serialize = "tzm")]
    TamazightCentralAtlas,
    #[strum(serialize = "tam")]
    Tamil, // Tamil
    TamilGrantha,
    Tangsa,
    Tangut,
    #[strum(serialize = "tat")]
    Tatar,
    #[strum(serialize = "crh")]
    TatarCrimean,
    #[strum(serialize = "tel")]
    Telugu,
    #[strum(serialize = "tha")]
    Thai,
    #[strum(serialize = "bod")]
    Tibetan,
    TibetanPhagsPa,
    TibetanSoyombo,
    TibetanZanabazarSquare,
    #[strum(serialize = "tir")]
    Tigrinya,
    Toba,
    #[strum(serialize = "tpi")]
    TokPisin,
    Toto,
    #[strum(serialize = "tso")]
    Tsonga,
    #[strum(serialize = "tsn")]
    Tswana,
    Tulu,
    #[strum(serialize = "tum")]
    Tumbuka,
    #[strum(serialize = "tur")]
    Turkish,
    #[strum(serialize = "tuk")]
    Turkmen,
    #[strum(serialize = "twi")]
    Twi,
    Ugaritic,
    #[strum(serialize = "ukr")]
    Ukrainian,
    Umbrian,
    #[strum(serialize = "umb")]
    Umbundu,
    #[strum(serialize = "urd")]
    Urdu,
    #[strum(serialize = "uig")]
    Uyghur,
    #[strum(serialize = "uzn")]
    UzbekNorthern,
    Vai,
    #[strum(serialize = "vec")]
    Venetian,
    #[strum(serialize = "vie")]
    Vietnamese,
    Wancho,
    #[strum(serialize = "war")]
    Waray,
    #[strum(serialize = "cym")]
    Welsh,
    #[strum(serialize = "wol")]
    Wolof, // Latin
    WolofGaray,
    #[strum(serialize = "xho")]
    Xhosa,
    Yi,
    Yiddish,
    #[strum(serialize = "ydd")]
    YiddishEastern,
    #[strum(serialize = "yor")]
    Yoruba,
    ZoLanguages,
    #[strum(serialize = "zul")]
    Zulu,
}

// const LANGUAGE_COUNT: usize = ::core::mem::variant_count::<Language>();
pub type LanguageArr<T> = [T; Language::COUNT];
#[inline(always)]
pub fn lang_arr_default<T: Default + Copy>() -> LanguageArr<T> {
    [Default::default(); Language::COUNT]
}

impl From<usize> for Language {
    #[inline(always)]
    fn from(v: usize) -> Self {
        unsafe { ::core::mem::transmute(v) }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
}

impl Language {
    /// Returns an iterator of all languages
    #[inline]
    pub fn all() -> impl Iterator<Item = Language> {
        Language::iter()
    }

    /// Returns all languages supporting selected `Script`
    #[inline]
    pub fn all_with_script(script: Script) -> &'static [Language] {
        script_char_to_langs(script, char::default())
    }

    /// Returns a set of all supported spoken languages.
    #[deprecated]
    pub fn all_spoken_ones() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| it != &Language::Latin)
            .collect()
    }

    /// Returns the language associated with the ISO 639-1 code
    /// passed to this method.
    #[deprecated]
    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    /// Returns the language associated with the ISO 639-3 code
    /// passed to this method.
    #[deprecated]
    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    /// Returns the ISO 639-1 code of this language.
    #[deprecated]
    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        use Language::*;
        match self {
            Afrikaans => IsoCode639_1::AF,
            AlbanianTosk => IsoCode639_1::SQ, // invalid
            Arabic => IsoCode639_1::AR,
            Armenian => IsoCode639_1::HY,
            AzerbaijaniNorth => IsoCode639_1::AZ,
            Basque => IsoCode639_1::EU,
            Belarusian => IsoCode639_1::BE,
            Bengali => IsoCode639_1::BN,
            Bokmal => IsoCode639_1::NB,
            Bosnian => IsoCode639_1::BS,
            Bulgarian => IsoCode639_1::BG,
            Catalan => IsoCode639_1::CA,
            ChineseSimplified => IsoCode639_1::ZH,
            Croatian => IsoCode639_1::HR,
            Czech => IsoCode639_1::CS,
            Danish => IsoCode639_1::DA,
            Dutch => IsoCode639_1::NL,
            English => IsoCode639_1::EN,
            Esperanto => IsoCode639_1::EO,
            Estonian => IsoCode639_1::ET,
            Finnish => IsoCode639_1::FI,
            French => IsoCode639_1::FR,
            Ganda => IsoCode639_1::LG,
            Georgian => IsoCode639_1::KA,
            German => IsoCode639_1::DE,
            Greek => IsoCode639_1::EL,
            Gujarati => IsoCode639_1::GU,
            Hebrew => IsoCode639_1::HE,
            Hindi => IsoCode639_1::HI,
            Hungarian => IsoCode639_1::HU,
            Icelandic => IsoCode639_1::IS,
            Indonesian => IsoCode639_1::ID,
            Irish => IsoCode639_1::GA,
            Italian => IsoCode639_1::IT,
            Japanese => IsoCode639_1::JA,
            Kazakh => IsoCode639_1::KK,
            Korean => IsoCode639_1::KO,
            Latin => IsoCode639_1::LA,
            Latvian => IsoCode639_1::LV,
            Lithuanian => IsoCode639_1::LT,
            Macedonian => IsoCode639_1::MK,
            Malay => IsoCode639_1::MS,
            Maori => IsoCode639_1::MI,
            Marathi => IsoCode639_1::MR,
            MongolianHalh => IsoCode639_1::MN,
            Nynorsk => IsoCode639_1::NN,
            Persian => IsoCode639_1::FA,
            Polish => IsoCode639_1::PL,
            Portuguese => IsoCode639_1::PT,
            PunjabiEastern => IsoCode639_1::PA,
            Romanian => IsoCode639_1::RO,
            Russian => IsoCode639_1::RU,
            Serbian => IsoCode639_1::SR,
            Shona => IsoCode639_1::SN,
            Slovak => IsoCode639_1::SK,
            Slovene => IsoCode639_1::SL,
            Somali => IsoCode639_1::SO,
            Sesotho => IsoCode639_1::ST,
            Spanish => IsoCode639_1::ES,
            Swahili => IsoCode639_1::SW,
            Swedish => IsoCode639_1::SV,
            Tagalog => IsoCode639_1::TL,
            Tamil => IsoCode639_1::TA,
            Telugu => IsoCode639_1::TE,
            Thai => IsoCode639_1::TH,
            Tsonga => IsoCode639_1::TS,
            Tswana => IsoCode639_1::TN,
            Turkish => IsoCode639_1::TR,
            Ukrainian => IsoCode639_1::UK,
            Urdu => IsoCode639_1::UR,
            Vietnamese => IsoCode639_1::VI,
            Welsh => IsoCode639_1::CY,
            Xhosa => IsoCode639_1::XH,
            Yoruba => IsoCode639_1::YO,
            Zulu => IsoCode639_1::ZU,
            _ => IsoCode639_1::SQ,
        }
    }

    /// Returns the ISO 639-3 code of this language.
    #[deprecated]
    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        use Language::*;
        match self {
            Afrikaans => IsoCode639_3::AFR,
            AlbanianTosk => IsoCode639_3::SQI,
            Arabic => IsoCode639_3::ARA,
            Armenian => IsoCode639_3::HYE,
            AzerbaijaniNorth => IsoCode639_3::AZE,
            Basque => IsoCode639_3::EUS,
            Belarusian => IsoCode639_3::BEL,
            Bengali => IsoCode639_3::BEN,
            Bokmal => IsoCode639_3::NOB,
            Bosnian => IsoCode639_3::BOS,
            Bulgarian => IsoCode639_3::BUL,
            Catalan => IsoCode639_3::CAT,
            ChineseSimplified => IsoCode639_3::ZHO,
            Croatian => IsoCode639_3::HRV,
            Czech => IsoCode639_3::CES,
            Danish => IsoCode639_3::DAN,
            Dutch => IsoCode639_3::NLD,
            English => IsoCode639_3::ENG,
            Esperanto => IsoCode639_3::EPO,
            Estonian => IsoCode639_3::EST,
            Finnish => IsoCode639_3::FIN,
            French => IsoCode639_3::FRA,
            Ganda => IsoCode639_3::LUG,
            Georgian => IsoCode639_3::KAT,
            German => IsoCode639_3::DEU,
            Greek => IsoCode639_3::ELL,
            Gujarati => IsoCode639_3::GUJ,
            Hebrew => IsoCode639_3::HEB,
            Hindi => IsoCode639_3::HIN,
            Hungarian => IsoCode639_3::HUN,
            Icelandic => IsoCode639_3::ISL,
            Indonesian => IsoCode639_3::IND,
            Irish => IsoCode639_3::GLE,
            Italian => IsoCode639_3::ITA,
            Japanese => IsoCode639_3::JPN,
            Kazakh => IsoCode639_3::KAZ,
            Korean => IsoCode639_3::KOR,
            Latin => IsoCode639_3::LAT,
            Latvian => IsoCode639_3::LAV,
            Lithuanian => IsoCode639_3::LIT,
            Macedonian => IsoCode639_3::MKD,
            Malay => IsoCode639_3::MSA,
            Maori => IsoCode639_3::MRI,
            Marathi => IsoCode639_3::MAR,
            MongolianHalh => IsoCode639_3::MON,
            Nynorsk => IsoCode639_3::NNO,
            Persian => IsoCode639_3::FAS,
            Polish => IsoCode639_3::POL,
            Portuguese => IsoCode639_3::POR,
            PunjabiEastern => IsoCode639_3::PAN,
            Romanian => IsoCode639_3::RON,
            Russian => IsoCode639_3::RUS,
            Serbian => IsoCode639_3::SRP,
            Shona => IsoCode639_3::SNA,
            Slovak => IsoCode639_3::SLK,
            Slovene => IsoCode639_3::SLV,
            Somali => IsoCode639_3::SOM,
            Sesotho => IsoCode639_3::SOT,
            Spanish => IsoCode639_3::SPA,
            Swahili => IsoCode639_3::SWA,
            Swedish => IsoCode639_3::SWE,
            Tagalog => IsoCode639_3::TGL,
            Tamil => IsoCode639_3::TAM,
            Telugu => IsoCode639_3::TEL,
            Thai => IsoCode639_3::THA,
            Tsonga => IsoCode639_3::TSO,
            Tswana => IsoCode639_3::TSN,
            Turkish => IsoCode639_3::TUR,
            Ukrainian => IsoCode639_3::UKR,
            Urdu => IsoCode639_3::URD,
            Vietnamese => IsoCode639_3::VIE,
            Welsh => IsoCode639_3::CYM,
            Xhosa => IsoCode639_3::XHO,
            Yoruba => IsoCode639_3::YOR,
            Zulu => IsoCode639_3::ZUL,
            _ => IsoCode639_3::SQI,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Language::*;
    use ::core::str::FromStr;

    #[test]
    fn test_language_max_value() {
        for lang in Language::iter() {
            assert!(
                (lang as usize) < Language::COUNT,
                "Language value >= it's count"
            );
        }
    }

    #[test]
    fn assert_language_string_representation_is_correct() {
        assert_eq!(English.to_string(), "English");
    }

    #[test]
    fn test_language_serializer() {
        let serialized = serde_json::to_string(&English).unwrap();
        assert_eq!(serialized, "\"ENGLISH\"");
    }

    #[test]
    fn test_language_deserializer() {
        let deserialized = serde_json::from_str::<Language>("\"ENGLISH\"").unwrap();
        assert_eq!(deserialized, English);
    }

    #[test]
    fn test_from_str() {
        let language = Language::from_str("eng").unwrap();
        assert_eq!(language, English);
    }
}
