import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolamiRewards } from "../target/types/solami_rewards";

describe("solami_rewards", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolamiRewards as Program<SolamiRewards>;

  it("Is initialized!", async () => {
    const treasuryAccount = anchor.web3.Keypair.generate();

    const tx = await program.methods.initialize().accounts({
      treasuryAccount: treasuryAccount.publicKey,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([treasuryAccount]).rpc();

    console.log("Your transaction signature", tx);
  });
});
