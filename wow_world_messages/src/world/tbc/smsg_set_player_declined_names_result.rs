use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_set_player_declined_names_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_set_player_declined_names_result.wowm#L1):
/// ```text
/// smsg SMSG_SET_PLAYER_DECLINED_NAMES_RESULT = 0x0419 {
///     u32 result;
///     Guid guid;
/// }
/// ```
pub struct SMSG_SET_PLAYER_DECLINED_NAMES_RESULT {
    pub result: u32,
    pub guid: Guid,
}

impl crate::Message for SMSG_SET_PLAYER_DECLINED_NAMES_RESULT {
    const OPCODE: u32 = 0x0419;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: u32
        w.write_all(&self.result.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0419, size: body_size as u32 });
        }

        // result: u32
        let result = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            result,
            guid,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_SET_PLAYER_DECLINED_NAMES_RESULT {}
