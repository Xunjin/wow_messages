use std::convert::{TryFrom, TryInto};
use crate::world::tbc::DungeonDifficulty;
use crate::world::tbc::Map;
use crate::world::tbc::TransferAbortReason;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L45):
/// ```text
/// smsg SMSG_TRANSFER_ABORTED = 0x0040 {
///     Map map;
///     TransferAbortReason reason;
///     if (reason == INSUFFICIENT_EXPANSION_LEVEL
///         || reason == DIFFICULTY_NOT_AVAILABLE) {
///         DungeonDifficulty difficulty;
///     }
/// }
/// ```
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: SMSG_TRANSFER_ABORTED_TransferAbortReason,
}

impl crate::Message for SMSG_TRANSFER_ABORTED {
    const OPCODE: u32 = 0x0040;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        match &self.reason {
            SMSG_TRANSFER_ABORTED_TransferAbortReason::None => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::IsFull => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneIsInCombat => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int() as u8).to_le_bytes())?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                difficulty,
            } => {
                // difficulty: DungeonDifficulty
                w.write_all(&(difficulty.as_int() as u8).to_le_bytes())?;

            }
            SMSG_TRANSFER_ABORTED_TransferAbortReason::MissingDifficulty => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::InstanceIsFull => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::NotAllowed => {}
            SMSG_TRANSFER_ABORTED_TransferAbortReason::HasBind => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reason: TransferAbortReason
        let reason: TransferAbortReason = crate::util::read_u8_le(r)?.try_into()?;

        let reason_if = match reason {
            TransferAbortReason::None => SMSG_TRANSFER_ABORTED_TransferAbortReason::None,
            TransferAbortReason::IsFull => SMSG_TRANSFER_ABORTED_TransferAbortReason::IsFull,
            TransferAbortReason::NotFound => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotFound,
            TransferAbortReason::TooManyInstances => SMSG_TRANSFER_ABORTED_TransferAbortReason::TooManyInstances,
            TransferAbortReason::ZoneIsInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneIsInCombat,
            TransferAbortReason::InsufficientExpansionLevel => {
                // difficulty: DungeonDifficulty
                let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::InsufficientExpansionLevel {
                    difficulty,
                }
            }
            TransferAbortReason::DifficultyNotAvailable => {
                // difficulty: DungeonDifficulty
                let difficulty: DungeonDifficulty = crate::util::read_u8_le(r)?.try_into()?;

                SMSG_TRANSFER_ABORTED_TransferAbortReason::DifficultyNotAvailable {
                    difficulty,
                }
            }
            TransferAbortReason::MissingDifficulty => SMSG_TRANSFER_ABORTED_TransferAbortReason::MissingDifficulty,
            TransferAbortReason::ZoneInCombat => SMSG_TRANSFER_ABORTED_TransferAbortReason::ZoneInCombat,
            TransferAbortReason::InstanceIsFull => SMSG_TRANSFER_ABORTED_TransferAbortReason::InstanceIsFull,
            TransferAbortReason::NotAllowed => SMSG_TRANSFER_ABORTED_TransferAbortReason::NotAllowed,
            TransferAbortReason::HasBind => SMSG_TRANSFER_ABORTED_TransferAbortReason::HasBind,
        };

        Ok(Self {
            map,
            reason: reason_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_TRANSFER_ABORTED {}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + self.reason.size() // reason: SMSG_TRANSFER_ABORTED_TransferAbortReason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_TRANSFER_ABORTED_TransferAbortReason {
    None,
    IsFull,
    NotFound,
    TooManyInstances,
    ZoneIsInCombat,
    InsufficientExpansionLevel {
        difficulty: DungeonDifficulty,
    },
    DifficultyNotAvailable {
        difficulty: DungeonDifficulty,
    },
    MissingDifficulty,
    ZoneInCombat,
    InstanceIsFull,
    NotAllowed,
    HasBind,
}

impl Default for SMSG_TRANSFER_ABORTED_TransferAbortReason {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SMSG_TRANSFER_ABORTED_TransferAbortReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::IsFull => 1,
            Self::NotFound => 2,
            Self::TooManyInstances => 3,
            Self::ZoneIsInCombat => 5,
            Self::InsufficientExpansionLevel { .. } => 6,
            Self::DifficultyNotAvailable { .. } => 7,
            Self::MissingDifficulty => 8,
            Self::ZoneInCombat => 9,
            Self::InstanceIsFull => 10,
            Self::NotAllowed => 11,
            Self::HasBind => 12,
        }
    }

}

impl SMSG_TRANSFER_ABORTED_TransferAbortReason {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                1
            }
            Self::IsFull => {
                1
            }
            Self::NotFound => {
                1
            }
            Self::TooManyInstances => {
                1
            }
            Self::ZoneIsInCombat => {
                1
            }
            Self::InsufficientExpansionLevel {
                difficulty,
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::DifficultyNotAvailable {
                difficulty,
            } => {
                1
                + 1 // difficulty: DungeonDifficulty
            }
            Self::MissingDifficulty => {
                1
            }
            Self::ZoneInCombat => {
                1
            }
            Self::InstanceIsFull => {
                1
            }
            Self::NotAllowed => {
                1
            }
            Self::HasBind => {
                1
            }
        }
    }
}
