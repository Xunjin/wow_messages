use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_highest_threat_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_highest_threat_update.wowm#L1):
/// ```text
/// struct ThreatUpdateUnit {
///     PackedGuid unit;
///     u32 threat;
/// }
/// ```
pub struct ThreatUpdateUnit {
    pub unit: Guid,
    pub threat: u32,
}

impl ThreatUpdateUnit {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // threat: u32
        w.write_all(&self.threat.to_le_bytes())?;

        Ok(())
    }
}

impl ThreatUpdateUnit {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // threat: u32
        let threat = crate::util::read_u32_le(r)?;

        Ok(Self {
            unit,
            threat,
        })
    }

}

impl ThreatUpdateUnit {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + 4 // threat: u32
    }
}
