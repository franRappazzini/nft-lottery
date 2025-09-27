use anchor_lang::error_code;

#[error_code]
pub enum DappError {
    #[msg("The lottery has reached the maximum number of tickets.")]
    MaxTicketsReached,
    #[msg("The lottery has ended; the ticket purchase deadline has passed.")]
    OutOfTime,
    #[msg("Randomness not yet resolved.")]
    RandomnessNotResolved,
    #[msg("Randomness already revealed.")]
    RandomnessAlreadyRevealed,
}
