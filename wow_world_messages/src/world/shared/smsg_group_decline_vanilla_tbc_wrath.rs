use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_decline.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_decline.wowm#L3):
/// ```text
/// smsg SMSG_GROUP_DECLINE = 0x0074 {
///     CString name;
/// }
/// ```
pub struct SMSG_GROUP_DECLINE {
    pub name: String,
}

impl crate::Message for SMSG_GROUP_DECLINE {
    const OPCODE: u32 = 0x0074;

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

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0074, size: body_size as u32 });
        }

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GROUP_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_GROUP_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_GROUP_DECLINE {}

impl SMSG_GROUP_DECLINE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}
