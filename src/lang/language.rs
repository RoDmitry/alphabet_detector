use ::std::fmt::Debug;
use alphabet_detector_macros::Language;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

// For dialect create a new Language.
/// Int representation is unstable and can be changed anytime.
/// Code representation (const
/// [`into_code`](enum.Language.html#method.into_code)/[`from_code`](enum.Language.html#method.from_code))
/// or string representation (const
/// [`into_str`](enum.Language.html#method.into_str)/[`from_str`](enum.Language.html#method.from_str))
/// are more stable.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumCountMacro, EnumIter, Language,
)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
#[non_exhaustive]
pub enum Language {
    #[language(short = "ace")]
    Acehnese,
    #[language(short = "afr")]
    Afrikaans,
    #[language(short = "aho")]
    Ahom,
    #[language(short = "aka")]
    Akan,
    #[language(short = "akk")]
    Akkadian,
    /// Macro
    #[language(short = "sqi")]
    Albanian,
    #[language(short = "als")]
    AlbanianTosk,
    #[language(short = "amh")]
    Amharic,
    #[language(short = "grc")]
    AncientGreek,
    /// Group
    #[language(short = "narb")]
    AncientNorthArabian,
    /// Group
    #[language(short = "sarb")]
    AncientSouthArabian,
    #[language(short = "akb")]
    Angkola,
    #[language(short = "arb")]
    Arabic,
    #[language(short = "arz")]
    ArabicEgyptian,
    #[language(short = "acm")]
    ArabicMesopotamian,
    #[language(short = "ary")]
    ArabicMoroccan,
    #[language(short = "ars")]
    ArabicNajdi,
    #[language(short = "apc")]
    ArabicNorthLevantine,
    #[language(short = "acq")]
    ArabicSouthernYemeni,
    #[language(short = "ajp")]
    ArabicSouthLevantine,
    #[language(short = "aeb")]
    ArabicTunisian,
    #[language(short = "elym")]
    AramaicElymaic,
    #[language(short = "hatr")]
    AramaicHatran,
    #[language(short = "arc")]
    AramaicImperial,
    #[language(short = "myz")]
    AramaicMandaic,
    #[language(short = "nabt")]
    AramaicNabataean,
    #[language(short = "palm")]
    AramaicPalmyrene,
    #[language(short = "sam")]
    AramaicSamaritan,
    #[language(short = "syc")]
    AramaicSyriac,
    #[language(short = "hye")]
    Armenian,
    #[language(short = "asm")]
    Assamese,
    #[language(short = "ast")]
    Asturian,
    #[language(short = "ave")]
    Avestan,
    #[language(short = "awa")]
    Awadhi,
    #[language(short = "ayr")]
    AymaraCentral,
    /// Latin
    #[language(short = "azj")]
    AzerbaijaniNorth,
    /// Arabic
    #[language(short = "azb")]
    AzerbaijaniSouth,
    #[language(short = "ban")]
    Balinese,
    #[language(short = "bam")]
    Bambara,
    #[language(short = "bax")]
    Bamum,
    #[language(short = "bjn")]
    Banjar,
    #[language(short = "bap")]
    Bantawa,
    #[language(short = "bak")]
    Bashkir,
    #[language(short = "eus")]
    Basque,
    #[language(short = "bsq")]
    Bassa,
    #[language(short = "bel")]
    Belarusian,
    #[language(short = "bem")]
    Bemba,
    #[language(short = "ben")]
    Bengali,
    #[language(short = "bho")]
    Bhojpuri,
    #[language(short = "bhum")]
    Bhumij,
    #[language(short = "bpy")]
    BishnupriyaManipuri,
    #[language(short = "bos")]
    Bosnian,
    /// Group, any language adapted to Braille
    #[language(short = "brai")]
    Braille,
    #[language(short = "bug")]
    Buginese,
    #[language(short = "bku")]
    Buhid,
    #[language(short = "bul")]
    Bulgarian,
    #[language(short = "mya")]
    Burmese,
    #[language(short = "xcr")]
    Carian,
    #[language(short = "cat")]
    Catalan,
    #[language(short = "xag")]
    CaucasianAlbanian,
    #[language(short = "ceb")]
    Cebuano,
    #[language(short = "ccp")]
    Chakma,
    #[language(short = "cjm")]
    ChamEastern,
    #[language(short = "cja")]
    ChamWestern,
    #[language(short = "chr")]
    Cherokee,
    #[language(short = "hne")]
    Chhattisgarhi,
    // Macro
    // #[language(short = "zho")]
    // Chinese,
    #[language(short = "yue")]
    ChineseCantonese,
    #[language(short = "cmn")]
    ChineseMandarin,
    #[language(short = "tuhu")]
    ChineseTuhua,
    #[language(short = "cjk")]
    Chokwe,
    #[language(short = "xco")]
    Chorasmian,
    #[language(short = "chu")]
    ChurchSlavonic,
    #[language(short = "chv")]
    Chuvash,
    #[language(short = "cop")]
    Coptic,
    #[language(short = "cre")]
    Cree,
    #[language(short = "hat")]
    CreoleHaitian,
    #[language(short = "hrv")]
    Croatian,
    #[language(short = "ces")]
    Czech,
    #[language(short = "dan")]
    Danish,
    #[language(short = "div")]
    Dhivehi,
    #[language(short = "luo")]
    Dholuo,
    #[language(short = "dik")]
    DinkaSouthwestern,
    #[language(short = "doi")]
    Dogri,
    #[language(short = "nld")]
    Dutch,
    #[language(short = "dyu")]
    Dyula,
    #[language(short = "dzo")]
    Dzongkha,
    #[language(short = "egyp")]
    EgyptianHieroglyphs,
    #[language(short = "eng")]
    English,
    #[language(short = "epo")]
    Esperanto,
    #[language(short = "ekk")]
    Estonian,
    #[language(short = "ett")]
    Etruscan,
    #[language(short = "ewe")]
    Ewe,
    #[language(short = "fao")]
    Faroese,
    #[language(short = "fij")]
    Fijian,
    #[language(short = "fil")]
    Filipino,
    #[language(short = "fin")]
    Finnish,
    #[language(short = "fon")]
    Fon,
    #[language(short = "fra")]
    French,
    #[language(short = "fur")]
    Friulian,
    /// Macro
    #[language(short = "ful")]
    Fulani,
    #[language(short = "fuv")]
    FulfuldeNigerian,
    #[language(short = "gla")]
    GaelicScottish,
    #[language(short = "glg")]
    Galician,
    #[language(short = "lug")]
    Ganda,
    #[language(short = "pgd")]
    Gandhari,
    #[language(short = "gez")]
    Geez,
    #[language(short = "kat")]
    Georgian,
    #[language(short = "deu")]
    German,
    /// Macro
    #[language(short = "gon")]
    Gondi,
    #[language(short = "got")]
    Gothic,
    #[language(short = "ell")]
    Greek,
    #[language(short = "gug")]
    Guarani,
    #[language(short = "guj")]
    Gujarati,
    #[language(short = "gvr")]
    Gurung,
    #[language(short = "hnn")]
    Hanunoo,
    #[language(short = "hau")]
    Hausa,
    #[language(short = "haw")]
    Hawaiian,
    #[language(short = "heb")]
    Hebrew,
    #[language(short = "smp")]
    HebrewSamaritan,
    #[language(short = "hin")]
    Hindi,
    #[language(short = "hit")]
    Hittite,
    /// Macro
    #[language(short = "hmn")]
    Hmong,
    #[language(short = "hoc")]
    Ho,
    #[language(short = "hun")]
    Hungarian,
    #[language(short = "isl")]
    Icelandic,
    #[language(short = "ibo")]
    Igbo,
    #[language(short = "ilo")]
    Ilocano,
    #[language(short = "ind")]
    Indonesian,
    /// Macro
    #[language(short = "iku")]
    Inuktitut,
    #[language(short = "gle")]
    Irish,
    #[language(short = "ita")]
    Italian,
    #[language(short = "jpn")]
    Japanese,
    #[language(short = "jav")]
    Javanese,
    #[language(short = "kac")]
    Jingpho,
    #[language(short = "kbp")]
    Kabiye,
    #[language(short = "kea")]
    Kabuverdianu,
    #[language(short = "kab")]
    Kabyle,
    #[language(short = "kam")]
    Kamba,
    #[language(short = "kan")]
    Kannada,
    #[language(short = "knc")]
    KanuriCentral,
    #[language(short = "btx")]
    Karo,
    #[language(short = "kas")]
    Kashmiri,
    #[language(short = "eky")]
    KayahEastern,
    #[language(short = "kyu")]
    KayahWestern,
    #[language(short = "kaz")]
    Kazakh,
    #[language(short = "zkt")]
    Khitan,
    #[language(short = "khm")]
    Khmer,
    #[language(short = "ktu")]
    KikongoKituba,
    #[language(short = "kik")]
    Kikuyu,
    #[language(short = "kmb")]
    Kimbundu,
    #[language(short = "kin")]
    Kinyarwanda,
    #[language(short = "kpv")]
    Komi,
    #[language(short = "kor")]
    Korean,
    #[language(short = "ckb")]
    KurdishCentral,
    #[language(short = "kmr")]
    KurdishNorthern,
    #[language(short = "sdh")]
    KurdishSouthern,
    #[language(short = "kfr")]
    Kutchi,
    #[language(short = "kir")]
    Kyrgyz,
    #[language(short = "lao")]
    Lao,
    #[language(short = "ltg")]
    Latgalian,
    #[language(short = "lat")]
    Latin,
    #[language(short = "lvs")]
    Latvian,
    #[language(short = "lep")]
    Lepcha,
    #[language(short = "lij")]
    Ligurian,
    #[language(short = "lif")]
    Limbu,
    #[language(short = "lim")]
    Limburgish,
    #[language(short = "lin")]
    Lingala,
    #[language(short = "lis")]
    Lisu,
    #[language(short = "lit")]
    Lithuanian,
    /// Group
    #[language(short = "lolo")]
    Loloish,
    #[language(short = "lmo")]
    Lombard,
    #[language(short = "lua")]
    LubaKasai,
    #[language(short = "xlu")]
    LuwianCuneiform,
    #[language(short = "hlu")]
    LuwianHieroglyphic,
    #[language(short = "ltz")]
    Luxembourgish,
    #[language(short = "xlc")]
    Lycian,
    #[language(short = "xld")]
    Lydian,
    #[language(short = "mkd")]
    Macedonian,
    #[language(short = "mag")]
    Magahi,
    #[language(short = "mai")]
    Maithili,
    #[language(short = "mak")]
    Makassarese,
    #[language(short = "zsm")]
    Malay,
    #[language(short = "mal")]
    Malayalam,
    #[language(short = "plt")]
    MalgasyPlateau,
    #[language(short = "mlt")]
    Maltese,
    #[language(short = "btm")]
    Mandailing,
    /// Macro
    #[language(short = "man")]
    Manding,
    #[language(short = "mri")]
    Maori,
    #[language(short = "mar")]
    Marathi,
    /// Macro
    #[language(short = "mwr")]
    Marwari,
    #[language(short = "dmf")]
    Medefaidrin,
    #[language(short = "mni")]
    Meitei,
    #[language(short = "men")]
    Mende,
    #[language(short = "xmr")]
    Meroitic,
    #[language(short = "xmn")]
    MiddlePersianManichaean,
    #[language(short = "pal")]
    MiddlePersianPahlavi,
    #[language(short = "min")]
    Minangkabau,
    #[language(short = "omn")]
    Minoan,
    #[language(short = "lab")]
    MinoanLinearA,
    #[language(short = "lus")]
    Mizo,
    #[language(short = "khk")]
    MongolianHalh,
    #[language(short = "mos")]
    Mossi,
    #[language(short = "mro")]
    Mro,
    #[language(short = "unr")]
    Mundari,
    #[language(short = "gmy")]
    MycenaeanGreek,
    #[language(short = "npi")]
    Nepali,
    #[language(short = "new")]
    Newar,
    #[language(short = "nod")]
    NorthernThai,
    #[language(short = "nob")]
    NorwegianBokmal,
    #[language(short = "nno")]
    NorwegianNynorsk,
    #[language(short = "nus")]
    Nuer,
    #[language(short = "nya")]
    Nyanja,
    #[language(short = "oci")]
    Occitan,
    #[language(short = "ory")]
    Odia,
    /// Macro
    #[language(short = "oji")]
    Ojibwe,
    #[language(short = "ang")]
    OldEnglish,
    #[language(short = "sga")]
    OldIrish,
    #[language(short = "kaw")]
    OldJavanese,
    #[language(short = "non")]
    OldNorse,
    #[language(short = "peo")]
    OldPersian,
    #[language(short = "otk")]
    OldTurkic,
    #[language(short = "oui")]
    OldUyghur,
    #[language(short = "gax")]
    OromoSouthern,
    #[language(short = "gaz")]
    OromoWestCentral,
    #[language(short = "osa")]
    Osage,
    #[language(short = "osc")]
    Oscan,
    #[language(short = "btd")]
    Pakpak,
    #[language(short = "pag")]
    Pangasinan,
    #[language(short = "pap")]
    Papiamento,
    #[language(short = "xpr")]
    Parthian,
    #[language(short = "pbt")]
    PashtoSouthern,
    /// Afghani
    #[language(short = "prs")]
    PersianDari,
    /// Iranian
    #[language(short = "pes")]
    PersianWestern,
    #[language(short = "phn")]
    Phoenician,
    #[language(short = "pol")]
    Polish,
    #[language(short = "por")]
    Portuguese,
    /// Group
    #[language(short = "pra")]
    Prakrit,
    #[language(short = "fuf")]
    Pular,
    #[language(short = "pan")]
    PunjabiEastern,
    #[language(short = "quy")]
    QuechuaAyacucho,
    #[language(short = "rej")]
    Rejang,
    #[language(short = "rhg")]
    Rohingya,
    #[language(short = "ron")]
    Romanian,
    #[language(short = "run")]
    Rundi,
    #[language(short = "rus")]
    Russian,
    #[language(short = "smo")]
    Samoan,
    #[language(short = "sag")]
    Sango,
    #[language(short = "san")]
    Sanskrit,
    #[language(short = "sat")]
    Santali,
    #[language(short = "skr")]
    Saraiki,
    #[language(short = "srd")]
    Sardinian,
    #[language(short = "saz")]
    Saurashtra,
    #[language(short = "nso")]
    Sepedi,
    #[language(short = "srp")]
    Serbian,
    #[language(short = "sot")]
    Sesotho,
    #[language(short = "shn")]
    Shan,
    #[language(short = "sna")]
    Shona,
    #[language(short = "scn")]
    Sicilian,
    /// Group
    #[language(short = "sgnw")]
    SignLanguages,
    #[language(short = "szl")]
    Silesian,
    #[language(short = "bts")]
    Simalungun,
    #[language(short = "snd")]
    Sindhi,
    #[language(short = "sin")]
    Sinhala,
    #[language(short = "slk")]
    Slovak,
    #[language(short = "slv")]
    Slovene,
    #[language(short = "sog")]
    Sogdian,
    #[language(short = "som")]
    Somali,
    #[language(short = "srb")]
    Sora,
    #[language(short = "spa")]
    Spanish,
    #[language(short = "sux")]
    Sumerian,
    #[language(short = "sun")]
    Sundanese,
    #[language(short = "suz")]
    Sunuwar,
    #[language(short = "swh")]
    Swahili,
    #[language(short = "ssw")]
    Swati,
    #[language(short = "swe")]
    Swedish,
    #[language(short = "syl")]
    Sylheti,
    #[language(short = "tgw")]
    Tagbanwa,
    #[language(short = "blt")]
    TaiDam,
    #[language(short = "twh")]
    TaiDon,
    #[language(short = "khb")]
    TaiLue,
    #[language(short = "tdd")]
    TaiNuea,
    #[language(short = "tgk")]
    Tajik,
    #[language(short = "taq")]
    Tamasheq,
    #[language(short = "tzm")]
    TamazightCentralAtlas,
    #[language(short = "tam")]
    Tamil,
    /// Macro
    #[language(short = "nst")]
    Tangsa,
    #[language(short = "txg")]
    Tangut,
    #[language(short = "tat")]
    Tatar,
    #[language(short = "crh")]
    TatarCrimean,
    #[language(short = "ctd")]
    Tedim,
    #[language(short = "tel")]
    Telugu,
    #[language(short = "tha")]
    Thai,
    #[language(short = "bod")]
    Tibetan,
    #[language(short = "tir")]
    Tigrinya,
    #[language(short = "bbc")]
    TobaBatak,
    #[language(short = "tpi")]
    TokPisin,
    #[language(short = "txo")]
    Toto,
    #[language(short = "tso")]
    Tsonga,
    #[language(short = "tsn")]
    Tswana,
    #[language(short = "tcy")]
    Tulu,
    #[language(short = "tum")]
    Tumbuka,
    #[language(short = "tur")]
    Turkish,
    #[language(short = "tuk")]
    Turkmen,
    #[language(short = "twi")]
    Twi,
    #[language(short = "uga")]
    Ugaritic,
    #[language(short = "ukr")]
    Ukrainian,
    #[language(short = "xum")]
    Umbrian,
    #[language(short = "umb")]
    Umbundu,
    #[language(short = "urd")]
    Urdu,
    #[language(short = "uig")]
    Uyghur,
    #[language(short = "uzn")]
    UzbekNorthern,
    #[language(short = "vai")]
    Vai,
    #[language(short = "vec")]
    Venetian,
    #[language(short = "vie")]
    Vietnamese,
    #[language(short = "nnp")]
    Wancho,
    #[language(short = "war")]
    Waray,
    #[language(short = "cym")]
    Welsh,
    #[language(short = "wol")]
    Wolof,
    #[language(short = "xho")]
    Xhosa,
    #[language(short = "ydd")]
    YiddishEastern,
    #[language(short = "yor")]
    Yoruba,
    #[language(short = "xzh")]
    Zhangzhung,
    #[language(short = "zul")]
    Zulu,
}

impl_try_from!(Language, u32, u32 i32 usize isize u64 i64 u128 i128);
impl_serde!(Language, "Language");
