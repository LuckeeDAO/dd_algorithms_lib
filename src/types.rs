//! Common types and data structures used throughout the library.

/// Unique identifier for participants
pub type ParticipantId = u64;

/// Voting power amount
pub type VotingPower = u128;

/// Timestamp (seconds since epoch)
pub type Timestamp = u64;

// (Removed ProposalStatus enum and its Default implementation as it's not needed currently)

/// Fair division result
#[derive(Debug, Clone)]
pub struct FairDivisionResult {
    /// Allocation for each participant
    pub allocations: &'static [i128],
    /// Total value being divided
    pub total_value: i128,
    /// Whether the division is fair (sum = 0)
    pub is_fair: bool,
}

/// Random selection result
#[derive(Debug, Clone)]
pub struct RandomSelectionResult {
    /// Selected participant indices
    pub selected_indices: &'static [usize],
    /// Number of participants
    pub total_participants: usize,
    /// Number of selections made
    pub selections_made: usize,
    /// Whether all selections are unique
    pub is_unique: bool,
}
