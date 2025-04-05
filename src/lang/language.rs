use ::std::fmt::Debug;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, EnumString, IntoStaticStr};

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
    EnumCountMacro,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
#[strum(const_into_str)]
// #[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
#[non_exhaustive]
pub enum Language {
    #[strum(serialize = "ace")]
    Acehnese,
    #[strum(serialize = "afr")]
    Afrikaans,
    Ahom,
    #[strum(serialize = "aka")]
    Akan,
    Akkadian,
    AlbanianHistorical,
    #[strum(serialize = "als")]
    AlbanianTosk,
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
    /// Latin
    #[strum(serialize = "azj")]
    AzerbaijaniNorth,
    /// Arabic
    #[strum(serialize = "azb")]
    AzerbaijaniSouth,
    #[strum(serialize = "ban")]
    Balinese,
    #[strum(serialize = "bam")]
    Bambara,
    Bamum,
    #[strum(serialize = "bjn")]
    Banjar,
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
    Bhojpuri,
    BishnupriyaManipuri,
    #[strum(serialize = "bos")]
    Bosnian,
    /// any language adapted to Braille
    Braille,
    BuddhistMarchen,
    #[strum(serialize = "bug")]
    Buginese,
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
    #[strum(serialize = "yue")]
    ChineseCantonese,
    ChineseMandarin,
    #[strum(serialize = "zho")]
    Chinese,
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
    /// used in ancient Cyprus
    CyproMinoan,
    #[strum(serialize = "ces")]
    Czech,
    #[strum(serialize = "dan")]
    Danish,
    #[strum(serialize = "dik")]
    DinkaSouthwestern,
    Dogri,
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
    /// shorthand systems for English
    EnglishDuployan,
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
    Gondi,
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
    Hindi,
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
    Javanese,
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
    #[strum(serialize = "knc")]
    KanuriCentral,
    Karo,
    #[strum(serialize = "kas")]
    Kashmiri,
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
    /// Arabic
    #[strum(serialize = "ckb")]
    KurdishCentral,
    /// Latin
    #[strum(serialize = "kmr")]
    KurdishNorthern,
    /// Arabic
    KurdishSouthern,
    KurdishYezidi,
    #[strum(serialize = "kir")]
    Kyrgyz,
    #[strum(serialize = "lao")]
    Lao,
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
    Magahi,
    #[strum(serialize = "mai")]
    Maithili,
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
    Marathi,
    Marwari,
    Medefaidrin,
    #[strum(serialize = "mni")]
    Meitei,
    Mende,
    Meroitic,
    MiddlePersian,
    #[strum(serialize = "min")]
    Minangkabau,
    Minoan,
    #[strum(serialize = "lus")]
    Mizo,
    #[strum(serialize = "khk")]
    MongolianHalh,
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
    #[strum(serialize = "oci")]
    Occitan,
    #[strum(serialize = "ory")]
    Odia,
    Ojibwe,
    OldChurchSlavonic,
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
    #[strum(serialize = "gax")]
    OromoSouthern,
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
    /// Afghani
    #[strum(serialize = "prs")]
    PersianDari,
    /// Iranian
    #[strum(serialize = "pes")]
    PersianWestern,
    Phoenician,
    #[strum(serialize = "pol")]
    Polish,
    #[strum(serialize = "por")]
    Portuguese,
    Prakrit,
    Pular,
    #[strum(serialize = "pan")]
    PunjabiEastern,
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
    Sanskrit,
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
    Sindhi,
    #[strum(serialize = "sin")]
    Sinhala,
    #[strum(serialize = "slk")]
    Slovak,
    #[strum(serialize = "slv")]
    Slovene,
    Sogdian,
    #[strum(serialize = "som")]
    Somali,
    Sora,
    #[strum(serialize = "spa")]
    Spanish,
    Sumerian,
    #[strum(serialize = "sun")]
    Sundanese,
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
    Tagalog,
    Tagbanwa,
    TaiDam,
    TaiDon,
    TaiLe,
    TaiLue,
    #[strum(serialize = "tgk")]
    Tajik,
    #[strum(serialize = "taq")]
    Tamasheq,
    #[strum(serialize = "tzm")]
    TamazightCentralAtlas,
    #[strum(serialize = "tam")]
    Tamil,
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
    Wolof,
    #[strum(serialize = "xho")]
    Xhosa,
    Yi,
    #[strum(serialize = "ydd")]
    YiddishEastern,
    #[strum(serialize = "yor")]
    Yoruba,
    ZoLanguages,
    #[strum(serialize = "zul")]
    Zulu,
}
