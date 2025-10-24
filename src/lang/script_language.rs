use super::{script_char_to_slangs, Language, Script, UcdScript};
use ::std::fmt::Debug;
use alphabet_detector_macros::ScriptLanguage;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

/// Language + script. Ordered by total speakers.
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
    // Latin
    #[slang(script = Latin)]
    English,
    #[slang(script = Latin)]
    Spanish, // 558
    #[slang(script = Latin)]
    French, // 310
    #[slang(script = Latin)]
    Indonesian, // 270
    #[slang(script = Latin)]
    Portuguese, // 267
    #[slang(script = Latin)]
    German, // 180
    #[slang(script = Latin)]
    NigerianPidgin, // 120
    #[slang(script = Latin)]
    Swahili, // 98
    #[slang(script = Latin)]
    Vietnamese, // 97
    #[slang(script = Latin)]
    Hausa, // 94
    #[slang(script = Latin)]
    Turkish, // 90
    #[slang(script = Latin)]
    Filipino, // 83
    #[slang(script = Latin)]
    Javanese, // 68
    #[slang(script = Latin)]
    Italian, // 68
    #[slang(script = Latin)]
    UzbekNorthern, // 58
    #[slang(script = Latin)]
    Yoruba, // 50
    #[slang(script = Latin)]
    Igbo, // 47
    #[slang(script = Latin)]
    Polish, // 43
    #[slang(script = Latin)]
    Lingala, // 41
    #[slang(script = Latin)]
    Sundanese, // 32 2015
    #[slang(script = Latin)]
    Malay, // 32?
    #[slang(script = Latin)]
    Zulu, // 28 old
    #[slang(script = Latin)]
    Dutch, // 30
    #[slang(script = Latin)]
    Somali, // 24
    #[slang(script = Latin)]
    Romanian, // 22
    #[slang(script = Latin)]
    Xhosa, // 19 old
    #[slang(script = Latin)]
    Cebuano, // 20
    #[slang(script = Latin)]
    Afrikaans, // 17.5 old
    #[slang(script = Latin)]
    KikongoKituba, // 18.2
    #[slang(script = Latin)]
    Wolof, // 18
    #[slang(script = Latin)]
    KurdishNorthern, // 17
    #[slang(script = Latin)]
    FulfuldeNigerian, // 17
    #[slang(script = Latin)]
    Sepedi, // 15.3
    #[slang(script = Latin)]
    Kinyarwanda, // 15
    #[slang(script = Latin)]
    LubaKasai, // 15
    #[slang(script = Latin)]
    Bambara, // 14 2012
    #[slang(script = Latin)]
    Sesotho, // 13.5 old
    #[slang(script = Latin)]
    Shona, // 14
    #[slang(script = Latin)]
    Hungarian, // 14
    #[slang(script = Latin)]
    Czech, // 12 old
    #[slang(script = Latin)]
    Dyula, // 13
    #[slang(script = Latin)]
    Rundi, // 13
    #[slang(script = Latin)]
    Swedish, // 13
    #[slang(script = Latin)]
    CreoleHaitian, // 13
    #[slang(script = Latin)]
    Ganda, // 11 old
    #[slang(script = Latin)]
    Mossi, // 12
    #[slang(script = Latin)]
    Banjar, // 10.6 old
    #[slang(script = Latin)]
    Ilocano, // 11
    #[slang(script = Latin)]
    AzerbaijaniNorth, // 10?
    #[slang(script = Latin)]
    MalagasyPlateau, // ??
    #[slang(script = Latin)]
    OromoSouthern, // ??
    #[slang(script = Latin)]
    OromoWestCentral, // ??
    #[slang(script = Latin)]
    Catalan, // 9.2
    #[slang(script = Latin)]
    KanuriCentral, // 9.1
    #[slang(script = Latin)]
    Tswana, // 8? old
    #[slang(script = Latin)]
    Turkmen, // 7.8
    #[slang(script = Latin)]
    Slovak, // 7.1 old
    #[slang(script = Latin)]
    Tsonga, // 7.1 old
    #[slang(script = Latin)]
    Tumbuka, // 7.1
    #[slang(script = Latin)]
    Nyanja, // 7 2007
    #[slang(script = Latin)]
    AkanTwi, // ~7.1 2013
    #[slang(script = Latin)]
    Umbundu, // 7
    #[slang(script = Latin)]
    Kikuyu, // 6.6 old
    #[slang(script = Latin)]
    Croatian, // 6.5
    #[slang(script = Latin)]
    GuaraniParaguayan, // 6.5
    #[slang(script = Latin)]
    FulaPulaar, // 6.3 2022
    #[slang(script = Latin)]
    Minangkabau, // 5.5 old
    #[slang(script = Latin)]
    Danish, // 6
    #[slang(script = Latin)]
    Kamba, // 5.6
    #[slang(script = Latin)]
    Sicilian, // 4.7 old
    #[slang(script = Latin)]
    Finnish, // 5
    #[slang(script = Latin)]
    Ewe, // 5
    #[slang(script = Latin)]
    FulaPular, // 4.8
    #[slang(script = Latin)]
    Swati, // 4.7 2013
    #[slang(script = Latin)]
    Dholuo, // 4.2 old
    #[slang(script = Latin)]
    Bemba, // 4.1 old
    #[slang(script = Latin)]
    TokPisin, // 4.1 old
    #[slang(script = Latin)]
    Buginese, // 4 2015
    #[slang(script = Latin)]
    NorwegianBokmal, // 4.3 old with Nynorsk
    #[slang(script = Latin)]
    NorwegianNynorsk,
    #[slang(script = Latin)]
    Venetian, // 3.9 2002
    #[slang(script = Latin)]
    Lombard, // 3.8 old
    #[slang(script = Latin)]
    Waray, // 3.6 old
    #[slang(script = Latin)]
    Balinese, // 3.3 2000
    #[slang(script = Latin)]
    Kabyle, // 3 old
    #[slang(script = Latin)]
    Lithuanian, // 3 2012
    #[slang(script = Latin)]
    Acehnese, // 2.8 2010
    #[slang(script = Latin)]
    AkanFante, // 2.8 2013
    #[slang(script = Latin)]
    Bosnian, // 2.7
    #[slang(script = Latin)]
    Slovenian, // 2.5 old
    #[slang(script = Latin)]
    Chokwe, // 2.5 2018
    #[slang(script = Latin)]
    Galician, // 2.4 2012
    #[slang(script = Latin)]
    Fon, // 2.3
    #[slang(script = Latin)]
    Esperanto, // 2
    #[slang(script = Latin)]
    AlbanianTosk, // 1.8 2011
    #[slang(script = Latin)]
    DinkaSouthwestern, // < 2
    #[slang(script = Latin)]
    Pangasinan, // 1.8 2010
    #[slang(script = Latin)]
    Irish, // 1.8
    #[slang(script = Latin)]
    Kimbundu, // 1.7 2015
    #[slang(script = Latin)]
    Nuer, // 1.7 2017
    #[slang(script = Latin)]
    AymaraCentral, // 1.7
    #[slang(script = Latin)]
    Latvian, // 1.5 2023
    #[slang(script = Latin)]
    Limburgish, // 1.3 2001
    #[slang(script = Latin)]
    Estonian, // 1.2 2022
    #[slang(script = Latin)]
    Mandailing, // 1.1 2000
    #[slang(script = Latin)]
    Kabiye, // 1 2012
    #[slang(script = Latin)]
    Sardinian, // 1
    #[slang(script = Latin)]
    Mizo, // 1
    #[slang(script = Latin)]
    Jingpho, // 0.94 2001
    #[slang(script = Latin)]
    QuechuaAyacucho, // 0.92 2000
    #[slang(script = Latin, lang = Tamasheq)]
    TamasheqLatin, // 0.9 2022
    #[slang(script = Latin)]
    Kabuverdianu, // 0.87 2017
    #[slang(script = Latin)]
    Occitan, // 0.8 2012
    #[slang(script = Latin)]
    Basque, // 0.8
    #[slang(script = Latin)]
    Fijian, // 0.7 1996
    #[slang(script = Latin)]
    Welsh, // 0.65 2021
    #[slang(script = Latin)]
    Sango, // 0.62 2017
    #[slang(script = Latin)]
    Asturian, // 0.62
    #[slang(script = Latin)]
    Ligurian, // 0.6 2002
    #[slang(script = Latin)]
    Friulian, // 0.6 2014
    #[slang(script = Latin)]
    Maltese, // 0.57 2012
    #[slang(script = Latin)]
    Silesian, // 0.46 2021
    #[slang(script = Latin)]
    Samoan, // 0.43
    #[slang(script = Latin)]
    Luxembourgish, // 0.4
    #[slang(script = Latin)]
    Papiamento, // 0.35
    #[slang(script = Latin)]
    Icelandic, // 0.33
    #[slang(script = Latin)]
    Latgalian, // 0.2 2009
    #[slang(script = Latin)]
    Faroese, // 0.07
    #[slang(script = Latin)]
    GaelicScottish, // 0.07
    #[slang(script = Latin)]
    TatarCrimean, // 0.06 2020
    #[slang(script = Latin)]
    Maori, // 0.05 2015
    #[slang(script = Latin)]
    Hawaiian, // 0.025
    #[slang(script = Latin)]
    Latin, // 0

    // Han
    #[slang(script = HanSimplified, lang = ChineseMandarin)]
    ChineseMandarinSimplified, // 1200
    #[slang(script = Japanese)]
    Japanese, // 123
    #[slang(script = HanTraditional, lang = ChineseMandarin)]
    ChineseMandarinTraditional, // 30
    #[slang(script = HanTraditional, lang = ChineseCantonese)]
    ChineseCantoneseTraditional, // 15
    #[slang(script = Korean)]
    Korean, // not popular script

    // Arabic
    #[slang(script = Arabic)]
    Arabic, // 335
    #[slang(script = Arabic)]
    Urdu, // 255
    #[slang(script = Arabic)]
    ArabicEgyptian, // 119
    #[slang(script = Arabic)]
    PersianWestern, // 83
    #[slang(script = Arabic)]
    ArabicNorthLevantine, // 60 with SouthLevantine
    #[slang(script = Arabic)]
    ArabicSouthLevantine,
    #[slang(script = Arabic)]
    ArabicSudanese, // 52
    #[slang(script = Arabic)]
    ArabicMoroccan, // 40
    #[slang(script = Arabic)]
    Sindhi, // 37
    #[slang(script = Arabic)]
    PersianDari, // 33
    #[slang(script = Arabic)]
    Saraiki, // 29
    #[slang(script = Arabic)]
    ArabicNajdi, // 19
    #[slang(script = Arabic)]
    PashtoSouthern, // 18.5
    #[slang(script = Arabic)]
    ArabicMesopotamian, // 17
    #[slang(script = Arabic, lang = PunjabiEastern)]
    PunjabiEasternShahmukhi, // ~15
    #[slang(script = Arabic)]
    AzerbaijaniSouth, // 14
    #[slang(script = Arabic)]
    ArabicTunisian, // 13
    #[slang(script = Arabic)]
    Uyghur, // 13
    #[slang(script = Arabic)]
    ArabicSouthernYemeni, // 12
    #[slang(script = Arabic, lang = Banjar)]
    BanjarJawi, // 10.7 not popular script
    #[slang(script = Arabic, lang = KanuriCentral)]
    KanuriCentralAjami, // 9.1
    #[slang(script = Arabic)]
    Kashmiri, // 7.1
    #[slang(script = Arabic)]
    KurdishCentral, // 6.1
    #[slang(script = Arabic)]
    KurdishSouthern, // 6
    #[slang(script = Arabic, lang = Acehnese)]
    AcehneseJawi, // 2.8 2010
    #[slang(script = Arabic, lang = Dogri)]
    DogriPersoArabic, // 2.6

    // Devanagari
    #[slang(script = Devanagari)]
    Hindi, // 610
    #[slang(script = Devanagari)]
    Marathi, // 99
    #[slang(script = Devanagari)]
    Bhojpuri, // 52
    #[slang(script = Devanagari)]
    Awadhi, // 39 2011
    #[slang(script = Devanagari, lang = Sindhi)]
    SindhiDevanagari, // 37
    #[slang(script = Devanagari)]
    Nepali, // 32
    #[slang(script = Devanagari)]
    Maithili, // 22
    #[slang(script = Devanagari)]
    Chhattisgarhi, // 16 2011
    #[slang(script = Devanagari)]
    Magahi, // 13 2011
    #[slang(script = Devanagari)]
    Dogri, // 2.6
    #[slang(script = Devanagari, lang = Kashmiri)]
    KashmiriDevanagari, // 0.6
    #[slang(script = Devanagari, lang = Saurashtra)]
    SaurashtraDevanagari, // 0.2
    #[slang(script = Devanagari)]
    Wancho, // 0.06
    #[slang(script = Devanagari)]
    Sanskrit,
    #[slang(script = Devanagari)]
    SanskritVedic,

    // Cyrillic
    #[slang(script = Cyrillic)]
    Russian, // 253
    #[slang(script = Cyrillic)]
    Ukrainian, // 39
    #[slang(script = Cyrillic)]
    Kazakh, // 16
    #[slang(script = Cyrillic)]
    Serbian, // 12
    #[slang(script = Cyrillic)]
    Tajik, // 10.5
    #[slang(script = Cyrillic)]
    Bulgarian, // 7.9
    #[slang(script = Cyrillic)]
    Kyrgyz, // 5.2 2009
    #[slang(script = Cyrillic)]
    MongolianHalh, // 5
    #[slang(script = Cyrillic)]
    Belarusian, // 5
    #[slang(script = Cyrillic)]
    Tatar, // 4.8
    #[slang(script = Cyrillic)]
    Macedonian, // 2
    #[slang(script = Cyrillic)]
    Bashkir, // 0.75
    #[slang(script = Cyrillic)]
    Chuvash, // 0.75
    #[slang(script = Cyrillic, lang = ChurchSlavonic)]
    ChurchSlavonicOld,

    // Bengali
    #[slang(script = Bengali)]
    Bengali, // 284
    #[slang(script = Bengali)]
    Assamese, // 24
    #[slang(script = Bengali, lang = Meitei)]
    MeiteiBengali, // 3
    #[slang(script = Bengali)]
    BishnupriyaManipuri, // 0.12

    // Kannada
    #[slang(script = Kannada)]
    Kannada, // 79
    #[slang(script = Kannada, lang = Tulu)]
    Tulu, // < 2

    // Ethiopic
    #[slang(script = Ethiopic)]
    Amharic, // 60
    #[slang(script = Ethiopic)]
    Tigrinya, // 9.9
    #[slang(script = Ethiopic)]
    Geez, // 0

    // Myanmar
    #[slang(script = Myanmar)]
    Burmese, // 43 2007
    #[slang(script = Myanmar)]
    Shan, // 4.7 2017

    // Hebrew
    #[slang(script = Hebrew)]
    Hebrew, // 8.3 2018
    #[slang(script = Hebrew)]
    YiddishEastern,

    // TaiTham
    #[slang(script = TaiTham)]
    NorthernThai, // 6 2015
    #[slang(script = TaiTham, lang = Lao)]
    LaoTaiTham, // 4.5 2015
    #[slang(script = TaiTham)]
    TaiLue, // 0.55 2013

    // Buginese
    #[slang(script = Buginese, lang = Buginese)]
    BugineseBuginese, // 4 2015
    #[slang(script = Buginese, lang = Makassarese)]
    MakassareseBuginese, // 2.1 2000

    // Batak
    #[slang(script = Batak)]
    TobaBatak, // 1.6 2010
    #[slang(script = Batak)]
    Pakpak, // 1.2 1991
    #[slang(script = Batak)]
    Simalungun, // 1.2 2000
    #[slang(script = Batak)]
    Angkola, // 0.5 2010
    #[slang(script = Batak)]
    Karo, // 0.5 2010
    /// Historical
    #[slang(script = Batak, lang = Mandailing)]
    MandailingBatak,

    // Adlam
    #[slang(script = Adlam, lang = FulaPular)]
    FulaPularAdlam, // 4.8

    // Tifinagh
    #[slang(script = Tifinagh)]
    TamazightCentralAtlas, // 2.7
    #[slang(script = Tifinagh, lang = Tamasheq)]
    TamasheqTifinagh, // 0.9

    // Tibetan
    #[slang(script = Tibetan)]
    Tibetan, // 1.2 1990
    #[slang(script = Tibetan)]
    Dzongkha, // 0.64 2013

    // TaiViet
    #[slang(script = TaiViet)]
    TaiDam, // 0.76 2002
    #[slang(script = TaiViet)]
    TaiDon, // 0.5 2002

    // Cham
    #[slang(script = Cham)]
    ChamEastern, // 0.5 with Western
    #[slang(script = Cham)]
    ChamWestern,

    // CanadianAboriginal
    #[slang(script = CanadianAboriginal)]
    Cree, // 0.096 2016
    #[slang(script = CanadianAboriginal)]
    Ojibwe, // 0.05 2016
    #[slang(script = CanadianAboriginal)]
    Inuktitut, // 0.042

    // KayahLi
    #[slang(script = KayahLi)]
    KayahEastern,
    #[slang(script = KayahLi)]
    KayahWestern,

    // Khojki
    #[slang(script = Khojki, lang = Kutchi)]
    KutchiKhojki,
    /// Historical
    #[slang(script = Khojki, lang = Sindhi)]
    SindhiKhojki,

    // Manichaean
    #[slang(script = Manichaean)]
    MiddlePersianManichaean,
    #[slang(script = Manichaean, lang = Sogdian)]
    SogdianManichaean,

    // Samaritan
    #[slang(script = Samaritan)]
    AramaicSamaritan,
    #[slang(script = Samaritan)]
    HebrewSamaritan,

    // Takri
    #[slang(script = Takri, lang = Dogri)]
    DogriTakri,
    #[slang(script = Takri, lang = DogriKangri)]
    DogriKangriTakri,
    #[slang(script = Takri, lang = Kashmiri)]
    KashmiriTakri,

    // TuluTigalari
    #[slang(script = TuluTigalari, lang = Tulu)]
    TuluTigalari,
    #[slang(script = TuluTigalari, lang = Kannada)]
    KannadaTuluTigalari,
    #[slang(script = TuluTigalari, lang = Sanskrit)]
    SanskritTuluTigalari,

    // Historicals
    // Brahmi
    #[slang(script = Brahmi)]
    Prakrit,
    #[slang(script = Brahmi, lang = Sanskrit)]
    SanskritBrahmi,

    // Cuneiform
    #[slang(script = Cuneiform)]
    Akkadian,
    #[slang(script = Cuneiform)]
    Hittite,
    #[slang(script = Cuneiform)]
    LuwianCuneiform,
    #[slang(script = Cuneiform)]
    Sumerian,

    // Duployan
    /// shorthand systems for English
    #[slang(script = Duployan, lang = English)]
    EnglishDuployan,
    #[slang(script = Duployan, lang = French)]
    FrenchDuployan,

    // Grantha
    /// Historical
    #[slang(script = Grantha, lang = Tamil)]
    TamilGrantha,
    #[slang(script = Grantha, lang = Sanskrit)]
    SanskritGrantha,

    // Kaithi
    /// Historical
    #[slang(script = Kaithi, lang = Bhojpuri)]
    BhojpuriKaithi,
    /// Historical
    #[slang(script = Kaithi, lang = Hindi)]
    HindiKaithi,
    /// Historical
    #[slang(script = Kaithi, lang = Magahi)]
    MagahiKaithi,
    /// Historical
    #[slang(script = Kaithi, lang = Maithili)]
    MaithiliKaithi,

    // Kawi
    #[slang(script = Kawi)]
    OldJavanese,
    #[slang(script = Kawi, lang = Sanskrit)]
    SanskritKawi,

    // Mahajani
    /// Historical
    #[slang(script = Mahajani, lang = Hindi)]
    HindiMahajani,
    /// Historical
    #[slang(script = Mahajani, lang = Marwari)]
    MarwariMahajani,
    /// Historical
    #[slang(script = Mahajani, lang = PunjabiEastern)]
    PunjabiEasternMahajani,

    // OldItalic
    #[slang(script = OldItalic)]
    Etruscan,
    #[slang(script = OldItalic)]
    Oscan,
    #[slang(script = OldItalic)]
    Umbrian,

    // OldPermic
    #[slang(script = OldPermic, lang = KomiZyrian)]
    KomiZyrianOldPermic, // 0.1 but not that script
    #[slang(script = OldPermic, lang = KomiPermyak)]
    KomiPermyakOldPermic, // 0.063

    // PhagsPa
    /// Historical
    #[slang(script = PhagsPa, lang = MongolianHalh)]
    MongolianHalhPhagsPa,
    /// Historical
    #[slang(script = PhagsPa, lang = Tibetan)]
    TibetanPhagsPa,

    // Runic
    #[slang(script = Runic, lang = OldEnglish)]
    OldEnglishRunic,
    #[slang(script = Runic)]
    OldNorse,

    // Sharada
    /// Liturgical
    #[slang(script = Sharada, lang = Kashmiri)]
    KashmiriSharada,
    #[slang(script = Sharada, lang = Sanskrit)]
    SanskritSharada,

    // Soyombo
    /// Historical
    #[slang(script = Soyombo, lang = Tibetan)]
    TibetanSoyombo,
    /// Historical
    #[slang(script = Soyombo, lang = MongolianHalh)]
    MongolianHalhSoyombo,
    #[slang(script = Soyombo, lang = Sanskrit)]
    SanskritSoyombo,

    // ZanabazarSquare
    /// Historical
    #[slang(script = ZanabazarSquare, lang = MongolianHalh)]
    MongolianHalhZanabazarSquare,
    /// Historical
    #[slang(script = ZanabazarSquare, lang = Tibetan)]
    TibetanZanabazarSquare,
    #[slang(script = ZanabazarSquare, lang = Sanskrit)]
    SanskritZanabazarSquare,

    // Single language scripts
    #[slang(script = Ahom)]
    Ahom,
    /// Historical
    #[slang(script = Elbasan, lang = Albanian)]
    AlbanianElbasan,
    /// Historical
    #[slang(script = Todhri, lang = Albanian)]
    AlbanianTodhri,
    #[slang(script = Vithkuqi, lang = AlbanianTosk)]
    AlbanianToskVithkuqi,
    #[slang(script = Cypriot)]
    AncientGreek,
    #[slang(script = OldNorthArabian)]
    AncientNorthArabian,
    #[slang(script = OldSouthArabian)]
    AncientSouthArabian,
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
    #[slang(script = Syriac)]
    AramaicSyriac,
    #[slang(script = Armenian)]
    Armenian,
    #[slang(script = Avestan)]
    Avestan,
    #[slang(script = Balinese, lang = Balinese)]
    BalineseBalinese,
    #[slang(script = Bamum)]
    Bamum,
    #[slang(script = KiratRai)]
    Bantawa,
    #[slang(script = BassaVah)]
    Bassa,
    #[slang(script = OlOnal)]
    Bhumij,
    /// any language adapted to Braille
    #[slang(script = Braille)]
    Braille,
    #[slang(script = Buhid)]
    Buhid,
    #[slang(script = Carian)]
    Carian,
    #[slang(script = CaucasianAlbanian)]
    CaucasianAlbanian,
    #[slang(script = Chakma)]
    Chakma,
    #[slang(script = Cherokee)]
    Cherokee,
    #[slang(script = Bopomofo, lang = ChineseMandarin)]
    ChineseMandarinBopomofo,
    #[slang(script = Nushu)]
    ChineseTuhua,
    #[slang(script = Chorasmian)]
    Chorasmian,
    #[slang(script = Glagolitic, lang = ChurchSlavonic)]
    ChurchSlavonicOldGlagolitic,
    #[slang(script = Coptic)]
    Coptic,
    #[slang(script = Thaana)]
    Dhivehi,
    /// Historical
    #[slang(script = DivesAkuru, lang = Dhivehi)]
    DhivehiDivesAkuru,
    #[slang(script = Dogra, lang = Dogri)]
    DogriDogra,
    #[slang(script = EgyptianHieroglyphs)]
    EgyptianHieroglyphs,
    #[slang(script = Deseret, lang = English)]
    EnglishDeseret,
    #[slang(script = Shavian, lang = English)]
    EnglishShavian,
    #[slang(script = Tagalog, lang = Filipino)]
    FilipinoTagalog,
    #[slang(script = Kharoshthi)]
    Gandhari,
    #[slang(script = Georgian)]
    Georgian,
    #[slang(script = GunjalaGondi, lang = Gondi)]
    GondiGunjala,
    #[slang(script = MasaramGondi, lang = Gondi)]
    GondiMasaram,
    #[slang(script = Gothic)]
    Gothic,
    #[slang(script = Greek)]
    Greek,
    #[slang(script = Gujarati)]
    Gujarati,
    #[slang(script = GurungKhema, lang = Gurung)]
    GurungKhema,
    #[slang(script = Hanunoo)]
    Hanunoo,
    #[slang(script = Miao, lang = Hmong)]
    HmongMiao,
    #[slang(script = NyiakengPuachueHmong, lang = Hmong)]
    HmongNyiakengPuachue,
    #[slang(script = PahawhHmong, lang = Hmong)]
    HmongPahawh,
    #[slang(script = WarangCiti, lang = Ho)]
    HoWarangCiti,
    #[slang(script = OldHungarian, lang = Hungarian)]
    HungarianOld,
    #[slang(script = Javanese, lang = Javanese)]
    JavaneseJavanese,
    #[slang(script = KhitanSmallScript)]
    Khitan,
    #[slang(script = Khmer)]
    Khmer,
    #[slang(script = Yezidi, lang = KurdishNorthern)]
    KurdishNorthernYezidi,
    #[slang(script = Lao)]
    Lao,
    #[slang(script = Lepcha)]
    Lepcha,
    #[slang(script = Limbu)]
    Limbu,
    #[slang(script = Lisu)]
    Lisu,
    #[slang(script = Yi)]
    Loloish,
    #[slang(script = AnatolianHieroglyphs)]
    LuwianHieroglyphic,
    #[slang(script = Lycian)]
    Lycian,
    #[slang(script = Lydian)]
    Lydian,
    #[slang(script = Tirhuta, lang = Maithili)]
    MaithiliTirhuta,
    /// Historical
    #[slang(script = Makasar, lang = Makassarese)]
    MakassareseMakasar,
    #[slang(script = Malayalam)]
    Malayalam,
    #[slang(script = Nko)]
    Manding,
    #[slang(script = Modi, lang = Marathi)]
    MarathiModi,
    #[slang(script = Medefaidrin)]
    Medefaidrin,
    #[slang(script = MeeteiMayek, lang = Meitei)]
    MeiteiMeeteiMayek,
    #[slang(script = MendeKikakui)]
    Mende,
    #[slang(script = MeroiticCursive, lang = Meroitic)]
    MeroiticCursive,
    #[slang(script = MeroiticHieroglyphs, lang = Meroitic)]
    MeroiticHieroglyphs,
    #[slang(script = InscriptionalPahlavi, lang = MiddlePersianPahlavi)]
    MiddlePersianPahlaviInscriptional,
    #[slang(script = PsalterPahlavi, lang = MiddlePersianPahlavi)]
    MiddlePersianPahlaviPsalter,
    #[slang(script = CyproMinoan)]
    Minoan,
    #[slang(script = LinearA)]
    MinoanLinearA,
    #[slang(script = Mongolian, lang = MongolianHalh)]
    MongolianHalhMongolian,
    #[slang(script = Mro)]
    Mro,
    #[slang(script = NagMundari)]
    Mundari,
    #[slang(script = LinearB)]
    MycenaeanGreek,
    #[slang(script = Newa)]
    Newar,
    #[slang(script = Ogham, lang = OldIrish)]
    OldIrishOgham,
    #[slang(script = OldPersian)]
    OldPersian,
    #[slang(script = OldTurkic)]
    OldTurkic,
    #[slang(script = OldUyghur)]
    OldUyghur,
    #[slang(script = Oriya)]
    OriyaOdia,
    #[slang(script = Osage)]
    Osage,
    #[slang(script = InscriptionalParthian)]
    Parthian,
    #[slang(script = Phoenician)]
    Phoenician,
    #[slang(script = Gurmukhi)]
    PunjabiEastern,
    /// Historical
    #[slang(script = Rejang, lang = Rejang)]
    RejangRejang,
    #[slang(script = HanifiRohingya)]
    Rohingya,
    #[slang(script = Bhaiksuki, lang = Sanskrit)]
    SanskritBhaiksuki,
    #[slang(script = Nandinagari, lang = Sanskrit)]
    SanskritNandinagari,
    #[slang(script = Siddham, lang = Sanskrit)]
    SanskritSiddham,
    #[slang(script = OlChiki)]
    Santali,
    #[slang(script = Multani, lang = Saraiki)]
    SaraikiMultani,
    #[slang(script = Saurashtra, lang = Saurashtra)]
    SaurashtraSaurashtra,
    #[slang(script = SignWriting)]
    SignLanguages,
    #[slang(script = Khudawadi, lang = Sindhi)]
    SindhiKhudawadi,
    #[slang(script = Sinhala)]
    Sinhala,
    #[slang(script = Sogdian)]
    Sogdian,
    #[slang(script = OldSogdian, lang = Sogdian)]
    SogdianOld,
    #[slang(script = Osmanya, lang = Somali)]
    SomaliOsmanya,
    #[slang(script = SoraSompeng)]
    Sora,
    #[slang(script = Sundanese, lang = Sundanese)]
    SundaneseSundanese,
    #[slang(script = Sunuwar)]
    Sunuwar,
    #[slang(script = SylotiNagri)]
    Sylheti,
    #[slang(script = Tagbanwa)]
    Tagbanwa,
    #[slang(script = NewTaiLue, lang = TaiLue)]
    TaiLueNew,
    #[slang(script = TaiLe)]
    TaiNuea,
    #[slang(script = Tamil)]
    Tamil,
    #[slang(script = Tangsa)]
    Tangsa,
    #[slang(script = Tangut)]
    Tangut,
    #[slang(script = PauCinHau)]
    Tedim,
    #[slang(script = Telugu)]
    Telugu,
    #[slang(script = Thai)]
    Thai,
    #[slang(script = Toto)]
    Toto,
    #[slang(script = Ugaritic)]
    Ugaritic,
    #[slang(script = Vai)]
    Vai,
    #[slang(script = Wancho, lang = Wancho)]
    WanchoWancho,
    #[slang(script = Garay, lang = Wolof)]
    WolofGaray,
    #[slang(script = Marchen)]
    Zhangzhung,
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
