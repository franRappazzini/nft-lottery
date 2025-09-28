import * as anchor from "@coral-xyz/anchor";

import { ON_DEMAND_MAINNET_PID, Queue, Randomness, asV0Tx } from "@switchboard-xyz/on-demand";

import { ComputeBudgetProgram } from "@solana/web3.js";
import { NftLottery } from "../target/types/nft_lottery";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getSimulationComputeUnits } from "@solana-developers/helpers";

describe("nft-lottery", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(provider);

  const program = anchor.workspace.nftLottery as Program<NftLottery>;
  let sbProgram: anchor.Program;

  const rngKp = anchor.web3.Keypair.generate();

  before("Loading Switchboard IDL", async () => {
    const sbIdl = (await anchor.Program.fetchIdl(ON_DEMAND_MAINNET_PID, {
      connection: new anchor.web3.Connection("https://api.mainnet-beta.solana.com"),
    })) as anchor.Idl;

    console.log("Switchboard IDL:", sbIdl);

    sbProgram = new anchor.Program(sbIdl, provider);
  });

  it.skip("Is initialized!", async () => {
    const feeBps = 500; // 5% fee

    const tx = await program.methods.initializeConfig(feeBps).rpc();
    console.log("initializeConfigsignature", tx);
  });

  it.skip("Create lottery!", async () => {
    const ticketPrice = bn(100_000_000); // 0.1 SOL
    const maxTickets = bn(10); // Max 10 tickets

    const tx = await program.methods
      .createLottery(ticketPrice, maxTickets)
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("createLottery signature", tx);
  });

  it.skip("Buy ticket!", async () => {
    await buyTicket();
    await buyTicket();
    await buyTicket();
    await buyTicket();
    await buyTicket();
  });

  it.skip("Commit randomness & select winner!", async () => {
    const lotteryId = bn(0);
    // const queue = new anchor.web3.PublicKey("FfD96yeXs4cxZshoPPSKhSPgVQxLAJUT3gefgh84m1Di");
    const queue = new anchor.web3.PublicKey("A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w");
    // const queue = ON_DEMAND_MAINNET_QUEUE;

    const queueAccount = new Queue(sbProgram, queue);
    console.log("Queue account", queue.toString());
    try {
      await queueAccount.loadData();
    } catch (err) {
      console.log("Queue account not found:", err);
      process.exit(1);
    }

    const [randomness, ix] = await Randomness.create(sbProgram, rngKp, queue);
    console.log("Created randomness account..");
    console.log("Randomness account", randomness.pubkey.toBase58());
    console.log("rkp account", rngKp.publicKey.toBase58());
    const createRandomnessTx = await asV0Tx({
      connection: connection,
      ixs: [ix],
      payer: wallet.publicKey,
      signers: [wallet.payer, rngKp],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    const blockhashContext = await connection.getLatestBlockhashAndContext();

    const createRandomnessSignature = await connection.sendTransaction(createRandomnessTx);
    await connection.confirmTransaction({
      signature: createRandomnessSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight,
    });
    console.log(
      "Transaction Signature for randomness account creation: ",
      createRandomnessSignature
    );

    let sbCommitIx: anchor.web3.TransactionInstruction;
    try {
      sbCommitIx = await randomness.commitIx(queue);
    } catch (err) {
      console.error("Error creating commit ix:", err);
      process.exit(1);
    }

    const commitIx = await program.methods
      .commitRandomness(lotteryId)
      .accounts({
        randomnessAccount: randomness.pubkey,
      })
      .instruction();

    const commitTx = await asV0Tx({
      connection: sbProgram.provider.connection,
      ixs: [sbCommitIx, commitIx],
      payer: wallet.publicKey,
      signers: [wallet.payer],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    const commitSignature = await connection.sendTransaction(commitTx);
    await connection.confirmTransaction({
      signature: commitSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight,
    });
    console.log("Transaction Signature for commit: ", commitSignature);

    const sbRevealIx = await randomness.revealIx();
    const revealIx = await program.methods.selectWinner(lotteryId).instruction();

    const revealTx = await asV0Tx({
      connection: sbProgram.provider.connection,
      ixs: [sbRevealIx, revealIx],
      payer: wallet.publicKey,
      signers: [wallet.payer],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    const revealSignature = await connection.sendTransaction(revealTx);
    await connection.confirmTransaction({
      signature: commitSignature,
      blockhash: blockhashContext.value.blockhash,
      lastValidBlockHeight: blockhashContext.value.lastValidBlockHeight,
    });
    console.log("Transaction Signature revealTx", revealSignature);
  });

  it("Claim prize!", async () => {
    const lotteryId = bn(0);

    const [lotteryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lottery"), lotteryId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const lotteryAccount = await program.account.lottery.fetch(lotteryPda);
    console.log("Lottery account:", lotteryAccount);

    const tx = await program.methods
      .claimPrize(lotteryId)
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("claimPrize tx signature:", tx);
  });

  it("Withdraw fees!", async () => {
    const tx = await program.methods.withdrawFees().rpc();
    console.log("withdrawFees tx signature:", tx);
  });

  async function buyTicket() {
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
  }
});

function bn(n: number) {
  return new anchor.BN(n);
}
