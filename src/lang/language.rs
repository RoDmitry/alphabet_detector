use ::std::fmt::Debug;
use alphabet_detector_macros::Language;
use strum_macros::{EnumCount as EnumCountDerive, EnumIter};

// For dialect create a new Language. Avoid macrolanguages.
// short = ISO 639-3 code. shortest = ISO 639-1 code.
/// Int representation is unstable and can be changed anytime.
/// Code representation (const
/// [`into_code`](enum.Language.html#method.into_code)/[`from_code`](enum.Language.html#method.from_code))
/// or string representation (const
/// [`into_str`](enum.Language.html#method.into_str)/[`from_str`](enum.Language.html#method.from_str))
/// are more stable.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumCountDerive, EnumIter, Language,
)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
pub enum Language {
    #[language(short = "abk", shortest = "ab")]
    Abkhaz,
    #[language(short = "ace", shortest = "ac")]
    Acehnese,
    #[language(short = "afr", shortest = "af")]
    Afrikaans,
    #[language(short = "aho")]
    Ahom,
    #[language(short = "fat", shortest = "ak")]
    AkanFante,
    /// default Akan
    #[language(short = "twi", old_short = "aka", shortest = "tw")]
    AkanTwi,
    #[language(short = "akk")]
    Akkadian,
    /// Macro, unique scripts
    #[language(short = "sqi", old_short = "alb", shortest = "sq")]
    Albanian,
    #[language(short = "als", shortest = "sq")]
    AlbanianTosk,
    #[language(short = "amh", shortest = "am")]
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
    #[language(short = "arb", shortest = "ar")]
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
    #[language(short = "apd")]
    ArabicSudanese,
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
    #[language(short = "hye", old_short = "arm", shortest = "hy")]
    Armenian,
    #[language(short = "asm", shortest = "as")]
    Assamese,
    #[language(short = "ast")]
    Asturian,
    #[language(short = "ave", shortest = "ae")]
    Avestan,
    #[language(short = "awa")]
    Awadhi,
    #[language(short = "ayr", old_short = "aym", shortest = "ay")]
    AymaraCentral,
    /// Latin
    #[language(short = "azj", shortest = "az")]
    AzerbaijaniNorth,
    /// Arabic
    #[language(short = "azb", shortest = "az")]
    AzerbaijaniSouth,
    #[language(short = "ban")]
    Balinese,
    #[language(short = "bam", shortest = "bm")]
    Bambara,
    #[language(short = "bax")]
    Bamum,
    #[language(short = "bjn")]
    Banjar,
    #[language(short = "bap")]
    Bantawa,
    #[language(short = "bak", shortest = "ba")]
    Bashkir,
    #[language(short = "eus", old_short = "baq", shortest = "eu")]
    Basque,
    #[language(short = "bsq")]
    Bassa,
    #[language(short = "bel", shortest = "be")]
    Belarusian,
    #[language(short = "bem")]
    Bemba,
    #[language(short = "ben", shortest = "bn")]
    Bengali,
    #[language(short = "bho")]
    Bhojpuri,
    #[language(short = "bhum")]
    Bhumij,
    #[language(short = "bpy")]
    BishnupriyaManipuri,
    #[language(short = "bos", shortest = "bs")]
    Bosnian,
    /// Group, any language adapted to Braille
    #[language(short = "brai")]
    Braille,
    #[language(short = "bug")]
    Buginese,
    #[language(short = "bku")]
    Buhid,
    #[language(short = "bul", shortest = "bg")]
    Bulgarian,
    #[language(short = "mya", old_short = "bur", shortest = "my")]
    Burmese,
    #[language(short = "xcr")]
    Carian,
    #[language(short = "cat", shortest = "ca")]
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
    #[language(short = "che", shortest = "ce")]
    Chechen,
    #[language(short = "chr")]
    Cherokee,
    #[language(short = "hne")]
    Chhattisgarhi,
    #[language(short = "yue", shortest = "zh")]
    ChineseCantonese,
    /// default Chinese
    #[language(short = "cmn", old_short = ["zho", "chi"], shortest = "zh")]
    ChineseMandarin,
    #[language(short = "tuhu")]
    ChineseTuhua,
    #[language(short = "cjk")]
    Chokwe,
    #[language(short = "xco")]
    Chorasmian,
    #[language(short = "chu", shortest = "cu")]
    ChurchSlavonic,
    #[language(short = "chv", shortest = "cv")]
    Chuvash,
    #[language(short = "cop")]
    Coptic,
    /// Macro, unique script
    #[language(short = "cre", shortest = "cr")]
    Cree,
    #[language(short = "hat", shortest = "ht")]
    CreoleHaitian,
    #[language(short = "hrv", shortest = "hr")]
    Croatian,
    #[language(short = "ces", old_short = "cze", shortest = "cs")]
    Czech,
    #[language(short = "dan", shortest = "da")]
    Danish,
    #[language(short = "div", shortest = "dv")]
    Dhivehi,
    #[language(short = "luo")]
    Dholuo,
    #[language(short = "dik")]
    DinkaSouthwestern,
    #[language(short = "dgo", old_short = "doi")]
    Dogri,
    #[language(short = "xnr")]
    DogriKangri,
    #[language(short = "nld", old_short = "dut", shortest = "nl")]
    Dutch,
    #[language(short = "dyu")]
    Dyula,
    #[language(short = "dzo", shortest = "dz")]
    Dzongkha,
    #[language(short = "egyp")]
    EgyptianHieroglyphs,
    #[language(short = "eng", shortest = "en")]
    English,
    #[language(short = "epo", shortest = "eo")]
    Esperanto,
    #[language(short = "ekk", old_short = "est", shortest = "et")]
    Estonian,
    #[language(short = "ett")]
    Etruscan,
    #[language(short = "ewe", shortest = "ee")]
    Ewe,
    #[language(short = "fao", shortest = "fo")]
    Faroese,
    #[language(short = "fij", shortest = "fj")]
    Fijian,
    #[language(short = "fil", old_short = "tgl", shortest = "tl")]
    Filipino,
    #[language(short = "fin", shortest = "fi")]
    Finnish,
    #[language(short = "fon")]
    Fon,
    #[language(short = "fra", old_short = "fre", shortest = "fr")]
    French,
    #[language(short = "fur")]
    Friulian,
    #[language(short = "fuc", shortest = "ff")]
    FulaPulaar,
    #[language(short = "fuf", shortest = "ff")]
    FulaPular,
    #[language(short = "fuv", shortest = "ff")]
    FulfuldeNigerian,
    #[language(short = "gla", shortest = "gd")]
    GaelicScottish,
    #[language(short = "glg", shortest = "gl")]
    Galician,
    #[language(short = "lug", shortest = "lg")]
    Ganda,
    #[language(short = "pgd")]
    Gandhari,
    #[language(short = "gez")]
    Geez,
    #[language(short = "kat", old_short = "geo", shortest = "ka")]
    Georgian,
    #[language(short = "deu", old_short = "ger", shortest = "de")]
    German,
    /// Macro, unique scripts
    #[language(short = "gon")]
    Gondi,
    #[language(short = "got")]
    Gothic,
    #[language(short = "ell", old_short = "gre", shortest = "el")]
    Greek,
    #[language(short = "gug", old_short = "grn", shortest = "gn")]
    GuaraniParaguayan,
    #[language(short = "guj", shortest = "gu")]
    Gujarati,
    #[language(short = "gvr")]
    Gurung,
    #[language(short = "hnn")]
    Hanunoo,
    #[language(short = "hau", shortest = "ha")]
    Hausa,
    #[language(short = "haw")]
    Hawaiian,
    #[language(short = "heb", shortest = "he")]
    Hebrew,
    #[language(short = "smp", shortest = "he")]
    HebrewSamaritan,
    #[language(short = "hin", shortest = "hi")]
    Hindi,
    #[language(short = "hit")]
    Hittite,
    /// Macro, unique scripts
    #[language(short = "hmn")]
    Hmong,
    #[language(short = "hoc")]
    Ho,
    #[language(short = "hun", shortest = "hu")]
    Hungarian,
    #[language(short = "isl", old_short = "ice", shortest = "is")]
    Icelandic,
    #[language(short = "ibo", shortest = "ig")]
    Igbo,
    #[language(short = "ilo")]
    Ilocano,
    #[language(short = "ind", shortest = "id")]
    Indonesian,
    /// Macro, unique script
    #[language(short = "iku", shortest = "iu")]
    Inuktitut,
    #[language(short = "gle", shortest = "ga")]
    Irish,
    #[language(short = "ita", shortest = "it")]
    Italian,
    #[language(short = "jpn", shortest = "ja")]
    Japanese,
    #[language(short = "jav", shortest = "jv")]
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
    #[language(short = "kan", shortest = "kn")]
    Kannada,
    #[language(short = "knc", shortest = "kr")]
    KanuriCentral,
    #[language(short = "btx")]
    Karo,
    #[language(short = "kas", shortest = "ks")]
    Kashmiri,
    #[language(short = "eky")]
    KayahEastern,
    #[language(short = "kyu")]
    KayahWestern,
    #[language(short = "kaz", shortest = "kk")]
    Kazakh,
    #[language(short = "zkt")]
    Khitan,
    #[language(short = "khm", shortest = "km")]
    Khmer,
    #[language(short = "ktu")]
    KikongoKituba,
    #[language(short = "kik", shortest = "ki")]
    Kikuyu,
    #[language(short = "kmb")]
    Kimbundu,
    #[language(short = "kin", shortest = "rw")]
    Kinyarwanda,
    #[language(short = "koi", shortest = "kv")]
    KomiPermyak,
    #[language(short = "kpv", shortest = "kv")]
    KomiZyrian,
    #[language(short = "kor", shortest = "ko")]
    Korean,
    #[language(short = "ckb", shortest = "ku")]
    KurdishCentral,
    #[language(short = "kmr", shortest = "ku")]
    KurdishNorthern,
    #[language(short = "sdh", shortest = "ku")]
    KurdishSouthern,
    #[language(short = "kru")]
    Kurukh,
    #[language(short = "kfr")]
    Kutchi,
    #[language(short = "kir", shortest = "ky")]
    Kyrgyz,
    #[language(short = "lao", shortest = "lo")]
    Lao,
    #[language(short = "ltg", shortest = "lv")]
    Latgalian,
    #[language(short = "lat", shortest = "la")]
    Latin,
    #[language(short = "lvs", shortest = "lv")]
    Latvian,
    #[language(short = "lep")]
    Lepcha,
    #[language(short = "lij")]
    Ligurian,
    #[language(short = "lif")]
    Limbu,
    #[language(short = "lim", shortest = "li")]
    Limburgish,
    #[language(short = "lin", shortest = "ln")]
    Lingala,
    #[language(short = "lis")]
    Lisu,
    #[language(short = "lit", shortest = "lt")]
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
    #[language(short = "ltz", shortest = "lb")]
    Luxembourgish,
    #[language(short = "xlc")]
    Lycian,
    #[language(short = "xld")]
    Lydian,
    #[language(short = "mkd", old_short = "mac", shortest = "mk")]
    Macedonian,
    #[language(short = "mag")]
    Magahi,
    #[language(short = "mai")]
    Maithili,
    #[language(short = "mak")]
    Makassarese,
    #[language(short = "plt", shortest = "mg")]
    MalagasyPlateau,
    #[language(short = "zsm", old_short = ["msa", "may"], shortest = "ms")]
    Malay,
    #[language(short = "mal", shortest = "ml")]
    Malayalam,
    #[language(short = "mlt", shortest = "mt")]
    Maltese,
    #[language(short = "btm")]
    Mandailing,
    /// Macro, unique script
    #[language(short = "man")]
    Manding,
    #[language(short = "mri", old_short = "mao", shortest = "mi")]
    Maori,
    #[language(short = "mar", shortest = "mr")]
    Marathi,
    #[language(short = "mhr", old_short = "chm")]
    MariEastern,
    /// Macro, unique script
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
    #[language(short = "khk", shortest = "mn")]
    MongolianHalh,
    #[language(short = "mos")]
    Mossi,
    #[language(short = "mro")]
    Mro,
    #[language(short = "unr")]
    Mundari,
    #[language(short = "gmy")]
    MycenaeanGreek,
    #[language(short = "yrk")]
    Nenets,
    #[language(short = "npi", shortest = "ne")]
    Nepali,
    #[language(short = "new")]
    Newar,
    #[language(short = "pcm")]
    NigerianPidgin,
    #[language(short = "nod")]
    NorthernThai,
    #[language(short = "nob", shortest = "nb")]
    NorwegianBokmal,
    #[language(short = "nno", shortest = "nn")]
    NorwegianNynorsk,
    #[language(short = "nus")]
    Nuer,
    #[language(short = "nya", shortest = "ny")]
    Nyanja,
    #[language(short = "oci", shortest = "oc")]
    Occitan,
    /// Macro, unique script
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
    #[language(short = "ory", shortest = "or")]
    OriyaOdia,
    #[language(short = "gax", shortest = "om")]
    OromoSouthern,
    #[language(short = "gaz", shortest = "om")]
    OromoWestCentral,
    #[language(short = "osa")]
    Osage,
    #[language(short = "osc")]
    Oscan,
    #[language(short = "oss", shortest = "os")]
    Ossetian,
    #[language(short = "btd")]
    Pakpak,
    #[language(short = "pag")]
    Pangasinan,
    #[language(short = "pap")]
    Papiamento,
    #[language(short = "xpr")]
    Parthian,
    #[language(short = "pbt", shortest = "ps")]
    PashtoSouthern,
    /// Afghani
    #[language(short = "prs", shortest = "fa")]
    PersianDari,
    /// Iranian
    #[language(short = "pes", old_short = ["fas", "per"], shortest = "fa")]
    PersianWestern,
    #[language(short = "phn")]
    Phoenician,
    #[language(short = "pol", shortest = "pl")]
    Polish,
    #[language(short = "por", shortest = "pt")]
    Portuguese,
    /// Group
    #[language(short = "pra")]
    Prakrit,
    #[language(short = "pan", shortest = "pa")]
    PunjabiEastern,
    #[language(short = "quy", shortest = "qu")]
    QuechuaAyacucho,
    #[language(short = "rej")]
    Rejang,
    #[language(short = "rhg")]
    Rohingya,
    #[language(short = "ron", old_short = "rum", shortest = "ro")]
    Romanian,
    #[language(short = "run", shortest = "rn")]
    Rundi,
    #[language(short = "rus", shortest = "ru")]
    Russian,
    #[language(short = "sme", shortest = "se")]
    SamiNorthern,
    #[language(short = "smo", shortest = "sm")]
    Samoan,
    #[language(short = "sag", shortest = "sg")]
    Sango,
    #[language(short = "cls", old_short = "san", shortest = "sa")]
    Sanskrit,
    #[language(short = "vsn", shortest = "sa")]
    SanskritVedic,
    #[language(short = "sat")]
    Santali,
    #[language(short = "skr")]
    Saraiki,
    /// Macro, hard to separate
    #[language(short = "srd", shortest = "sc")]
    Sardinian,
    #[language(short = "saz")]
    Saurashtra,
    #[language(short = "nso")]
    Sepedi,
    #[language(short = "srp", shortest = "sr")]
    Serbian,
    #[language(short = "srr")]
    Serer,
    #[language(short = "sot")]
    Sesotho,
    #[language(short = "shn")]
    Shan,
    #[language(short = "sna", shortest = "sn")]
    Shona,
    #[language(short = "scn")]
    Sicilian,
    #[language(short = "xsd")]
    Sidetic,
    /// Group
    #[language(short = "sgn")]
    SignLanguages,
    #[language(short = "szl")]
    Silesian,
    #[language(short = "bts")]
    Simalungun,
    #[language(short = "snd", shortest = "sd")]
    Sindhi,
    #[language(short = "sin", shortest = "si")]
    Sinhala,
    #[language(short = "slk", old_short = "slo", shortest = "sk")]
    Slovak,
    #[language(short = "slv", shortest = "sl")]
    Slovenian,
    #[language(short = "sog")]
    Sogdian,
    #[language(short = "som", shortest = "so")]
    Somali,
    #[language(short = "srb")]
    Sora,
    #[language(short = "spa", shortest = "es")]
    Spanish,
    #[language(short = "sux")]
    Sumerian,
    #[language(short = "sun", shortest = "su")]
    Sundanese,
    #[language(short = "suz")]
    Sunuwar,
    #[language(short = "swh", shortest = "sw")]
    Swahili,
    #[language(short = "ssw", shortest = "ss")]
    Swati,
    #[language(short = "swe", shortest = "sv")]
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
    #[language(short = "tyj")]
    TaiYo,
    #[language(short = "tgk", shortest = "tg")]
    Tajik,
    #[language(short = "taq")]
    Tamasheq,
    #[language(short = "tzm")]
    TamazightCentralAtlas,
    #[language(short = "tam", shortest = "ta")]
    Tamil,
    /// Macro, unique script
    #[language(short = "nst")]
    Tangsa,
    #[language(short = "txg")]
    Tangut,
    #[language(short = "tat", shortest = "tt")]
    Tatar,
    #[language(short = "crh")]
    TatarCrimean,
    #[language(short = "ctd")]
    Tedim,
    #[language(short = "tel", shortest = "te")]
    Telugu,
    #[language(short = "tha", shortest = "th")]
    Thai,
    #[language(short = "bod", old_short = "tib", shortest = "bo")]
    Tibetan,
    #[language(short = "tir", shortest = "ti")]
    Tigrinya,
    #[language(short = "bbc")]
    TobaBatak,
    #[language(short = "tpi")]
    TokPisin,
    #[language(short = "txo")]
    Toto,
    #[language(short = "tso", shortest = "ts")]
    Tsonga,
    #[language(short = "tsn", shortest = "tn")]
    Tswana,
    #[language(short = "tcy")]
    Tulu,
    #[language(short = "tum")]
    Tumbuka,
    #[language(short = "tur", shortest = "tr")]
    Turkish,
    #[language(short = "tuk", shortest = "tk")]
    Turkmen,
    #[language(short = "uga")]
    Ugaritic,
    #[language(short = "ukr", shortest = "uk")]
    Ukrainian,
    #[language(short = "xum")]
    Umbrian,
    #[language(short = "umb")]
    Umbundu,
    #[language(short = "urd", shortest = "ur")]
    Urdu,
    #[language(short = "uig", shortest = "ug")]
    Uyghur,
    #[language(short = "uzn", shortest = "uz")]
    UzbekNorthern,
    #[language(short = "vai")]
    Vai,
    #[language(short = "vec")]
    Venetian,
    #[language(short = "vie", shortest = "vi")]
    Vietnamese,
    #[language(short = "nnp")]
    Wancho,
    #[language(short = "war")]
    Waray,
    #[language(short = "cym", old_short = "wel", shortest = "cy")]
    Welsh,
    #[language(short = "wol", shortest = "wo")]
    Wolof,
    #[language(short = "xho", shortest = "xh")]
    Xhosa,
    #[language(short = "ydd", shortest = "yi")]
    YiddishEastern,
    #[language(short = "yor", shortest = "yo")]
    Yoruba,
    #[language(short = "zag")]
    Zaghawa,
    #[language(short = "xzh")]
    Zhangzhung,
    #[language(short = "zul", shortest = "zu")]
    Zulu,
}

impl_try_from!(Language, u32, u32 i32 usize isize u64 i64 u128 i128);
impl_serde!(Language, "Language");
