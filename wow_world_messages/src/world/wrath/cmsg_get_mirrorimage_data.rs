use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_get_mirrorimage_data.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_get_mirrorimage_data.wowm#L7):
/// ```text
/// cmsg CMSG_GET_MIRRORIMAGE_DATA = 0x0401 {
///     Guid target;
/// }
/// ```
pub struct CMSG_GET_MIRRORIMAGE_DATA {
    pub target: Guid,
}

impl crate::Message for CMSG_GET_MIRRORIMAGE_DATA {
    const OPCODE: u32 = 0x0401;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0401, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GET_MIRRORIMAGE_DATA {}
