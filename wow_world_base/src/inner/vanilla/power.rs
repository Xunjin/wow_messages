use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L3):
/// ```text
/// enum Power : u8 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     HEALTH = 0xFE;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Power {
    /// mangoszero: The most common one, mobs usually have this or rage
    ///
    Mana,
    /// mangoszero: This is what warriors use to cast their spells
    ///
    Rage,
    /// mangoszero: Used by hunters after Cataclysm (4.x)
    ///
    Focus,
    /// mangoszero: Used by rouges to do their spells
    ///
    Energy,
    /// mangoszero: Hunter's pet's happiness affect their damage
    ///
    Happiness,
    /// mangoszero: Health, everyone has this (-2 as signed value)
    /// This might not actually be sent to the client.
    ///
    Health,
}

impl Default for Power {
    fn default() -> Self {
        Self::Mana
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mana => f.write_str("Mana"),
            Self::Rage => f.write_str("Rage"),
            Self::Focus => f.write_str("Focus"),
            Self::Energy => f.write_str("Energy"),
            Self::Happiness => f.write_str("Happiness"),
            Self::Health => f.write_str("Health"),
        }
    }
}

impl Power {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Mana => 0x0,
            Self::Rage => 0x1,
            Self::Focus => 0x2,
            Self::Energy => 0x3,
            Self::Happiness => 0x4,
            Self::Health => 0xfe,
        }
    }

}

impl TryFrom<u8> for Power {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Rage),
            2 => Ok(Self::Focus),
            3 => Ok(Self::Energy),
            4 => Ok(Self::Happiness),
            254 => Ok(Self::Health),
            v => Err(crate::errors::EnumError::new("Power", v as u32),)
        }
    }
}
