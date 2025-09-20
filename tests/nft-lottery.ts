import * as anchor from "@coral-xyz/anchor";

import { NftLottery } from "../target/types/nft_lottery";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

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
    const tx = await program.methods
      .createLottery()
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("createLottery signature", tx);
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
