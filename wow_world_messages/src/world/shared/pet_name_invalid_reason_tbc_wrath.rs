use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm#L6):
/// ```text
/// enum PetNameInvalidReason : u8 {
///     INVALID = 1;
///     NO_NAME = 2;
///     TOO_SHORT = 3;
///     TOO_LONG = 4;
///     MIXED_LANGUAGES = 6;
///     PROFANE = 7;
///     RESERVED = 8;
///     THREE_CONSECUTIVE = 11;
///     INVALID_SPACE = 12;
///     CONSECUTIVE_SPACES = 13;
///     RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS = 14;
///     RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END = 15;
///     DECLENSION_DOESNT_MATCH_BASE_NAME = 16;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetNameInvalidReason {
    Invalid,
    NoName,
    TooShort,
    TooLong,
    MixedLanguages,
    Profane,
    Reserved,
    ThreeConsecutive,
    InvalidSpace,
    ConsecutiveSpaces,
    RussianConsecutiveSilentCharacters,
    RussianSilentCharacterAtBeginningOrEnd,
    DeclensionDoesntMatchBaseName,
}

impl PetNameInvalidReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Invalid => 0x1,
            Self::NoName => 0x2,
            Self::TooShort => 0x3,
            Self::TooLong => 0x4,
            Self::MixedLanguages => 0x6,
            Self::Profane => 0x7,
            Self::Reserved => 0x8,
            Self::ThreeConsecutive => 0xb,
            Self::InvalidSpace => 0xc,
            Self::ConsecutiveSpaces => 0xd,
            Self::RussianConsecutiveSilentCharacters => 0xe,
            Self::RussianSilentCharacterAtBeginningOrEnd => 0xf,
            Self::DeclensionDoesntMatchBaseName => 0x10,
        }
    }

}

impl Default for PetNameInvalidReason {
    fn default() -> Self {
        Self::Invalid
    }
}

impl std::fmt::Display for PetNameInvalidReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => f.write_str("Invalid"),
            Self::NoName => f.write_str("NoName"),
            Self::TooShort => f.write_str("TooShort"),
            Self::TooLong => f.write_str("TooLong"),
            Self::MixedLanguages => f.write_str("MixedLanguages"),
            Self::Profane => f.write_str("Profane"),
            Self::Reserved => f.write_str("Reserved"),
            Self::ThreeConsecutive => f.write_str("ThreeConsecutive"),
            Self::InvalidSpace => f.write_str("InvalidSpace"),
            Self::ConsecutiveSpaces => f.write_str("ConsecutiveSpaces"),
            Self::RussianConsecutiveSilentCharacters => f.write_str("RussianConsecutiveSilentCharacters"),
            Self::RussianSilentCharacterAtBeginningOrEnd => f.write_str("RussianSilentCharacterAtBeginningOrEnd"),
            Self::DeclensionDoesntMatchBaseName => f.write_str("DeclensionDoesntMatchBaseName"),
        }
    }
}

impl TryFrom<u8> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Invalid),
            2 => Ok(Self::NoName),
            3 => Ok(Self::TooShort),
            4 => Ok(Self::TooLong),
            6 => Ok(Self::MixedLanguages),
            7 => Ok(Self::Profane),
            8 => Ok(Self::Reserved),
            11 => Ok(Self::ThreeConsecutive),
            12 => Ok(Self::InvalidSpace),
            13 => Ok(Self::ConsecutiveSpaces),
            14 => Ok(Self::RussianConsecutiveSilentCharacters),
            15 => Ok(Self::RussianSilentCharacterAtBeginningOrEnd),
            16 => Ok(Self::DeclensionDoesntMatchBaseName),
            v => Err(crate::errors::EnumError::new("PetNameInvalidReason", v as u32),)
        }
    }
}
