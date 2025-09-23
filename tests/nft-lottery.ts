import * as anchor from "@coral-xyz/anchor";

import { ComputeBudgetProgram } from "@solana/web3.js";
import { NftLottery } from "../target/types/nft_lottery";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getSimulationComputeUnits } from "@solana-developers/helpers";

// solana-test-validator --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --reset

describe("nft-lottery", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(provider);

  const program = anchor.workspace.nftLottery as Program<NftLottery>;

  it("Is initialized!", async () => {
    const feeBps = 500; // 5% fee

    const tx = await program.methods.initializeConfig(feeBps).rpc();
    console.log("initializeConfigsignature", tx);
  });

  it("Create lottery!", async () => {
    const ticketPrice = bn(100_000_000); // 0.1 SOL

    const tx = await program.methods
      .createLottery(ticketPrice)
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("createLottery signature", tx);
  });

  it("Buy ticket!", async () => {
    const lotteryId = bn(0);

    const ix = await program.methods
      .buyTicket(lotteryId)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    const computeUnits = await getSimulationComputeUnits(connection, [ix], wallet.publicKey, []);

    console.log("Estimated compute units:", computeUnits);

    const computeUnitIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: computeUnits,
    });

    const buildTx = new anchor.web3.Transaction().add(computeUnitIx, ix);

    const tx = await anchor.web3.sendAndConfirmTransaction(connection, buildTx, [wallet.payer], {
      skipPreflight: true,
    });

    console.log("buyTicket signature", tx);
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
