use super::{script_char_to_slangs, Language, Script, UcdScript};
use ::std::fmt::Debug;
use alphabet_detector_macros::ScriptLanguage;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

/// Language + script.
/// Value-names not always represent a script used, so a "default" script can be changed.
/// Int representation is unstable and can be changed anytime.
/// Parts representation (const
/// [`into_parts`](enum.ScriptLanguage.html#method.into_parts)/[`from_parts`](enum.ScriptLanguage.html#method.from_parts))
/// or code representation (const
/// [`into_code`](enum.ScriptLanguage.html#method.into_code)/[`from_code`](enum.ScriptLanguage.html#method.from_code))
/// or string representation (const
/// [`into_str`](enum.ScriptLanguage.html#method.into_str)/[`from_str`](enum.ScriptLanguage.html#method.from_str))
/// are more stable.
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
    #[slang(script = HanTraditional, lang = ChineseCantonese)]
    ChineseCantoneseTraditional,
    #[slang(script = Bopomofo, lang = ChineseMandarin)]
    ChineseMandarinBopomofo,
    #[slang(script = HanSimplified, lang = ChineseMandarin)]
    ChineseMandarinSimplified,
    #[slang(script = HanTraditional, lang = ChineseMandarin)]
    ChineseMandarinTraditional,
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
    #[slang(script = Cyrillic)]
    Chuvash,
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
    Filipino,
    #[slang(script = Tagalog, lang = Filipino)]
    FilipinoTagalog,
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
    #[slang(script = Japanese)]
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
    KikongoKituba,
    #[slang(script = Latin)]
    Kikuyu,
    #[slang(script = Latin)]
    Kimbundu,
    #[slang(script = Latin)]
    Kinyarwanda,
    #[slang(script = OldPermic, lang = Komi)]
    KomiOldPermic,
    #[slang(script = Korean)]
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
    #[slang(script = Runic, lang = OldEnglish)]
    OldEnglishRunic,
    #[slang(script = Ogham, lang = OldIrish)]
    OldIrishOgham,
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

impl_try_from!(ScriptLanguage, u32, u32 i32 usize isize u64 i64 u128 i128);
impl_serde!(ScriptLanguage, "ScriptLanguage");

impl ScriptLanguage {
    /// Returns an iterator of all `ScriptLanguage`s
    #[inline(always)]
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    /// Returns all `ScriptLanguage`s supporting selected `UcdScript`
    #[inline]
    pub fn all_with_script(script: UcdScript) -> &'static [Self] {
        script_char_to_slangs(script, char::default())
    }

    #[inline(always)]
    pub fn transmute_from_usize(v: usize) -> Self {
        debug_assert!(v < Self::COUNT);
        unsafe { ::core::mem::transmute::<usize, Self>(v) }
    }
}

/* impl Display for ScriptLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
} */

impl From<ScriptLanguage> for UcdScript {
    #[inline]
    fn from(sl: ScriptLanguage) -> Self {
        UcdScript::from(Script::from(sl))
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
