use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm#L15):
/// ```text
/// struct InspectTalentGear {
///     u32 item;
/// }
/// ```
pub struct InspectTalentGear {
    pub item: u32,
}

impl InspectTalentGear {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        Ok(())
    }
}

impl InspectTalentGear {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
        })
    }

}
