/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_command_result.wowm#L1):
/// ```text
/// enum ArenaTeamCommandError : u32 {
///     ARENA_TEAM_INTERNAL = 0x01;
///     ALREADY_IN_ARENA_TEAM = 0x02;
///     ALREADY_IN_ARENA_TEAM_S = 0x03;
///     INVITED_TO_ARENA_TEAM = 0x04;
///     ALREADY_INVITED_TO_ARENA_TEAM_S = 0x05;
///     ARENA_TEAM_NAME_INVALID = 0x06;
///     ARENA_TEAM_NAME_EXISTS_S = 0x07;
///     ARENA_TEAM_LEADER_LEAVE_S = 0x08;
///     ARENA_TEAM_PLAYER_NOT_IN_TEAM = 0x09;
///     ARENA_TEAM_PLAYER_NOT_IN_TEAM_SS = 0x0A;
///     ARENA_TEAM_PLAYER_NOT_FOUND_S = 0x0B;
///     ARENA_TEAM_NOT_ALLIED = 0x0C;
///     ARENA_TEAM_IGNORING_YOU_S = 0x13;
///     ARENA_TEAM_TARGET_TOO_LOW_S = 0x15;
///     ARENA_TEAM_TOO_MANY_MEMBERS_S = 0x16;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ArenaTeamCommandError {
    ArenaTeamInternal,
    AlreadyInArenaTeam,
    AlreadyInArenaTeamS,
    InvitedToArenaTeam,
    AlreadyInvitedToArenaTeamS,
    ArenaTeamNameInvalid,
    ArenaTeamNameExistsS,
    /// This value also has the name `ARENA_TEAM_PERMISSIONS`.
    ///
    ArenaTeamLeaderLeaveS,
    ArenaTeamPlayerNotInTeam,
    ArenaTeamPlayerNotInTeamSs,
    ArenaTeamPlayerNotFoundS,
    ArenaTeamNotAllied,
    ArenaTeamIgnoringYouS,
    ArenaTeamTargetTooLowS,
    ArenaTeamTooManyMembersS,
}

impl ArenaTeamCommandError {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::ArenaTeamInternal => 0x1,
            Self::AlreadyInArenaTeam => 0x2,
            Self::AlreadyInArenaTeamS => 0x3,
            Self::InvitedToArenaTeam => 0x4,
            Self::AlreadyInvitedToArenaTeamS => 0x5,
            Self::ArenaTeamNameInvalid => 0x6,
            Self::ArenaTeamNameExistsS => 0x7,
            Self::ArenaTeamLeaderLeaveS => 0x8,
            Self::ArenaTeamPlayerNotInTeam => 0x9,
            Self::ArenaTeamPlayerNotInTeamSs => 0xa,
            Self::ArenaTeamPlayerNotFoundS => 0xb,
            Self::ArenaTeamNotAllied => 0xc,
            Self::ArenaTeamIgnoringYouS => 0x13,
            Self::ArenaTeamTargetTooLowS => 0x15,
            Self::ArenaTeamTooManyMembersS => 0x16,
        }
    }

}

impl Default for ArenaTeamCommandError {
    fn default() -> Self {
        Self::ArenaTeamInternal
    }
}

impl std::fmt::Display for ArenaTeamCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArenaTeamInternal => f.write_str("ArenaTeamInternal"),
            Self::AlreadyInArenaTeam => f.write_str("AlreadyInArenaTeam"),
            Self::AlreadyInArenaTeamS => f.write_str("AlreadyInArenaTeamS"),
            Self::InvitedToArenaTeam => f.write_str("InvitedToArenaTeam"),
            Self::AlreadyInvitedToArenaTeamS => f.write_str("AlreadyInvitedToArenaTeamS"),
            Self::ArenaTeamNameInvalid => f.write_str("ArenaTeamNameInvalid"),
            Self::ArenaTeamNameExistsS => f.write_str("ArenaTeamNameExistsS"),
            Self::ArenaTeamLeaderLeaveS => f.write_str("ArenaTeamLeaderLeaveS"),
            Self::ArenaTeamPlayerNotInTeam => f.write_str("ArenaTeamPlayerNotInTeam"),
            Self::ArenaTeamPlayerNotInTeamSs => f.write_str("ArenaTeamPlayerNotInTeamSs"),
            Self::ArenaTeamPlayerNotFoundS => f.write_str("ArenaTeamPlayerNotFoundS"),
            Self::ArenaTeamNotAllied => f.write_str("ArenaTeamNotAllied"),
            Self::ArenaTeamIgnoringYouS => f.write_str("ArenaTeamIgnoringYouS"),
            Self::ArenaTeamTargetTooLowS => f.write_str("ArenaTeamTargetTooLowS"),
            Self::ArenaTeamTooManyMembersS => f.write_str("ArenaTeamTooManyMembersS"),
        }
    }
}

impl TryFrom<u32> for ArenaTeamCommandError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ArenaTeamInternal),
            2 => Ok(Self::AlreadyInArenaTeam),
            3 => Ok(Self::AlreadyInArenaTeamS),
            4 => Ok(Self::InvitedToArenaTeam),
            5 => Ok(Self::AlreadyInvitedToArenaTeamS),
            6 => Ok(Self::ArenaTeamNameInvalid),
            7 => Ok(Self::ArenaTeamNameExistsS),
            8 => Ok(Self::ArenaTeamLeaderLeaveS),
            9 => Ok(Self::ArenaTeamPlayerNotInTeam),
            10 => Ok(Self::ArenaTeamPlayerNotInTeamSs),
            11 => Ok(Self::ArenaTeamPlayerNotFoundS),
            12 => Ok(Self::ArenaTeamNotAllied),
            19 => Ok(Self::ArenaTeamIgnoringYouS),
            21 => Ok(Self::ArenaTeamTargetTooLowS),
            22 => Ok(Self::ArenaTeamTooManyMembersS),
            v => Err(crate::errors::EnumError::new("ArenaTeamCommandError", v as u64),)
        }
    }
}
