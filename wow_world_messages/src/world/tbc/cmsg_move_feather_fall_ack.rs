use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_feather_fall_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_feather_fall_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_FEATHER_FALL_ACK = 0x02CF {
///     Guid guid;
///     u32 movement_counter;
///     MovementInfo info;
///     u32 apply;
/// }
/// ```
pub struct CMSG_MOVE_FEATHER_FALL_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
    pub apply: u32,
}

impl crate::Message for CMSG_MOVE_FEATHER_FALL_ACK {
    const OPCODE: u32 = 0x02cf;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // apply: u32
        w.write_all(&self.apply.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // apply: u32
        let apply = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
            apply,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_MOVE_FEATHER_FALL_ACK {}

impl CMSG_MOVE_FEATHER_FALL_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // apply: u32
    }
}
