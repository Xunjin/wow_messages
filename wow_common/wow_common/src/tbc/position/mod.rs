mod positions;

use crate::shared::{position, tbc_starter_positions, vanilla_starter_positions};
pub use positions::*;
use wow_world_base::tbc::{Map, PlayerRace};

position!();

pub fn get_starting_position(race: PlayerRace) -> Position {
    match race {
        PlayerRace::Human => HUMAN_START_POSITION,
        PlayerRace::Orc => ORC_START_POSITION,
        PlayerRace::Dwarf => DWARF_START_POSITION,
        PlayerRace::NightElf => NIGHT_ELF_START_POSITION,
        PlayerRace::Undead => UNDEAD_START_POSITION,
        PlayerRace::Tauren => TAUREN_START_POSITION,
        PlayerRace::Gnome => GNOME_START_POSITION,
        PlayerRace::Troll => TROLL_START_POSITION,
        PlayerRace::BloodElf => BLOOD_ELF_START_POSITION,
        PlayerRace::Draenei => DRAENEI_START_POSITION,
    }
}

vanilla_starter_positions!();

tbc_starter_positions!();