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
    #[msg("The provided randomness account is invalid.")]
    InvalidRandomnessAccount,
    #[msg("A winner has already been selected for this lottery.")]
    WinnerAlreadySelected,
    #[msg("No winner has been selected yet for this lottery.")]
    WinnerNotSelected,
    #[msg("You do not own a ticket for this lottery.")]
    NoTicketOwned,
    #[msg("The prize for this lottery has already been claimed.")]
    PrizeAlreadyClaimed,
    #[msg("The treasury has no fees to withdraw.")]
    NoFees,
}
