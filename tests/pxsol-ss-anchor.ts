import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PxsolSsAnchor } from "../target/types/pxsol_ss_anchor";

describe("pxsol-ss-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.pxsolSsAnchor as Program<PxsolSsAnchor>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const wallet = provider.wallet as anchor.Wallet;
  const walletPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("data"), wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  it("Init with content and then update (grow and shrink)", async () => {
    // Airdrop SOL to fresh authority to fund rent and tx fees
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(
      wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    ), "confirmed");

    const poemInitial = Buffer.from("");
    const poemEnglish = Buffer.from("The quick brown fox jumps over the lazy dog");
    const poemChinese = Buffer.from("片云天共远, 永夜月同孤.");
    const walletPdaData = async (): Promise<Buffer<ArrayBuffer>> => {
      let walletPdaData = await program.account.data.fetch(walletPda);
      return Buffer.from(walletPdaData.data);
    }

    await program.methods
      .init()
      .accounts({
        user: wallet.publicKey,
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    if (!(await walletPdaData()).equals(poemInitial)) throw new Error("mismatch");

    await program.methods
      .update(poemEnglish)
      .accounts({
        user: wallet.publicKey,
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    if (!(await walletPdaData()).equals(poemEnglish)) throw new Error("mismatch");

    await program.methods
      .update(poemChinese)
      .accounts({
        user: wallet.publicKey,
        userPda: walletPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    if (!(await walletPdaData()).equals(poemChinese)) throw new Error("mismatch");
  });
});
