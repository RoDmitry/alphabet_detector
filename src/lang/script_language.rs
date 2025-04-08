use super::{script_char_to_slangs, Language, Script};
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use ::core::fmt;
use ::std::fmt::Debug;
use ahash::AHashSet;
use alphabet_match_macro::ScriptLanguage;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

const ENUM_NAME: &str = "ScriptLanguage";

/// Enum value-names or int representation is unstable and can be changed anytime
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumCountMacro,
    EnumIter,
    ScriptLanguage,
)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
#[non_exhaustive]
#[repr(usize)]
pub enum ScriptLanguage {
    #[slang(script = Latin)]
    Acehnese,
    #[slang(script = Arabic, lang = Acehnese)]
    AcehneseJawi,
    #[slang(script = Latin)]
    Afrikaans,
    #[slang(script = Ahom)]
    Ahom,
    #[slang(script = Latin)]
    Akan,
    #[slang(script = Cuneiform)]
    Akkadian,
    /// Historical
    #[slang(script = Elbasan, lang = Albanian)]
    AlbanianElbasan,
    /// Historical
    #[slang(script = Todhri, lang = Albanian)]
    AlbanianTodhri,
    #[slang(script = Latin)]
    AlbanianTosk,
    #[slang(script = Vithkuqi, lang = AlbanianTosk)]
    AlbanianToskVithkuqi,
    #[slang(script = Ethiopic)]
    Amharic,
    #[slang(script = Cypriot)]
    AncientGreek,
    #[slang(script = OldNorthArabian)]
    AncientNorthArabian,
    #[slang(script = OldSouthArabian)]
    AncientSouthArabian,
    #[slang(script = Batak)]
    Angkola,
    #[slang(script = Arabic)]
    Arabic,
    #[slang(script = Arabic)]
    ArabicEgyptian,
    #[slang(script = Arabic)]
    ArabicMesopotamian,
    #[slang(script = Arabic)]
    ArabicMoroccan,
    #[slang(script = Arabic)]
    ArabicNajdi,
    #[slang(script = Arabic)]
    ArabicNorthLevantine,
    #[slang(script = Arabic)]
    ArabicSouthernYemeni,
    #[slang(script = Arabic)]
    ArabicSouthLevantine,
    #[slang(script = Arabic)]
    ArabicTunisian,
    #[slang(script = Elymaic)]
    AramaicElymaic,
    #[slang(script = Hatran)]
    AramaicHatran,
    #[slang(script = ImperialAramaic)]
    AramaicImperial,
    #[slang(script = Mandaic)]
    AramaicMandaic,
    #[slang(script = Nabataean)]
    AramaicNabataean,
    #[slang(script = Palmyrene)]
    AramaicPalmyrene,
    #[slang(script = Samaritan)]
    AramaicSamaritan,
    #[slang(script = Syriac)]
    AramaicSyriac,
    #[slang(script = Armenian)]
    Armenian,
    #[slang(script = Bengali)]
    Assamese,
    #[slang(script = Latin)]
    Asturian,
    #[slang(script = Avestan)]
    Avestan,
    #[slang(script = Devanagari)]
    Awadhi,
    #[slang(script = Latin)]
    AymaraCentral,
    #[slang(script = Latin)]
    AzerbaijaniNorth,
    #[slang(script = Arabic)]
    AzerbaijaniSouth,
    #[slang(script = Latin)]
    Balinese,
    #[slang(script = Balinese, lang = Balinese)]
    BalineseBalinese,
    #[slang(script = Latin)]
    Bambara,
    #[slang(script = Bamum)]
    Bamum,
    #[slang(script = Latin)]
    Banjar,
    #[slang(script = Arabic, lang = Banjar)]
    BanjarJawi,
    #[slang(script = KiratRai)]
    Bantawa,
    #[slang(script = Cyrillic)]
    Bashkir,
    #[slang(script = Latin)]
    Basque,
    #[slang(script = BassaVah)]
    Bassa,
    #[slang(script = Cyrillic)]
    Belarusian,
    #[slang(script = Latin)]
    Bemba,
    #[slang(script = Bengali)]
    Bengali,
    #[slang(script = Devanagari)]
    Bhojpuri,
    #[slang(script = Kaithi, lang = Bhojpuri)]
    BhojpuriKaithi,
    #[slang(script = OlOnal)]
    Bhumij,
    #[slang(script = Bengali)]
    BishnupriyaManipuri,
    #[slang(script = Latin)]
    Bosnian,
    /// any language adapted to Braille
    #[slang(script = Braille)]
    Braille,
    #[slang(script = Latin)]
    Buginese,
    #[slang(script = Buginese, lang = Buginese)]
    BugineseBuginese,
    #[slang(script = Buhid)]
    Buhid,
    #[slang(script = Cyrillic)]
    Bulgarian,
    #[slang(script = Myanmar)]
    Burmese,
    #[slang(script = Carian)]
    Carian,
    #[slang(script = Latin)]
    Catalan,
    #[slang(script = CaucasianAlbanian)]
    CaucasianAlbanian,
    #[slang(script = Latin)]
    Cebuano,
    #[slang(script = Chakma)]
    Chakma,
    #[slang(script = Cham)]
    ChamEastern,
    #[slang(script = Cham)]
    ChamWestern,
    #[slang(script = Cherokee)]
    Cherokee,
    #[slang(script = Devanagari)]
    Chhattisgarhi,
    #[slang(script = "Hant", lang = ChineseCantonese)]
    ChineseCantoneseTraditional,
    #[slang(script = Bopomofo, lang = ChineseMandarin)]
    ChineseMandarinBopomofo,
    #[slang(script = "Hans", lang = Chinese)]
    ChineseSimplified,
    #[slang(script = "Hant", lang = Chinese)]
    ChineseTraditional,
    #[slang(script = Nushu)]
    ChineseTuhua,
    #[slang(script = Latin)]
    Chokwe,
    #[slang(script = Chorasmian)]
    Chorasmian,
    #[slang(script = Cyrillic, lang = ChurchSlavonic)]
    ChurchSlavonicOld,
    #[slang(script = Glagolitic, lang = ChurchSlavonic)]
    ChurchSlavonicOldGlagolitic,
    #[slang(script = Coptic)]
    Coptic,
    #[slang(script = CanadianAboriginal)]
    Cree,
    #[slang(script = Latin)]
    CreoleHaitian,
    #[slang(script = Latin)]
    Croatian,
    #[slang(script = Latin)]
    Czech,
    #[slang(script = Latin)]
    Danish,
    #[slang(script = Thaana)]
    Dhivehi,
    /// Historical
    #[slang(script = DivesAkuru, lang = Dhivehi)]
    DhivehiDivesAkuru,
    #[slang(script = Latin)]
    Dholuo,
    #[slang(script = Latin)]
    DinkaSouthwestern,
    #[slang(script = Devanagari)]
    Dogri,
    #[slang(script = Dogra, lang = Dogri)]
    DogriDogra,
    #[slang(script = Arabic, lang = Dogri)]
    DogriPersoArabic,
    #[slang(script = Takri, lang = Dogri)]
    DogriTakri,
    #[slang(script = Latin)]
    Dutch,
    #[slang(script = Latin)]
    Dyula,
    #[slang(script = Tibetan)]
    Dzongkha,
    #[slang(script = EgyptianHieroglyphs)]
    EgyptianHieroglyphs,
    #[slang(script = Latin)]
    English,
    #[slang(script = Deseret, lang = English)]
    EnglishDeseret,
    /// shorthand systems for English
    #[slang(script = Duployan, lang = English)]
    EnglishDuployan,
    #[slang(script = Shavian, lang = English)]
    EnglishShavian,
    #[slang(script = Latin)]
    Esperanto,
    #[slang(script = Latin)]
    Estonian,
    #[slang(script = OldItalic)]
    Etruscan,
    #[slang(script = Latin)]
    Ewe,
    #[slang(script = Latin)]
    Faroese,
    #[slang(script = Latin)]
    Fijian,
    #[slang(script = Latin)]
    Finnish,
    #[slang(script = Latin)]
    Fon,
    #[slang(script = Latin)]
    French,
    #[slang(script = Duployan, lang = French)]
    FrenchDuployan,
    #[slang(script = Latin)]
    Friulian,
    #[slang(script = Adlam)]
    Fulani,
    #[slang(script = Latin)]
    FulfuldeNigerian,
    #[slang(script = Latin)]
    GaelicScottish,
    #[slang(script = Latin)]
    Galician,
    #[slang(script = Latin)]
    Ganda,
    #[slang(script = Kharoshthi)]
    Gandhari,
    #[slang(script = Ethiopic)]
    Geez,
    #[slang(script = Georgian)]
    Georgian,
    #[slang(script = Latin)]
    German,
    #[slang(script = GunjalaGondi, lang = Gondi)]
    GondiGunjala,
    #[slang(script = MasaramGondi, lang = Gondi)]
    GondiMasaram,
    #[slang(script = Gothic)]
    Gothic,
    #[slang(script = Greek)]
    Greek,
    #[slang(script = Latin)]
    Guarani,
    #[slang(script = Gujarati)]
    Gujarati,
    #[slang(script = GurungKhema, lang = Gurung)]
    GurungKhema,
    #[slang(script = Hanunoo)]
    Hanunoo,
    #[slang(script = Latin)]
    Hausa,
    #[slang(script = Latin)]
    Hawaiian,
    #[slang(script = Hebrew)]
    Hebrew,
    #[slang(script = Samaritan)]
    HebrewSamaritan,
    #[slang(script = Devanagari)]
    Hindi,
    #[slang(script = Kaithi, lang = Hindi)]
    HindiKaithi,
    #[slang(script = Mahajani, lang = Hindi)]
    HindiMahajani,
    #[slang(script = Cuneiform)]
    Hittite,
    #[slang(script = Miao, lang = Hmong)]
    HmongMiao,
    #[slang(script = NyiakengPuachueHmong, lang = Hmong)]
    HmongNyiakengPuachue,
    #[slang(script = PahawhHmong, lang = Hmong)]
    HmongPahawh,
    #[slang(script = WarangCiti, lang = Ho)]
    HoWarangCiti,
    #[slang(script = Latin)]
    Hungarian,
    #[slang(script = OldHungarian, lang = Hungarian)]
    HungarianOld,
    #[slang(script = Latin)]
    Icelandic,
    #[slang(script = Latin)]
    Igbo,
    #[slang(script = Latin)]
    Ilocano,
    #[slang(script = Latin)]
    Indonesian,
    #[slang(script = CanadianAboriginal)]
    Inuktitut,
    #[slang(script = Latin)]
    Irish,
    #[slang(script = Latin)]
    Italian,
    #[slang(script = "Jpan")]
    Japanese,
    #[slang(script = Latin)]
    Javanese,
    #[slang(script = Javanese, lang = Javanese)]
    JavaneseJavanese,
    #[slang(script = Latin)]
    Jingpho,
    #[slang(script = Latin)]
    Kabiye,
    #[slang(script = Latin)]
    Kabuverdianu,
    #[slang(script = Latin)]
    Kabyle,
    #[slang(script = Latin)]
    Kamba,
    #[slang(script = Kannada)]
    Kannada,
    #[slang(script = TuluTigalari, lang = Kannada)]
    KannadaTuluTigalari,
    #[slang(script = Latin)]
    KanuriCentral,
    #[slang(script = Arabic, lang = KanuriCentral)]
    KanuriCentralAjami,
    #[slang(script = Batak)]
    Karo,
    #[slang(script = Arabic)]
    Kashmiri,
    #[slang(script = Devanagari, lang = Kashmiri)]
    KashmiriDevanagari,
    #[slang(script = Sharada, lang = Kashmiri)]
    KashmiriSharada,
    #[slang(script = Takri, lang = Kashmiri)]
    KashmiriTakri,
    #[slang(script = KayahLi)]
    KayahEastern,
    #[slang(script = KayahLi)]
    KayahWestern,
    #[slang(script = Cyrillic)]
    Kazakh,
    #[slang(script = KhitanSmallScript)]
    Khitan,
    #[slang(script = Khmer)]
    Khmer,
    #[slang(script = Latin)]
    Kikongo,
    #[slang(script = Latin)]
    Kikuyu,
    #[slang(script = Latin)]
    Kimbundu,
    #[slang(script = Latin)]
    Kinyarwanda,
    #[slang(script = OldPermic, lang = Komi)]
    KomiOldPermic,
    #[slang(script = "Kore")]
    Korean,
    #[slang(script = Arabic)]
    KurdishCentral,
    #[slang(script = Latin)]
    KurdishNorthern,
    #[slang(script = Yezidi, lang = KurdishNorthern)]
    KurdishNorthernYezidi,
    #[slang(script = Arabic)]
    KurdishSouthern,
    #[slang(script = Khojki, lang = Kutchi)]
    KutchiKhojki,
    #[slang(script = Cyrillic)]
    Kyrgyz,
    #[slang(script = Lao)]
    Lao,
    #[slang(script = TaiTham, lang = Lao)]
    LaoTaiTham,
    #[slang(script = Latin)]
    Latgalian,
    #[slang(script = Latin)]
    Latin,
    #[slang(script = Latin)]
    Latvian,
    #[slang(script = Lepcha)]
    Lepcha,
    #[slang(script = Latin)]
    Ligurian,
    #[slang(script = Limbu)]
    Limbu,
    #[slang(script = Latin)]
    Limburgish,
    #[slang(script = Latin)]
    Lingala,
    #[slang(script = Lisu)]
    Lisu,
    #[slang(script = Latin)]
    Lithuanian,
    #[slang(script = Yi)]
    Loloish,
    #[slang(script = Latin)]
    Lombard,
    #[slang(script = Latin)]
    LubaKasai,
    #[slang(script = Cuneiform)]
    LuwianCuneiform,
    #[slang(script = AnatolianHieroglyphs)]
    LuwianHieroglyphic,
    #[slang(script = Latin)]
    Luxembourgish,
    #[slang(script = Lycian)]
    Lycian,
    #[slang(script = Lydian)]
    Lydian,
    #[slang(script = Cyrillic)]
    Macedonian,
    #[slang(script = Devanagari)]
    Magahi,
    #[slang(script = Kaithi, lang = Magahi)]
    MagahiKaithi,
    #[slang(script = Devanagari)]
    Maithili,
    #[slang(script = Kaithi, lang = Maithili)]
    MaithiliKaithi,
    #[slang(script = Tirhuta, lang = Maithili)]
    MaithiliTirhuta,
    #[slang(script = Buginese, lang = Makassarese)]
    MakassareseBuginese,
    /// Historical
    #[slang(script = Makasar, lang = Makassarese)]
    MakassareseMakasar,
    #[slang(script = Latin)]
    Malay,
    #[slang(script = Malayalam)]
    Malayalam,
    #[slang(script = Latin)]
    MalgasyPlateau,
    #[slang(script = Latin)]
    Maltese,
    #[slang(script = Latin)]
    Mandailing,
    /// Historical
    #[slang(script = Batak, lang = Mandailing)]
    MandailingBatak,
    #[slang(script = Nko)]
    Manding,
    #[slang(script = Latin)]
    Maori,
    #[slang(script = Devanagari)]
    Marathi,
    #[slang(script = Brahmi, lang = Marathi)]
    MarathiBrahmi,
    #[slang(script = Modi, lang = Marathi)]
    MarathiModi,
    #[slang(script = Mahajani)]
    Marwari,
    #[slang(script = Medefaidrin)]
    Medefaidrin,
    #[slang(script = Bengali, lang = Meitei)]
    MeiteiBengali,
    #[slang(script = MeeteiMayek, lang = Meitei)]
    MeiteiMeeteiMayek,
    #[slang(script = MendeKikakui)]
    Mende,
    #[slang(script = MeroiticCursive, lang = Meroitic)]
    MeroiticCursive,
    #[slang(script = MeroiticHieroglyphs, lang = Meroitic)]
    MeroiticHieroglyphs,
    #[slang(script = Manichaean)]
    MiddlePersianManichaean,
    #[slang(script = InscriptionalPahlavi, lang = MiddlePersianPahlavi)]
    MiddlePersianPahlaviInscriptional,
    #[slang(script = PsalterPahlavi, lang = MiddlePersianPahlavi)]
    MiddlePersianPahlaviPsalter,
    #[slang(script = Latin)]
    Minangkabau,
    #[slang(script = CyproMinoan)]
    Minoan,
    #[slang(script = LinearA)]
    MinoanLinearA,
    #[slang(script = Latin)]
    Mizo,
    #[slang(script = Cyrillic)]
    MongolianHalh,
    #[slang(script = Mongolian, lang = MongolianHalh)]
    MongolianHalhMongolian,
    #[slang(script = PhagsPa, lang = MongolianHalh)]
    MongolianHalhPhagsPa,
    #[slang(script = Soyombo, lang = MongolianHalh)]
    MongolianHalhSoyombo,
    #[slang(script = ZanabazarSquare, lang = MongolianHalh)]
    MongolianHalhZanabazarSquare,
    #[slang(script = Latin)]
    Mossi,
    #[slang(script = Mro)]
    Mro,
    #[slang(script = NagMundari)]
    Mundari,
    #[slang(script = LinearB)]
    MycenaeanGreek,
    #[slang(script = Devanagari)]
    Nepali,
    #[slang(script = Newa)]
    Newar,
    #[slang(script = TaiTham)]
    NorthernThai,
    #[slang(script = Latin)]
    NorwegianBokmal,
    #[slang(script = Latin)]
    NorwegianNynorsk,
    #[slang(script = Latin)]
    Nuer,
    #[slang(script = Latin)]
    Nyanja,
    #[slang(script = Latin)]
    Occitan,
    #[slang(script = Oriya)]
    Odia,
    #[slang(script = CanadianAboriginal)]
    Ojibwe,
    #[slang(script = Runic)]
    OldEnglish,
    #[slang(script = Ogham)]
    OldIrish,
    #[slang(script = Kawi)]
    OldJavanese,
    #[slang(script = Runic)]
    OldNorse,
    #[slang(script = OldPersian)]
    OldPersian,
    #[slang(script = OldTurkic)]
    OldTurkic,
    #[slang(script = OldUyghur)]
    OldUyghur,
    #[slang(script = Latin)]
    OromoSouthern,
    #[slang(script = Latin)]
    OromoWestCentral,
    #[slang(script = Osage)]
    Osage,
    #[slang(script = OldItalic)]
    Oscan,
    #[slang(script = Batak)]
    Pakpak,
    #[slang(script = Latin)]
    Pangasinan,
    #[slang(script = Latin)]
    Papiamento,
    #[slang(script = InscriptionalParthian)]
    Parthian,
    #[slang(script = Arabic)]
    PashtoSouthern,
    #[slang(script = Arabic)]
    PersianDari,
    #[slang(script = Arabic)]
    PersianWestern,
    #[slang(script = Phoenician)]
    Phoenician,
    #[slang(script = Latin)]
    Polish,
    #[slang(script = Latin)]
    Portuguese,
    #[slang(script = Brahmi)]
    Prakrit,
    #[slang(script = Latin)]
    Pular,
    #[slang(script = Adlam, lang = Pular)]
    PularAdlam,
    #[slang(script = Gurmukhi)]
    PunjabiEastern,
    #[slang(script = Mahajani, lang = PunjabiEastern)]
    PunjabiEasternMahajani,
    #[slang(script = Arabic, lang = PunjabiEastern)]
    PunjabiEasternShahmukhi,
    #[slang(script = Latin)]
    QuechuaAyacucho,
    /// Historical
    #[slang(script = Rejang, lang = Rejang)]
    RejangRejang,
    #[slang(script = HanifiRohingya)]
    Rohingya,
    #[slang(script = Latin)]
    Romanian,
    #[slang(script = Latin)]
    Rundi,
    #[slang(script = Cyrillic)]
    Russian,
    #[slang(script = Latin)]
    Samoan,
    #[slang(script = Latin)]
    Sango,
    #[slang(script = Devanagari)]
    Sanskrit,
    #[slang(script = Bhaiksuki, lang = Sanskrit)]
    SanskritBhaiksuki,
    #[slang(script = Brahmi, lang = Sanskrit)]
    SanskritBrahmi,
    #[slang(script = Grantha, lang = Sanskrit)]
    SanskritGrantha,
    #[slang(script = Kawi, lang = Sanskrit)]
    SanskritKawi,
    #[slang(script = Nandinagari, lang = Sanskrit)]
    SanskritNandinagari,
    #[slang(script = Sharada, lang = Sanskrit)]
    SanskritSharada,
    #[slang(script = Siddham, lang = Sanskrit)]
    SanskritSiddham,
    #[slang(script = Soyombo, lang = Sanskrit)]
    SanskritSoyombo,
    #[slang(script = TuluTigalari, lang = Sanskrit)]
    SanskritTuluTigalari,
    #[slang(script = ZanabazarSquare, lang = Sanskrit)]
    SanskritZanabazarSquare,
    #[slang(script = OlChiki)]
    Santali,
    #[slang(script = Arabic)]
    Saraiki,
    #[slang(script = Multani, lang = Saraiki)]
    SaraikiMultani,
    #[slang(script = Latin)]
    Sardinian,
    #[slang(script = Devanagari, lang = Saurashtra)]
    SaurashtraDevanagari,
    #[slang(script = Saurashtra, lang = Saurashtra)]
    SaurashtraSaurashtra,
    #[slang(script = Latin)]
    Sepedi,
    #[slang(script = Cyrillic)]
    Serbian,
    #[slang(script = Latin)]
    Sesotho,
    #[slang(script = Myanmar)]
    Shan,
    #[slang(script = Latin)]
    Shona,
    #[slang(script = Latin)]
    Sicilian,
    #[slang(script = SignWriting)]
    SignLanguages,
    #[slang(script = Latin)]
    Silesian,
    #[slang(script = Batak)]
    Simalungun,
    #[slang(script = Arabic)]
    Sindhi,
    #[slang(script = Devanagari, lang = Sindhi)]
    SindhiDevanagari,
    #[slang(script = Khojki, lang = Sindhi)]
    SindhiKhojki,
    #[slang(script = Khudawadi, lang = Sindhi)]
    SindhiKhudawadi,
    #[slang(script = Sinhala)]
    Sinhala,
    #[slang(script = Latin)]
    Slovak,
    #[slang(script = Latin)]
    Slovene,
    #[slang(script = Sogdian)]
    Sogdian,
    #[slang(script = Manichaean, lang = Sogdian)]
    SogdianManichaean,
    #[slang(script = OldSogdian, lang = Sogdian)]
    SogdianOld,
    #[slang(script = Latin)]
    Somali,
    #[slang(script = Osmanya, lang = Somali)]
    SomaliOsmanya,
    #[slang(script = SoraSompeng)]
    Sora,
    #[slang(script = Latin)]
    Spanish,
    #[slang(script = Cuneiform)]
    Sumerian,
    #[slang(script = Latin)]
    Sundanese,
    #[slang(script = Sundanese, lang = Sundanese)]
    SundaneseSundanese,
    #[slang(script = Sunuwar)]
    Sunuwar,
    #[slang(script = Latin)]
    Swahili,
    #[slang(script = Latin)]
    Swati,
    #[slang(script = Latin)]
    Swedish,
    #[slang(script = SylotiNagri)]
    Sylheti,
    #[slang(script = Latin)]
    Tagalog,
    #[slang(script = Tagalog, lang = Tagalog)]
    TagalogTagalog,
    #[slang(script = Tagbanwa)]
    Tagbanwa,
    #[slang(script = TaiViet)]
    TaiDam,
    #[slang(script = TaiViet)]
    TaiDon,
    #[slang(script = TaiTham)]
    TaiLue,
    #[slang(script = NewTaiLue, lang = TaiLue)]
    TaiLueNew,
    #[slang(script = TaiLe)]
    TaiNuea,
    #[slang(script = Cyrillic)]
    Tajik,
    #[slang(script = Latin, lang = Tamasheq)]
    TamasheqLatin,
    #[slang(script = Tifinagh, lang = Tamasheq)]
    TamasheqTifinagh,
    #[slang(script = Tifinagh)]
    TamazightCentralAtlas,
    #[slang(script = Tamil)]
    Tamil,
    #[slang(script = Grantha, lang = Tamil)]
    TamilGrantha,
    #[slang(script = Tangsa)]
    Tangsa,
    #[slang(script = Tangut)]
    Tangut,
    #[slang(script = Cyrillic)]
    Tatar,
    #[slang(script = Latin)]
    TatarCrimean,
    #[slang(script = PauCinHau)]
    Tedim,
    #[slang(script = Telugu)]
    Telugu,
    #[slang(script = Thai)]
    Thai,
    #[slang(script = Tibetan)]
    Tibetan,
    #[slang(script = PhagsPa, lang = Tibetan)]
    TibetanPhagsPa,
    #[slang(script = Soyombo, lang = Tibetan)]
    TibetanSoyombo,
    #[slang(script = ZanabazarSquare, lang = Tibetan)]
    TibetanZanabazarSquare,
    #[slang(script = Ethiopic)]
    Tigrinya,
    #[slang(script = Batak)]
    TobaBatak,
    #[slang(script = Latin)]
    TokPisin,
    #[slang(script = Toto)]
    Toto,
    #[slang(script = Latin)]
    Tsonga,
    #[slang(script = Latin)]
    Tswana,
    #[slang(script = Kannada, lang = Tulu)]
    Tulu,
    #[slang(script = TuluTigalari, lang = Tulu)]
    TuluTigalari,
    #[slang(script = Latin)]
    Tumbuka,
    #[slang(script = Latin)]
    Turkish,
    #[slang(script = Latin)]
    Turkmen,
    #[slang(script = Latin)]
    Twi,
    #[slang(script = Ugaritic)]
    Ugaritic,
    #[slang(script = Cyrillic)]
    Ukrainian,
    #[slang(script = OldItalic)]
    Umbrian,
    #[slang(script = Latin)]
    Umbundu,
    #[slang(script = Arabic)]
    Urdu,
    #[slang(script = Arabic)]
    Uyghur,
    #[slang(script = Latin)]
    UzbekNorthern,
    #[slang(script = Vai)]
    Vai,
    #[slang(script = Latin)]
    Venetian,
    #[slang(script = Latin)]
    Vietnamese,
    #[slang(script = Devanagari)]
    Wancho,
    #[slang(script = Wancho, lang = Wancho)]
    WanchoWancho,
    #[slang(script = Latin)]
    Waray,
    #[slang(script = Latin)]
    Welsh,
    #[slang(script = Latin)]
    Wolof,
    #[slang(script = Garay, lang = Wolof)]
    WolofGaray,
    #[slang(script = Latin)]
    Xhosa,
    #[slang(script = Hebrew)]
    YiddishEastern,
    #[slang(script = Latin)]
    Yoruba,
    #[slang(script = Marchen)]
    Zhangzhung,
    #[slang(script = Latin)]
    Zulu,
}

impl ScriptLanguage {
    /// Returns an iterator of all languages
    #[inline]
    pub fn all() -> impl Iterator<Item = ScriptLanguage> {
        ScriptLanguage::iter()
    }

    /// Returns all languages supporting selected `Script`
    #[inline]
    pub fn all_with_script(script: Script) -> &'static [ScriptLanguage] {
        script_char_to_slangs(script, char::default())
    }

    /// Returns a set of all supported spoken languages.
    #[deprecated]
    pub fn all_spoken_ones() -> AHashSet<ScriptLanguage> {
        ScriptLanguage::iter()
            .filter(|it| it != &ScriptLanguage::Latin)
            .collect()
    }

    /// Returns the language associated with the ISO 639-1 code
    /// passed to this method.
    #[deprecated]
    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> ScriptLanguage {
        ScriptLanguage::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    /// Returns the language associated with the ISO 639-3 code
    /// passed to this method.
    #[deprecated]
    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> ScriptLanguage {
        ScriptLanguage::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    /// Returns the ISO 639-1 code of this language.
    #[deprecated]
    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        use ScriptLanguage::*;
        match self {
            Afrikaans => IsoCode639_1::AF,
            AlbanianTosk => IsoCode639_1::SQ, // invalid
            Arabic => IsoCode639_1::AR,
            Armenian => IsoCode639_1::HY,
            AzerbaijaniNorth => IsoCode639_1::AZ,
            Basque => IsoCode639_1::EU,
            Belarusian => IsoCode639_1::BE,
            Bengali => IsoCode639_1::BN,
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
            NorwegianBokmal => IsoCode639_1::NB,
            NorwegianNynorsk => IsoCode639_1::NN,
            PersianWestern => IsoCode639_1::FA,
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
        use ScriptLanguage::*;
        match self {
            Afrikaans => IsoCode639_3::AFR,
            AlbanianTosk => IsoCode639_3::SQI,
            Arabic => IsoCode639_3::ARA,
            Armenian => IsoCode639_3::HYE,
            AzerbaijaniNorth => IsoCode639_3::AZE,
            Basque => IsoCode639_3::EUS,
            Belarusian => IsoCode639_3::BEL,
            Bengali => IsoCode639_3::BEN,
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
            NorwegianBokmal => IsoCode639_3::NOB,
            NorwegianNynorsk => IsoCode639_3::NNO,
            PersianWestern => IsoCode639_3::FAS,
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

    #[inline(always)]
    pub fn from_usize_unchecked(v: usize) -> Self {
        debug_assert!(v < Self::COUNT);
        unsafe { ::core::mem::transmute::<usize, Self>(v) }
    }
}

macro_rules! impl_try_from {
    ($($t:ty)*) => {$(
        impl TryFrom<$t> for ScriptLanguage {
            type Error = &'static str;

            #[inline]
            fn try_from(v: $t) -> Result<Self, Self::Error> {
                if v < Self::COUNT as $t {
                    Ok(unsafe { ::core::mem::transmute::<usize, Self>(v as usize) })
                } else {
                    Err(concat_const::concat!(
                        "value > ",
                        concat_const::int!(ScriptLanguage::COUNT as i128)
                    ))
                }
            }
        }
    )*};
}

impl_try_from!(u16 u32 usize u64 u128);

/* impl Display for ScriptLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
} */

impl serde::Serialize for ScriptLanguage {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serializer::serialize_unit_variant(
            serializer,
            ENUM_NAME,
            *self as u32,
            self.into_str(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for ScriptLanguage {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[doc(hidden)]
        struct ScriptLanguageValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ScriptLanguageValueVisitor {
            type Value = ScriptLanguage;

            fn expecting(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Formatter::write_str(__formatter, "variant identifier")
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::try_from(v).map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(v),
                        &concat_const::concat!(
                            "variant index 0 <= i < ",
                            concat_const::int!(ScriptLanguage::COUNT as i128)
                        ),
                    )
                })
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::from_str(v)
                    .ok_or_else(|| serde::de::Error::unknown_variant(v, Self::Value::VARIANTS))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::from_bytes(v).ok_or_else(|| {
                    serde::de::Error::unknown_variant(
                        &serde::__private::from_utf8_lossy(v),
                        Self::Value::VARIANTS,
                    )
                })
            }
        }

        serde::Deserializer::deserialize_identifier(deserializer, ScriptLanguageValueVisitor)
    }
}

// const SCRIPT_LANGUAGE_COUNT: usize = ::core::mem::variant_count::<ScriptLanguage>();
pub type ScriptLanguageArr<T> = [T; ScriptLanguage::COUNT];

#[inline(always)]
pub fn slang_arr_default<T: Default + Copy>() -> ScriptLanguageArr<T> {
    [Default::default(); ScriptLanguage::COUNT]
}

#[inline]
pub fn slang_arr_default_nc<T: Default>() -> ScriptLanguageArr<T> {
    ::core::array::from_fn(|_| Default::default())
}
