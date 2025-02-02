//! Data type for scripts. The four-letter values defined by ISO 15924.

use std::os::raw::c_int;

use crate::{bind, Result, SdlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Script {
    Common,
    Inherited,
    Unknown,
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Han,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Lao,
    Latin,
    Malayalam,
    Oriya,
    Tamil,
    Telugu,
    Thai,
    Tibetan,
    Bopomofo,
    Braille,
    CanadianSyllabics,
    Cherokee,
    Ethiopic,
    Khmer,
    Mongolian,
    Myanmar,
    Ogham,
    Runic,
    Sinhala,
    Syriac,
    Thaana,
    Yi,
    Deseret,
    Gothic,
    OldItalic,
    Buhid,
    Hanunoo,
    Tagalog,
    Tagbanwa,
    Cypriot,
    Limbu,
    LinearB,
    Osmanya,
    Shavian,
    TaiLe,
    Ugaritic,
    Buginese,
    Coptic,
    Glagolitic,
    Kharoshthi,
    NewTaiLue,
    OldPersian,
    SylotiNagri,
    Tifinagh,
    Balinese,
    Cuneiform,
    Nko,
    PhagsPa,
    Phoenician,
    Carian,
    Cham,
    KayahLi,
    Lepcha,
    Lycian,
    Lydian,
    OlChiki,
    Rejang,
    Saurashtra,
    Sundanese,
    Vai,
    Avestan,
    Bamum,
    EgyptianHieroglyphs,
    ImperialAramaic,
    InscriptionalPahlavi,
    InscriptionalParthian,
    Javanese,
    Kaithi,
    Lisu,
    MeeteiMayek,
    OldSouthArabian,
    OldTurkic,
    Samaritan,
    TaiTham,
    TaiViet,
    Batak,
    Brahmi,
    Mandaic,
    Chakma,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    Sharada,
    SoraSompeng,
    Takri,
    BassaVah,
    CaucasianAlbanian,
    Duployan,
    Elbasan,
    Grantha,
    Khojki,
    Khudawadi,
    LinearA,
    Mahajani,
    Manichaean,
    MendeKikakui,
    Modi,
    Mro,
    Nabataean,
    OldNorthArabian,
    OldPermic,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PsalterPahlavi,
    Siddham,
    Tirhuta,
    WarangCiti,
    Ahom,
    AnatolianHieroglyphs,
    Hatran,
    Multani,
    OldHungarian,
    Signwriting,
    Adlam,
    Bhaiksuki,
    Marchen,
    Osage,
    Tangut,
    Newa,
    MasaramGondi,
    Nushu,
    Soyombo,
    ZanabazarSquare,
    Dogra,
    GunjalaGondi,
    HanifiRohingya,
    Makasar,
    Medefaidrin,
    OldSogdian,
    Sogdian,
    Elymaic,
    Nandinagari,
    NyiakengPuachueHmong,
    Wancho,
    Chorasmian,
    DivesAkuru,
    KhitanSmallScript,
    Yezidi,
}

impl Script {
    const fn into_raw(self) -> c_int {
        use Script::*;
        let code: &[u8; 4] = match self {
            Common => b"Zyyy",
            Inherited => b"Zinh",
            Unknown => b"Zzzz",
            Arabic => b"Arab",
            Armenian => b"Armn",
            Bengali => b"Beng",
            Cyrillic => b"Cyrl",
            Devanagari => b"Deva",
            Georgian => b"Geor",
            Greek => b"Grek",
            Gujarati => b"Gujr",
            Gurmukhi => b"Guru",
            Hangul => b"Hang",
            Han => b"Hani",
            Hebrew => b"Hebr",
            Hiragana => b"Hira",
            Kannada => b"Knda",
            Katakana => b"Kana",
            Lao => b"Laoo",
            Latin => b"Latn",
            Malayalam => b"Mlym",
            Oriya => b"Orya",
            Tamil => b"Taml",
            Telugu => b"Telu",
            Thai => b"Thai",
            Tibetan => b"Tibt",
            Bopomofo => b"Bopo",
            Braille => b"Brai",
            CanadianSyllabics => b"Cans",
            Cherokee => b"Cher",
            Ethiopic => b"Ethi",
            Khmer => b"Khmr",
            Mongolian => b"Mong",
            Myanmar => b"Mymr",
            Ogham => b"Ogam",
            Runic => b"Runr",
            Sinhala => b"Sinh",
            Syriac => b"Syrc",
            Thaana => b"Thaa",
            Yi => b"Yiii",
            Deseret => b"Dsrt",
            Gothic => b"Goth",
            OldItalic => b"Ital",
            Buhid => b"Buhd",
            Hanunoo => b"Hano",
            Tagalog => b"Tglg",
            Tagbanwa => b"Tagb",
            Cypriot => b"Cprt",
            Limbu => b"Limb",
            LinearB => b"Linb",
            Osmanya => b"Osma",
            Shavian => b"Shaw",
            TaiLe => b"Tale",
            Ugaritic => b"Ugar",
            Buginese => b"Bugi",
            Coptic => b"Copt",
            Glagolitic => b"Glag",
            Kharoshthi => b"Khar",
            NewTaiLue => b"Talu",
            OldPersian => b"Xpeo",
            SylotiNagri => b"Sylo",
            Tifinagh => b"Tfng",
            Balinese => b"Bali",
            Cuneiform => b"Xsux",
            Nko => b"Nkoo",
            PhagsPa => b"Phag",
            Phoenician => b"Phnx",
            Carian => b"Cari",
            Cham => b"Cham",
            KayahLi => b"Kali",
            Lepcha => b"Lepc",
            Lycian => b"Lyci",
            Lydian => b"Lydi",
            OlChiki => b"Olck",
            Rejang => b"Rjng",
            Saurashtra => b"Saur",
            Sundanese => b"Sund",
            Vai => b"Vaii",
            Avestan => b"Avst",
            Bamum => b"Bamu",
            EgyptianHieroglyphs => b"Egyp",
            ImperialAramaic => b"Armi",
            InscriptionalPahlavi => b"Phli",
            InscriptionalParthian => b"Prti",
            Javanese => b"Java",
            Kaithi => b"Kthi",
            Lisu => b"Lisu",
            MeeteiMayek => b"Mtei",
            OldSouthArabian => b"Sarb",
            OldTurkic => b"Orkh",
            Samaritan => b"Samr",
            TaiTham => b"Lana",
            TaiViet => b"Tavy",
            Batak => b"Batk",
            Brahmi => b"Brah",
            Mandaic => b"Mand",
            Chakma => b"Cakm",
            MeroiticCursive => b"Merc",
            MeroiticHieroglyphs => b"Mero",
            Miao => b"Plrd",
            Sharada => b"Shrd",
            SoraSompeng => b"Sora",
            Takri => b"Takr",
            BassaVah => b"Bass",
            CaucasianAlbanian => b"Aghb",
            Duployan => b"Dupl",
            Elbasan => b"Elba",
            Grantha => b"Gran",
            Khojki => b"Khoj",
            Khudawadi => b"Sind",
            LinearA => b"Lina",
            Mahajani => b"Mahj",
            Manichaean => b"Mani",
            MendeKikakui => b"Mend",
            Modi => b"Modi",
            Mro => b"Mroo",
            Nabataean => b"Nbat",
            OldNorthArabian => b"Narb",
            OldPermic => b"Perm",
            PahawhHmong => b"Hmng",
            Palmyrene => b"Palm",
            PauCinHau => b"Pauc",
            PsalterPahlavi => b"Phlp",
            Siddham => b"Sidd",
            Tirhuta => b"Tirh",
            WarangCiti => b"Wara",
            Ahom => b"Ahom",
            AnatolianHieroglyphs => b"Hluw",
            Hatran => b"Hatr",
            Multani => b"Mult",
            OldHungarian => b"Hung",
            Signwriting => b"Sgnw",
            Adlam => b"Adlm",
            Bhaiksuki => b"Bhks",
            Marchen => b"Marc",
            Osage => b"Osge",
            Tangut => b"Tang",
            Newa => b"Newa",
            MasaramGondi => b"Gonm",
            Nushu => b"Nshu",
            Soyombo => b"Soyo",
            ZanabazarSquare => b"Zanb",
            Dogra => b"Dogr",
            GunjalaGondi => b"Gong",
            HanifiRohingya => b"Rohg",
            Makasar => b"Maka",
            Medefaidrin => b"Medf",
            OldSogdian => b"Sogo",
            Sogdian => b"Sogd",
            Elymaic => b"Elym",
            Nandinagari => b"Nand",
            NyiakengPuachueHmong => b"Hmnp",
            Wancho => b"Wcho",
            Chorasmian => b"Chrs",
            DivesAkuru => b"Diak",
            KhitanSmallScript => b"Kits",
            Yezidi => b"Yezi",
        };
        c_int::from_be_bytes(*code)
    }

    /// Sets the script of the font, or `Err` on not supported.
    pub fn set(self) -> Result<()> {
        let ret = unsafe { bind::TTF_SetScript(self.into_raw()) };
        if ret != 0 {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(())
        }
    }
}
