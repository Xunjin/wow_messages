use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::AchievementNameLinkType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm#L8):
/// ```text
/// smsg SMSG_SERVER_FIRST_ACHIEVEMENT = 0x0498 {
///     CString name;
///     Guid player;
///     u32 achievement;
///     AchievementNameLinkType link_type;
/// }
/// ```
pub struct SMSG_SERVER_FIRST_ACHIEVEMENT {
    pub name: String,
    pub player: Guid,
    pub achievement: u32,
    pub link_type: AchievementNameLinkType,
}

impl crate::Message for SMSG_SERVER_FIRST_ACHIEVEMENT {
    const OPCODE: u32 = 0x0498;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // link_type: AchievementNameLinkType
        w.write_all(&(self.link_type.as_int() as u8).to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=269).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0498, size: body_size as u32 });
        }

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // player: Guid
        let player = Guid::read(r)?;

        // achievement: u32
        let achievement = crate::util::read_u32_le(r)?;

        // link_type: AchievementNameLinkType
        let link_type: AchievementNameLinkType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            name,
            player,
            achievement,
            link_type,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SERVER_FIRST_ACHIEVEMENT {}

impl SMSG_SERVER_FIRST_ACHIEVEMENT {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 8 // player: Guid
        + 4 // achievement: u32
        + 1 // link_type: AchievementNameLinkType
    }
}
