use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::CalendarStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_rsvp.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_rsvp.wowm#L16):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_RSVP = 0x0432 {
///     Guid event;
///     Guid invite_id;
///     (u32)CalendarStatus status;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_RSVP {
    pub event: Guid,
    pub invite_id: Guid,
    pub status: CalendarStatus,
}

impl crate::Message for CMSG_CALENDAR_EVENT_RSVP {
    const OPCODE: u32 = 0x0432;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // status: CalendarStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0432, size: body_size as u32 });
        }

        // event: Guid
        let event = Guid::read(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // status: CalendarStatus
        let status: CalendarStatus = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            event,
            invite_id,
            status,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_EVENT_RSVP {}
