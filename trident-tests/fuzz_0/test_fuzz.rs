use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // perform any initialization here, this method will be executed
        // at start of each iteration
        let mut tx = InitializeConfigTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("InitializeConfig"));
    }

    #[flow]
    fn flow1(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
        let mut tx = CreateLotteryTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("CreateLottery"));

        let mut tx_2 = BuyTicketTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx_2, Some("BuyTicket"));
    }

    #[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
        let mut tx = WithdrawFeesTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("WithdrawFees"));
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
