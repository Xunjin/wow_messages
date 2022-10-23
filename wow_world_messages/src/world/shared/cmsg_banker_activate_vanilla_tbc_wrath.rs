use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_banker_activate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_banker_activate.wowm#L3):
/// ```text
/// cmsg CMSG_BANKER_ACTIVATE = 0x01B7 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_BANKER_ACTIVATE {
    pub guid: Guid,
}

impl crate::Message for CMSG_BANKER_ACTIVATE {
    const OPCODE: u32 = 0x01b7;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BANKER_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_BANKER_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BANKER_ACTIVATE {}
