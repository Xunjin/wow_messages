use std::convert::{TryFrom, TryInto};
use crate::world::tbc::WorldState;
use crate::world::tbc::Area;
use crate::world::tbc::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_init_world_states.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_init_world_states.wowm#L1):
/// ```text
/// smsg SMSG_INIT_WORLD_STATES = 0x02C2 {
///     Map map;
///     Area area;
///     u16 amount_of_states;
///     WorldState[amount_of_states] states;
/// }
/// ```
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub states: Vec<WorldState>,
}

impl crate::Message for SMSG_INIT_WORLD_STATES {
    const OPCODE: u32 = 0x02c2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=524298).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C2, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(r)?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::read(r)?);
        }

        Ok(Self {
            map,
            area,
            states,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_INIT_WORLD_STATES {}

impl SMSG_INIT_WORLD_STATES {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 4 // area: Area
        + 2 // amount_of_states: u16
        + self.states.len() * 8 // states: WorldState[amount_of_states]
    }
}
