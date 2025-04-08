use ::std::fmt::Debug;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, EnumString, IntoStaticStr};

// For dialects create new Languages
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
    #[strum(serialize = "aho")]
    Ahom,
    #[strum(serialize = "aka")]
    Akan,
    #[strum(serialize = "akk")]
    Akkadian,
    /// Macro
    #[strum(serialize = "sqi")]
    Albanian,
    #[strum(serialize = "als")]
    AlbanianTosk,
    #[strum(serialize = "amh")]
    Amharic,
    #[strum(serialize = "grc")]
    AncientGreek,
    /// Group
    #[strum(serialize = "narb")]
    AncientNorthArabian,
    /// Group
    #[strum(serialize = "sarb")]
    AncientSouthArabian,
    #[strum(serialize = "akb")]
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
    #[strum(serialize = "elym")]
    AramaicElymaic,
    #[strum(serialize = "hatr")]
    AramaicHatran,
    #[strum(serialize = "arc")]
    AramaicImperial,
    #[strum(serialize = "myz")]
    AramaicMandaic,
    #[strum(serialize = "nabt")]
    AramaicNabataean,
    #[strum(serialize = "palm")]
    AramaicPalmyrene,
    #[strum(serialize = "sam")]
    AramaicSamaritan,
    #[strum(serialize = "syc")]
    AramaicSyriac,
    #[strum(serialize = "hye")]
    Armenian,
    #[strum(serialize = "asm")]
    Assamese,
    #[strum(serialize = "ast")]
    Asturian,
    #[strum(serialize = "ave")]
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
    #[strum(serialize = "bax")]
    Bamum,
    #[strum(serialize = "bjn")]
    Banjar,
    #[strum(serialize = "bap")]
    Bantawa,
    #[strum(serialize = "bak")]
    Bashkir,
    #[strum(serialize = "eus")]
    Basque,
    #[strum(serialize = "bsq")]
    Bassa,
    #[strum(serialize = "bel")]
    Belarusian,
    #[strum(serialize = "bem")]
    Bemba,
    #[strum(serialize = "ben")]
    Bengali,
    #[strum(serialize = "bho")]
    Bhojpuri,
    #[strum(serialize = "bhum")]
    Bhumij,
    #[strum(serialize = "bpy")]
    BishnupriyaManipuri,
    #[strum(serialize = "bos")]
    Bosnian,
    /// Group, any language adapted to Braille
    #[strum(serialize = "brai")]
    Braille,
    #[strum(serialize = "bug")]
    Buginese,
    #[strum(serialize = "bku")]
    Buhid,
    #[strum(serialize = "bul")]
    Bulgarian,
    #[strum(serialize = "mya")]
    Burmese,
    #[strum(serialize = "xcr")]
    Carian,
    #[strum(serialize = "cat")]
    Catalan,
    #[strum(serialize = "xag")]
    CaucasianAlbanian,
    #[strum(serialize = "ceb")]
    Cebuano,
    #[strum(serialize = "ccp")]
    Chakma,
    #[strum(serialize = "cjm")]
    ChamEastern,
    #[strum(serialize = "cja")]
    ChamWestern,
    #[strum(serialize = "chr")]
    Cherokee,
    #[strum(serialize = "hne")]
    Chhattisgarhi,
    /// Macro
    #[strum(serialize = "zho")]
    Chinese,
    #[strum(serialize = "yue")]
    ChineseCantonese,
    #[strum(serialize = "cmn")]
    ChineseMandarin,
    #[strum(serialize = "tuhu")]
    ChineseTuhua,
    #[strum(serialize = "cjk")]
    Chokwe,
    #[strum(serialize = "xco")]
    Chorasmian,
    #[strum(serialize = "chu")]
    ChurchSlavonic,
    #[strum(serialize = "cop")]
    Coptic,
    #[strum(serialize = "cre")]
    Cree,
    #[strum(serialize = "hat")]
    CreoleHaitian,
    #[strum(serialize = "hrv")]
    Croatian,
    #[strum(serialize = "ces")]
    Czech,
    #[strum(serialize = "dan")]
    Danish,
    #[strum(serialize = "div")]
    Dhivehi,
    #[strum(serialize = "luo")]
    Dholuo,
    #[strum(serialize = "dik")]
    DinkaSouthwestern,
    #[strum(serialize = "doi")]
    Dogri,
    #[strum(serialize = "nld")]
    Dutch,
    #[strum(serialize = "dyu")]
    Dyula,
    #[strum(serialize = "dzo")]
    Dzongkha,
    #[strum(serialize = "egyp")]
    EgyptianHieroglyphs,
    #[strum(serialize = "eng")]
    English,
    #[strum(serialize = "epo")]
    Esperanto,
    #[strum(serialize = "est")]
    Estonian,
    #[strum(serialize = "ett")]
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
    #[strum(serialize = "fur")]
    Friulian,
    /// Macro
    #[strum(serialize = "ful")]
    Fulani,
    #[strum(serialize = "fuv")]
    FulfuldeNigerian,
    #[strum(serialize = "gla")]
    GaelicScottish,
    #[strum(serialize = "glg")]
    Galician,
    #[strum(serialize = "lug")]
    Ganda,
    #[strum(serialize = "pgd")]
    Gandhari,
    #[strum(serialize = "gez")]
    Geez,
    #[strum(serialize = "kat")]
    Georgian,
    #[strum(serialize = "deu")]
    German,
    /// Macro
    #[strum(serialize = "gon")]
    Gondi,
    #[strum(serialize = "got")]
    Gothic,
    #[strum(serialize = "ell")]
    Greek,
    #[strum(serialize = "grn")]
    Guarani,
    #[strum(serialize = "guj")]
    Gujarati,
    #[strum(serialize = "gvr")]
    Gurung,
    #[strum(serialize = "hnn")]
    Hanunoo,
    #[strum(serialize = "hau")]
    Hausa,
    #[strum(serialize = "haw")]
    Hawaiian,
    #[strum(serialize = "heb")]
    Hebrew,
    #[strum(serialize = "smp")]
    HebrewSamaritan,
    #[strum(serialize = "hin")]
    Hindi,
    #[strum(serialize = "hit")]
    Hittite,
    /// Macro
    #[strum(serialize = "hmn")]
    Hmong,
    #[strum(serialize = "hoc")]
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
    /// Macro
    #[strum(serialize = "iku")]
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
    #[strum(serialize = "btx")]
    Karo,
    #[strum(serialize = "kas")]
    Kashmiri,
    #[strum(serialize = "eky")]
    KayahEastern,
    #[strum(serialize = "kyu")]
    KayahWestern,
    #[strum(serialize = "kaz")]
    Kazakh,
    #[strum(serialize = "zkt")]
    Khitan,
    #[strum(serialize = "khm")]
    Khmer,
    #[strum(serialize = "kon")]
    Kikongo,
    #[strum(serialize = "kik")]
    Kikuyu,
    #[strum(serialize = "kmb")]
    Kimbundu,
    #[strum(serialize = "kin")]
    Kinyarwanda,
    #[strum(serialize = "kpv")]
    Komi,
    #[strum(serialize = "kor")]
    Korean,
    #[strum(serialize = "ckb")]
    KurdishCentral,
    #[strum(serialize = "kmr")]
    KurdishNorthern,
    #[strum(serialize = "sdh")]
    KurdishSouthern,
    #[strum(serialize = "kfr")]
    Kutchi,
    #[strum(serialize = "kir")]
    Kyrgyz,
    #[strum(serialize = "lao")]
    Lao,
    #[strum(serialize = "ltg")]
    Latgalian,
    #[strum(serialize = "lat")]
    Latin,
    #[strum(serialize = "lvs")]
    Latvian,
    #[strum(serialize = "lep")]
    Lepcha,
    #[strum(serialize = "lij")]
    Ligurian,
    #[strum(serialize = "lif")]
    Limbu,
    #[strum(serialize = "lim")]
    Limburgish,
    #[strum(serialize = "lin")]
    Lingala,
    #[strum(serialize = "lis")]
    Lisu,
    #[strum(serialize = "lit")]
    Lithuanian,
    /// Group
    #[strum(serialize = "lolo")]
    Loloish,
    #[strum(serialize = "lmo")]
    Lombard,
    #[strum(serialize = "lua")]
    LubaKasai,
    #[strum(serialize = "xlu")]
    LuwianCuneiform,
    #[strum(serialize = "hlu")]
    LuwianHieroglyphic,
    #[strum(serialize = "ltz")]
    Luxembourgish,
    #[strum(serialize = "xlc")]
    Lycian,
    #[strum(serialize = "xld")]
    Lydian,
    #[strum(serialize = "mkd")]
    Macedonian,
    #[strum(serialize = "mag")]
    Magahi,
    #[strum(serialize = "mai")]
    Maithili,
    #[strum(serialize = "mak")]
    Makassarese,
    #[strum(serialize = "zsm")]
    Malay,
    #[strum(serialize = "mal")]
    Malayalam,
    #[strum(serialize = "plt")]
    MalgasyPlateau,
    #[strum(serialize = "mlt")]
    Maltese,
    #[strum(serialize = "btm")]
    Mandailing,
    /// Macro
    #[strum(serialize = "man")]
    Manding,
    #[strum(serialize = "mri")]
    Maori,
    #[strum(serialize = "mar")]
    Marathi,
    /// Macro
    #[strum(serialize = "mwr")]
    Marwari,
    #[strum(serialize = "dmf")]
    Medefaidrin,
    #[strum(serialize = "mni")]
    Meitei,
    #[strum(serialize = "men")]
    Mende,
    #[strum(serialize = "xmr")]
    Meroitic,
    #[strum(serialize = "xmn")]
    MiddlePersianManichaean,
    #[strum(serialize = "pal")]
    MiddlePersianPahlavi,
    #[strum(serialize = "min")]
    Minangkabau,
    #[strum(serialize = "omn")]
    Minoan,
    #[strum(serialize = "lab")]
    MinoanLinearA,
    #[strum(serialize = "lus")]
    Mizo,
    #[strum(serialize = "khk")]
    MongolianHalh,
    #[strum(serialize = "mos")]
    Mossi,
    #[strum(serialize = "mro")]
    Mro,
    #[strum(serialize = "unr")]
    Mundari,
    #[strum(serialize = "gmy")]
    MycenaeanGreek,
    #[strum(serialize = "npi")]
    Nepali,
    #[strum(serialize = "new")]
    Newar,
    #[strum(serialize = "nod")]
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
    /// Macro
    #[strum(serialize = "oji")]
    Ojibwe,
    #[strum(serialize = "ang")]
    OldEnglish,
    #[strum(serialize = "sga")]
    OldIrish,
    #[strum(serialize = "kaw")]
    OldJavanese,
    #[strum(serialize = "non")]
    OldNorse,
    #[strum(serialize = "peo")]
    OldPersian,
    #[strum(serialize = "otk")]
    OldTurkic,
    #[strum(serialize = "oui")]
    OldUyghur,
    #[strum(serialize = "gax")]
    OromoSouthern,
    #[strum(serialize = "gaz")]
    OromoWestCentral,
    #[strum(serialize = "osa")]
    Osage,
    #[strum(serialize = "osc")]
    Oscan,
    #[strum(serialize = "btd")]
    Pakpak,
    #[strum(serialize = "pag")]
    Pangasinan,
    #[strum(serialize = "pap")]
    Papiamento,
    #[strum(serialize = "xpr")]
    Parthian,
    #[strum(serialize = "pbt")]
    PashtoSouthern,
    /// Afghani
    #[strum(serialize = "prs")]
    PersianDari,
    /// Iranian
    #[strum(serialize = "pes")]
    PersianWestern,
    #[strum(serialize = "phn")]
    Phoenician,
    #[strum(serialize = "pol")]
    Polish,
    #[strum(serialize = "por")]
    Portuguese,
    /// Group
    #[strum(serialize = "pra")]
    Prakrit,
    #[strum(serialize = "fuf")]
    Pular,
    #[strum(serialize = "pan")]
    PunjabiEastern,
    #[strum(serialize = "quy")]
    QuechuaAyacucho,
    #[strum(serialize = "rej")]
    Rejang,
    #[strum(serialize = "rhg")]
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
    #[strum(serialize = "skr")]
    Saraiki,
    #[strum(serialize = "srd")]
    Sardinian,
    #[strum(serialize = "saz")]
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
    /// Group
    #[strum(serialize = "sgnw")]
    SignLanguages,
    #[strum(serialize = "szl")]
    Silesian,
    #[strum(serialize = "bts")]
    Simalungun,
    #[strum(serialize = "snd")]
    Sindhi,
    #[strum(serialize = "sin")]
    Sinhala,
    #[strum(serialize = "slk")]
    Slovak,
    #[strum(serialize = "slv")]
    Slovene,
    #[strum(serialize = "sog")]
    Sogdian,
    #[strum(serialize = "som")]
    Somali,
    #[strum(serialize = "srb")]
    Sora,
    #[strum(serialize = "spa")]
    Spanish,
    #[strum(serialize = "sux")]
    Sumerian,
    #[strum(serialize = "sun")]
    Sundanese,
    #[strum(serialize = "suz")]
    Sunuwar,
    #[strum(serialize = "swh")]
    Swahili,
    #[strum(serialize = "ssw")]
    Swati,
    #[strum(serialize = "swe")]
    Swedish,
    #[strum(serialize = "syl")]
    Sylheti,
    #[strum(serialize = "tgl")]
    Tagalog,
    #[strum(serialize = "tgw")]
    Tagbanwa,
    #[strum(serialize = "blt")]
    TaiDam,
    #[strum(serialize = "twh")]
    TaiDon,
    #[strum(serialize = "khb")]
    TaiLue,
    #[strum(serialize = "tdd")]
    TaiNuea,
    #[strum(serialize = "tgk")]
    Tajik,
    #[strum(serialize = "taq")]
    Tamasheq,
    #[strum(serialize = "tzm")]
    TamazightCentralAtlas,
    #[strum(serialize = "tam")]
    Tamil,
    /// Macro
    #[strum(serialize = "nst")]
    Tangsa,
    #[strum(serialize = "txg")]
    Tangut,
    #[strum(serialize = "tat")]
    Tatar,
    #[strum(serialize = "crh")]
    TatarCrimean,
    #[strum(serialize = "ctd")]
    Tedim,
    #[strum(serialize = "tel")]
    Telugu,
    #[strum(serialize = "tha")]
    Thai,
    #[strum(serialize = "bod")]
    Tibetan,
    #[strum(serialize = "tir")]
    Tigrinya,
    #[strum(serialize = "bbc")]
    TobaBatak,
    #[strum(serialize = "tpi")]
    TokPisin,
    #[strum(serialize = "txo")]
    Toto,
    #[strum(serialize = "tso")]
    Tsonga,
    #[strum(serialize = "tsn")]
    Tswana,
    #[strum(serialize = "tcy")]
    Tulu,
    #[strum(serialize = "tum")]
    Tumbuka,
    #[strum(serialize = "tur")]
    Turkish,
    #[strum(serialize = "tuk")]
    Turkmen,
    #[strum(serialize = "twi")]
    Twi,
    #[strum(serialize = "uga")]
    Ugaritic,
    #[strum(serialize = "ukr")]
    Ukrainian,
    #[strum(serialize = "xum")]
    Umbrian,
    #[strum(serialize = "umb")]
    Umbundu,
    #[strum(serialize = "urd")]
    Urdu,
    #[strum(serialize = "uig")]
    Uyghur,
    #[strum(serialize = "uzn")]
    UzbekNorthern,
    #[strum(serialize = "vai")]
    Vai,
    #[strum(serialize = "vec")]
    Venetian,
    #[strum(serialize = "vie")]
    Vietnamese,
    #[strum(serialize = "nnp")]
    Wancho,
    #[strum(serialize = "war")]
    Waray,
    #[strum(serialize = "cym")]
    Welsh,
    #[strum(serialize = "wol")]
    Wolof,
    #[strum(serialize = "xho")]
    Xhosa,
    #[strum(serialize = "ydd")]
    YiddishEastern,
    #[strum(serialize = "yor")]
    Yoruba,
    #[strum(serialize = "xzh")]
    Zhangzhung,
    #[strum(serialize = "zul")]
    Zulu,
}
