use super::UcdScript;
use alphabet_detector_macros::Script;
use strum_macros::{EnumCount as EnumCountDerive, EnumIter};

// ISO 15924 code.
/// Has aliases in comparison to [`UcdScript`](enum.UcdScript.html).
/// Int representation is unstable and can be changed anytime.
/// Code representation (const
/// [`into_code`](enum.Script.html#method.into_code)/[`from_code`](enum.Script.html#method.from_code))
/// or string representation (const
/// [`into_str`](enum.Script.html#method.into_str)/[`from_str`](enum.Script.html#method.from_str))
/// are more stable.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumCountDerive, EnumIter, Script,
)]
pub enum Script {
    #[scr(short = "Adlm", code = 166)]
    Adlam,
    #[scr(short = "Ahom", code = 338)]
    Ahom,
    #[scr(short = "Hluw", code = 80)]
    AnatolianHieroglyphs,
    #[scr(short = "Arab", code = 160)]
    Arabic,
    #[scr(short = "Armn", code = 230)]
    Armenian,
    #[scr(short = "Avst", code = 134)]
    Avestan,
    #[scr(short = "Bali", code = 360)]
    Balinese,
    #[scr(short = "Bamu", code = 435)]
    Bamum,
    #[scr(short = "Bass", code = 259)]
    BassaVah,
    #[scr(short = "Batk", code = 365)]
    Batak,
    #[scr(short = "Beng", code = 325)]
    Bengali,
    #[scr(short = "Berf", code = 258)]
    BeriaErfe,
    #[scr(short = "Bhks", code = 334)]
    Bhaiksuki,
    #[scr(short = "Bopo", code = 285)]
    Bopomofo,
    #[scr(short = "Brah", code = 300)]
    Brahmi,
    #[scr(short = "Brai", code = 570)]
    Braille,
    #[scr(short = "Bugi", code = 367)]
    Buginese,
    #[scr(short = "Buhd", code = 372)]
    Buhid,
    #[scr(short = "Cans", code = 440)]
    CanadianAboriginal,
    #[scr(short = "Cari", code = 201)]
    Carian,
    #[scr(short = "Aghb", code = 239)]
    CaucasianAlbanian,
    #[scr(short = "Cakm", code = 349)]
    Chakma,
    #[scr(short = "Cham", code = 358)]
    Cham,
    #[scr(short = "Cher", code = 445)]
    Cherokee,
    #[scr(short = "Chrs", code = 109)]
    Chorasmian,
    #[scr(short = "Zyyy", code = 998)]
    Common,
    #[scr(short = "Copt", code = 204)]
    Coptic,
    #[scr(short = "Xsux", code = 20)]
    Cuneiform,
    #[scr(short = "Cprt", code = 403)]
    Cypriot,
    #[scr(short = "Cpmn", code = 402)]
    CyproMinoan,
    #[scr(short = "Cyrl", code = 220)]
    Cyrillic,
    #[scr(short = "Dsrt", code = 250)]
    Deseret,
    #[scr(short = "Deva", code = 315)]
    Devanagari,
    #[scr(short = "Diak", code = 342)]
    DivesAkuru,
    #[scr(short = "Dogr", code = 328)]
    Dogra,
    #[scr(short = "Dupl", code = 755)]
    Duployan,
    #[scr(short = "Egyp", code = 50)]
    EgyptianHieroglyphs,
    #[scr(short = "Elba", code = 226)]
    Elbasan,
    #[scr(short = "Elym", code = 128)]
    Elymaic,
    #[scr(short = "Ethi", code = 430)]
    Ethiopic,
    #[scr(short = "Gara", code = 164)]
    Garay,
    #[scr(short = "Geor", code = 240)]
    Georgian,
    #[scr(short = "Glag", code = 225)]
    Glagolitic,
    #[scr(short = "Goth", code = 206)]
    Gothic,
    #[scr(short = "Gran", code = 343)]
    Grantha,
    #[scr(short = "Grek", code = 200)]
    Greek,
    #[scr(short = "Gujr", code = 320)]
    Gujarati,
    #[scr(short = "Gong", code = 312)]
    GunjalaGondi,
    #[scr(short = "Guru", code = 310)]
    Gurmukhi,
    #[scr(short = "Gukh", code = 397)]
    GurungKhema,
    #[scr(short = "Hani", code = 500)]
    Han,
    #[scr(short = "Hang", code = 286)]
    Hangul,
    #[scr(short = "Rohg", code = 167)]
    HanifiRohingya,
    #[scr(short = "Hans", code = 501)]
    HanSimplified,
    #[scr(short = "Hant", code = 502)]
    HanTraditional,
    #[scr(short = "Hano", code = 371)]
    Hanunoo,
    #[scr(short = "Hatr", code = 127)]
    Hatran,
    #[scr(short = "Hebr", code = 125)]
    Hebrew,
    #[scr(short = "Hira", code = 410)]
    Hiragana,
    #[scr(short = "Armi", code = 124)]
    ImperialAramaic,
    #[scr(short = "Zinh", code = 994)]
    Inherited,
    #[scr(short = "Phli", code = 131)]
    InscriptionalPahlavi,
    #[scr(short = "Prti", code = 130)]
    InscriptionalParthian,
    #[scr(short = "Jpan", code = 413)]
    Japanese,
    #[scr(short = "Java", code = 361)]
    Javanese,
    #[scr(short = "Kthi", code = 317)]
    Kaithi,
    #[scr(short = "Knda", code = 345)]
    Kannada,
    #[scr(short = "Kana", code = 411)]
    Katakana,
    #[scr(short = "Kawi", code = 368)]
    Kawi,
    #[scr(short = "Kali", code = 357)]
    KayahLi,
    #[scr(short = "Khar", code = 305)]
    Kharoshthi,
    #[scr(short = "Kits", code = 288)]
    KhitanSmallScript,
    #[scr(short = "Khmr", code = 355)]
    Khmer,
    #[scr(short = "Khoj", code = 322)]
    Khojki,
    #[scr(short = "Sind", code = 318)]
    Khudawadi,
    #[scr(short = "Krai", code = 396)]
    KiratRai,
    #[scr(short = "Kore", code = 287)]
    Korean,
    #[scr(short = "Laoo", code = 356)]
    Lao,
    #[scr(short = "Latn", code = 215)]
    Latin,
    #[scr(short = "Lepc", code = 335)]
    Lepcha,
    #[scr(short = "Limb", code = 336)]
    Limbu,
    #[scr(short = "Lina", code = 400)]
    LinearA,
    #[scr(short = "Linb", code = 401)]
    LinearB,
    #[scr(short = "Lisu", code = 399)]
    Lisu,
    #[scr(short = "Lyci", code = 202)]
    Lycian,
    #[scr(short = "Lydi", code = 116)]
    Lydian,
    #[scr(short = "Mahj", code = 314)]
    Mahajani,
    #[scr(short = "Maka", code = 366)]
    Makasar,
    #[scr(short = "Mlym", code = 347)]
    Malayalam,
    #[scr(short = "Mand", code = 140)]
    Mandaic,
    #[scr(short = "Mani", code = 139)]
    Manichaean,
    #[scr(short = "Marc", code = 332)]
    Marchen,
    #[scr(short = "Gonm", code = 313)]
    MasaramGondi,
    #[scr(short = "Medf", code = 265)]
    Medefaidrin,
    #[scr(short = "Mtei", code = 337)]
    MeeteiMayek,
    #[scr(short = "Mend", code = 438)]
    MendeKikakui,
    #[scr(short = "Merc", code = 101)]
    MeroiticCursive,
    #[scr(short = "Mero", code = 100)]
    MeroiticHieroglyphs,
    #[scr(short = "Plrd", code = 282)]
    Miao,
    #[scr(short = "Modi", code = 324)]
    Modi,
    #[scr(short = "Mong", code = 145)]
    Mongolian,
    #[scr(short = "Mroo", code = 264)]
    Mro,
    #[scr(short = "Mult", code = 323)]
    Multani,
    #[scr(short = "Mymr", code = 350)]
    Myanmar,
    #[scr(short = "Nbat", code = 159)]
    Nabataean,
    #[scr(short = "Nagm", code = 295)]
    NagMundari,
    #[scr(short = "Nand", code = 311)]
    Nandinagari,
    #[scr(short = "Newa", code = 333)]
    Newa,
    #[scr(short = "Talu", code = 354)]
    NewTaiLue,
    #[scr(short = "Nkoo", code = 165)]
    Nko,
    #[scr(short = "Nshu", code = 499)]
    Nushu,
    #[scr(short = "Hmnp", code = 451)]
    NyiakengPuachueHmong,
    #[scr(short = "Ogam", code = 212)]
    Ogham,
    #[scr(short = "Olck", code = 261)]
    OlChiki,
    #[scr(short = "Hung", code = 176)]
    OldHungarian,
    #[scr(short = "Ital", code = 210)]
    OldItalic,
    #[scr(short = "Narb", code = 106)]
    OldNorthArabian,
    #[scr(short = "Perm", code = 227)]
    OldPermic,
    #[scr(short = "Xpeo", code = 30)]
    OldPersian,
    #[scr(short = "Sogo", code = 142)]
    OldSogdian,
    #[scr(short = "Sarb", code = 105)]
    OldSouthArabian,
    #[scr(short = "Orkh", code = 175)]
    OldTurkic,
    #[scr(short = "Ougr", code = 143)]
    OldUyghur,
    #[scr(short = "Onao", code = 296)]
    OlOnal,
    #[scr(short = "Orya", code = 327)]
    Oriya,
    #[scr(short = "Osge", code = 219)]
    Osage,
    #[scr(short = "Osma", code = 260)]
    Osmanya,
    #[scr(short = "Hmng", code = 450)]
    PahawhHmong,
    #[scr(short = "Palm", code = 126)]
    Palmyrene,
    #[scr(short = "Pauc", code = 263)]
    PauCinHau,
    #[scr(short = "Phag", code = 331)]
    PhagsPa,
    #[scr(short = "Phnx", code = 115)]
    Phoenician,
    #[scr(short = "Phlp", code = 132)]
    PsalterPahlavi,
    #[scr(short = "Rjng", code = 363)]
    Rejang,
    #[scr(short = "Runr", code = 211)]
    Runic,
    #[scr(short = "Samr", code = 123)]
    Samaritan,
    #[scr(short = "Saur", code = 344)]
    Saurashtra,
    #[scr(short = "Shrd", code = 319)]
    Sharada,
    #[scr(short = "Shaw", code = 281)]
    Shavian,
    #[scr(short = "Sidd", code = 302)]
    Siddham,
    #[scr(short = "Sidt", code = 180)]
    Sidetic,
    #[scr(short = "Sgnw", code = 95)]
    SignWriting,
    #[scr(short = "Sinh", code = 348)]
    Sinhala,
    #[scr(short = "Sogd", code = 141)]
    Sogdian,
    #[scr(short = "Sora", code = 398)]
    SoraSompeng,
    #[scr(short = "Soyo", code = 329)]
    Soyombo,
    #[scr(short = "Sund", code = 362)]
    Sundanese,
    #[scr(short = "Sunu", code = 274)]
    Sunuwar,
    #[scr(short = "Sylo", code = 316)]
    SylotiNagri,
    #[scr(short = "Syrc", code = 135)]
    Syriac,
    #[scr(short = "Tglg", code = 370)]
    Tagalog,
    #[scr(short = "Tagb", code = 373)]
    Tagbanwa,
    #[scr(short = "Tale", code = 353)]
    TaiLe,
    #[scr(short = "Lana", code = 351)]
    TaiTham,
    #[scr(short = "Tavt", code = 359)]
    TaiViet,
    #[scr(short = "Tayo", code = 380)]
    TaiYo,
    #[scr(short = "Takr", code = 321)]
    Takri,
    #[scr(short = "Taml", code = 346)]
    Tamil,
    #[scr(short = "Tnsa", code = 275)]
    Tangsa,
    #[scr(short = "Tang", code = 520)]
    Tangut,
    #[scr(short = "Telu", code = 340)]
    Telugu,
    #[scr(short = "Thaa", code = 170)]
    Thaana,
    #[scr(short = "Thai", code = 352)]
    Thai,
    #[scr(short = "Tibt", code = 330)]
    Tibetan,
    #[scr(short = "Tfng", code = 120)]
    Tifinagh,
    #[scr(short = "Tirh", code = 326)]
    Tirhuta,
    #[scr(short = "Todr", code = 229)]
    Todhri,
    #[scr(short = "Tols", code = 299)]
    TolongSiki,
    #[scr(short = "Toto", code = 294)]
    Toto,
    #[scr(short = "Tutg", code = 341)]
    TuluTigalari,
    #[scr(short = "Ugar", code = 40)]
    Ugaritic,
    #[scr(short = "Vaii", code = 470)]
    Vai,
    #[scr(short = "Vith", code = 228)]
    Vithkuqi,
    #[scr(short = "Wcho", code = 283)]
    Wancho,
    #[scr(short = "Wara", code = 262)]
    WarangCiti,
    #[scr(short = "Yezi", code = 192)]
    Yezidi,
    #[scr(short = "Yiii", code = 460)]
    Yi,
    #[scr(short = "Zanb", code = 339)]
    ZanabazarSquare,
}

impl_try_from!(Script, u16, u16 i16 u32 i32 usize isize u64 i64 u128 i128);
impl_serde!(Script, "Script");

impl From<Script> for UcdScript {
    #[inline]
    fn from(s: Script) -> Self {
        use Script::*;
        match s {
            Adlam => UcdScript::Adlam,
            Ahom => UcdScript::Ahom,
            AnatolianHieroglyphs => UcdScript::AnatolianHieroglyphs,
            Arabic => UcdScript::Arabic,
            Armenian => UcdScript::Armenian,
            Avestan => UcdScript::Avestan,
            Balinese => UcdScript::Balinese,
            Bamum => UcdScript::Bamum,
            BassaVah => UcdScript::BassaVah,
            Batak => UcdScript::Batak,
            Bengali => UcdScript::Bengali,
            BeriaErfe => UcdScript::BeriaErfe,
            Bhaiksuki => UcdScript::Bhaiksuki,
            Bopomofo => UcdScript::Bopomofo,
            Brahmi => UcdScript::Brahmi,
            Braille => UcdScript::Braille,
            Buginese => UcdScript::Buginese,
            Buhid => UcdScript::Buhid,
            CanadianAboriginal => UcdScript::CanadianAboriginal,
            Carian => UcdScript::Carian,
            CaucasianAlbanian => UcdScript::CaucasianAlbanian,
            Chakma => UcdScript::Chakma,
            Cham => UcdScript::Cham,
            Cherokee => UcdScript::Cherokee,
            Chorasmian => UcdScript::Chorasmian,
            Common => UcdScript::Common,
            Coptic => UcdScript::Coptic,
            Cuneiform => UcdScript::Cuneiform,
            Cypriot => UcdScript::Cypriot,
            CyproMinoan => UcdScript::CyproMinoan,
            Cyrillic => UcdScript::Cyrillic,
            Deseret => UcdScript::Deseret,
            Devanagari => UcdScript::Devanagari,
            DivesAkuru => UcdScript::DivesAkuru,
            Dogra => UcdScript::Dogra,
            Duployan => UcdScript::Duployan,
            EgyptianHieroglyphs => UcdScript::EgyptianHieroglyphs,
            Elbasan => UcdScript::Elbasan,
            Elymaic => UcdScript::Elymaic,
            Ethiopic => UcdScript::Ethiopic,
            Garay => UcdScript::Garay,
            Georgian => UcdScript::Georgian,
            Glagolitic => UcdScript::Glagolitic,
            Gothic => UcdScript::Gothic,
            Grantha => UcdScript::Grantha,
            Greek => UcdScript::Greek,
            Gujarati => UcdScript::Gujarati,
            GunjalaGondi => UcdScript::GunjalaGondi,
            Gurmukhi => UcdScript::Gurmukhi,
            GurungKhema => UcdScript::GurungKhema,
            Han => UcdScript::Han,
            Hangul => UcdScript::Hangul,
            HanifiRohingya => UcdScript::HanifiRohingya,
            HanSimplified => UcdScript::Han,
            HanTraditional => UcdScript::Han,
            Hanunoo => UcdScript::Hanunoo,
            Hatran => UcdScript::Hatran,
            Hebrew => UcdScript::Hebrew,
            Hiragana => UcdScript::Hiragana,
            ImperialAramaic => UcdScript::ImperialAramaic,
            Inherited => UcdScript::Inherited,
            InscriptionalPahlavi => UcdScript::InscriptionalPahlavi,
            InscriptionalParthian => UcdScript::InscriptionalParthian,
            Japanese => UcdScript::Han,
            Javanese => UcdScript::Javanese,
            Kaithi => UcdScript::Kaithi,
            Kannada => UcdScript::Kannada,
            Katakana => UcdScript::Katakana,
            Kawi => UcdScript::Kawi,
            KayahLi => UcdScript::KayahLi,
            Kharoshthi => UcdScript::Kharoshthi,
            KhitanSmallScript => UcdScript::KhitanSmallScript,
            Khmer => UcdScript::Khmer,
            Khojki => UcdScript::Khojki,
            Khudawadi => UcdScript::Khudawadi,
            KiratRai => UcdScript::KiratRai,
            Korean => UcdScript::Han,
            Lao => UcdScript::Lao,
            Latin => UcdScript::Latin,
            Lepcha => UcdScript::Lepcha,
            Limbu => UcdScript::Limbu,
            LinearA => UcdScript::LinearA,
            LinearB => UcdScript::LinearB,
            Lisu => UcdScript::Lisu,
            Lycian => UcdScript::Lycian,
            Lydian => UcdScript::Lydian,
            Mahajani => UcdScript::Mahajani,
            Makasar => UcdScript::Makasar,
            Malayalam => UcdScript::Malayalam,
            Mandaic => UcdScript::Mandaic,
            Manichaean => UcdScript::Manichaean,
            Marchen => UcdScript::Marchen,
            MasaramGondi => UcdScript::MasaramGondi,
            Medefaidrin => UcdScript::Medefaidrin,
            MeeteiMayek => UcdScript::MeeteiMayek,
            MendeKikakui => UcdScript::MendeKikakui,
            MeroiticCursive => UcdScript::MeroiticCursive,
            MeroiticHieroglyphs => UcdScript::MeroiticHieroglyphs,
            Miao => UcdScript::Miao,
            Modi => UcdScript::Modi,
            Mongolian => UcdScript::Mongolian,
            Mro => UcdScript::Mro,
            Multani => UcdScript::Multani,
            Myanmar => UcdScript::Myanmar,
            Nabataean => UcdScript::Nabataean,
            NagMundari => UcdScript::NagMundari,
            Nandinagari => UcdScript::Nandinagari,
            Newa => UcdScript::Newa,
            NewTaiLue => UcdScript::NewTaiLue,
            Nko => UcdScript::Nko,
            Nushu => UcdScript::Nushu,
            NyiakengPuachueHmong => UcdScript::NyiakengPuachueHmong,
            Ogham => UcdScript::Ogham,
            OlChiki => UcdScript::OlChiki,
            OldHungarian => UcdScript::OldHungarian,
            OldItalic => UcdScript::OldItalic,
            OldNorthArabian => UcdScript::OldNorthArabian,
            OldPermic => UcdScript::OldPermic,
            OldPersian => UcdScript::OldPersian,
            OldSogdian => UcdScript::OldSogdian,
            OldSouthArabian => UcdScript::OldSouthArabian,
            OldTurkic => UcdScript::OldTurkic,
            OldUyghur => UcdScript::OldUyghur,
            OlOnal => UcdScript::OlOnal,
            Oriya => UcdScript::Oriya,
            Osage => UcdScript::Osage,
            Osmanya => UcdScript::Osmanya,
            PahawhHmong => UcdScript::PahawhHmong,
            Palmyrene => UcdScript::Palmyrene,
            PauCinHau => UcdScript::PauCinHau,
            PhagsPa => UcdScript::PhagsPa,
            Phoenician => UcdScript::Phoenician,
            PsalterPahlavi => UcdScript::PsalterPahlavi,
            Rejang => UcdScript::Rejang,
            Runic => UcdScript::Runic,
            Samaritan => UcdScript::Samaritan,
            Saurashtra => UcdScript::Saurashtra,
            Sharada => UcdScript::Sharada,
            Shavian => UcdScript::Shavian,
            Siddham => UcdScript::Siddham,
            Sidetic => UcdScript::Sidetic,
            SignWriting => UcdScript::SignWriting,
            Sinhala => UcdScript::Sinhala,
            Sogdian => UcdScript::Sogdian,
            SoraSompeng => UcdScript::SoraSompeng,
            Soyombo => UcdScript::Soyombo,
            Sundanese => UcdScript::Sundanese,
            Sunuwar => UcdScript::Sunuwar,
            SylotiNagri => UcdScript::SylotiNagri,
            Syriac => UcdScript::Syriac,
            Tagalog => UcdScript::Tagalog,
            Tagbanwa => UcdScript::Tagbanwa,
            TaiLe => UcdScript::TaiLe,
            TaiTham => UcdScript::TaiTham,
            TaiViet => UcdScript::TaiViet,
            TaiYo => UcdScript::TaiYo,
            Takri => UcdScript::Takri,
            Tamil => UcdScript::Tamil,
            Tangsa => UcdScript::Tangsa,
            Tangut => UcdScript::Tangut,
            Telugu => UcdScript::Telugu,
            Thaana => UcdScript::Thaana,
            Thai => UcdScript::Thai,
            Tibetan => UcdScript::Tibetan,
            Tifinagh => UcdScript::Tifinagh,
            Tirhuta => UcdScript::Tirhuta,
            Todhri => UcdScript::Todhri,
            TolongSiki => UcdScript::TolongSiki,
            Toto => UcdScript::Toto,
            TuluTigalari => UcdScript::TuluTigalari,
            Ugaritic => UcdScript::Ugaritic,
            Vai => UcdScript::Vai,
            Vithkuqi => UcdScript::Vithkuqi,
            Wancho => UcdScript::Wancho,
            WarangCiti => UcdScript::WarangCiti,
            Yezidi => UcdScript::Yezidi,
            Yi => UcdScript::Yi,
            ZanabazarSquare => UcdScript::ZanabazarSquare,
        }
    }
}
