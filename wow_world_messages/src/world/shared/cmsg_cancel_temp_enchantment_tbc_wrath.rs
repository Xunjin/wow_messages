use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_cancel_temp_enchantment.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_cancel_temp_enchantment.wowm#L1):
/// ```text
/// cmsg CMSG_CANCEL_TEMP_ENCHANTMENT = 0x0379 {
///     u32 slot;
/// }
/// ```
pub struct CMSG_CANCEL_TEMP_ENCHANTMENT {
    pub slot: u32,
}

impl crate::Message for CMSG_CANCEL_TEMP_ENCHANTMENT {
    const OPCODE: u32 = 0x0379;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0379, size: body_size as u32 });
        }

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_CANCEL_TEMP_ENCHANTMENT {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CANCEL_TEMP_ENCHANTMENT {}
