use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_criteria_deleted.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_criteria_deleted.wowm#L1):
/// ```text
/// smsg SMSG_CRITERIA_DELETED = 0x049E {
///     u32 criteria_id;
/// }
/// ```
pub struct SMSG_CRITERIA_DELETED {
    pub criteria_id: u32,
}

impl crate::Message for SMSG_CRITERIA_DELETED {
    const OPCODE: u32 = 0x049e;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // criteria_id: u32
        w.write_all(&self.criteria_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049E, size: body_size as u32 });
        }

        // criteria_id: u32
        let criteria_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            criteria_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CRITERIA_DELETED {}
