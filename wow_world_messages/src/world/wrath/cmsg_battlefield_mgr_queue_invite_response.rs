use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_battlefield_mgr_queue_invite_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_battlefield_mgr_queue_invite_response.wowm#L1):
/// ```text
/// cmsg CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE = 0x04E2 {
///     u32 battle_id;
///     Bool accepted;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {
    pub battle_id: u32,
    pub accepted: bool,
}

impl crate::Message for CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {
    const OPCODE: u32 = 0x04e2;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // accepted: Bool
        w.write_all(u8::from(self.accepted).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E2, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(r)?;

        // accepted: Bool
        let accepted = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            battle_id,
            accepted,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {}
