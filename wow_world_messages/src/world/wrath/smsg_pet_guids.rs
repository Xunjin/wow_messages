use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_guids.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_guids.wowm#L1):
/// ```text
/// smsg SMSG_PET_GUIDS = 0x04AA {
///     u32 amount_of_guids;
///     Guid[amount_of_guids] guids;
/// }
/// ```
pub struct SMSG_PET_GUIDS {
    pub guids: Vec<Guid>,
}

impl crate::Message for SMSG_PET_GUIDS {
    const OPCODE: u32 = 0x04aa;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_guids: u32
        w.write_all(&(self.guids.len() as u32).to_le_bytes())?;

        // guids: Guid[amount_of_guids]
        for i in self.guids.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04AA, size: body_size as u32 });
        }

        // amount_of_guids: u32
        let amount_of_guids = crate::util::read_u32_le(r)?;

        // guids: Guid[amount_of_guids]
        let mut guids = Vec::with_capacity(amount_of_guids as usize);
        for i in 0..amount_of_guids {
            guids.push(Guid::read(r)?);
        }

        Ok(Self {
            guids,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PET_GUIDS {}

impl SMSG_PET_GUIDS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_guids: u32
        + self.guids.iter().fold(0, |acc, _| acc + 8) // guids: Guid[amount_of_guids]
    }
}
