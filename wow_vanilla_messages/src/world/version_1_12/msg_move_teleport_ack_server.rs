use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::MovementInfo;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Can be response to [`CMSG_TELEPORT_TO_UNIT`](crate::world::version_1_12::CMSG_TELEPORT_TO_UNIT).
/// Can also be a response to [`MSG_MOVE_TELEPORT_ACK_Client`](crate::world::version_1_12::MSG_MOVE_TELEPORT_ACK_Client) after being sent.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L11):
/// ```text
/// smsg MSG_MOVE_TELEPORT_ACK_Server = 0x00C7 {
///     PackedGuid guid;
///     u32 movement_counter;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_ACK_Server {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
}

impl ServerMessage for MSG_MOVE_TELEPORT_ACK_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c7;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
        })
    }

}

impl MSG_MOVE_TELEPORT_ACK_Server {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
    }
}
