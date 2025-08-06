import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChainFundMe } from "../target/types/chain_fund_me";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert } from "chai";
describe("chain-fund-me", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();

  const creator = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  const feeWallet = Keypair.generate();
  const contributor = anchor.web3.Keypair.generate();
  let campaignTokenAccount;
  let feeWalletTokenAccount;
  let contributorTokenAccount;

  before(async () => {
    const stablecoin_mint = await createMint(
      connection,
      creator.payer,
      creator.publicKey,
      null,
      6

    );
    console.log("Stablecoin mint created:", stablecoin_mint.toBase58());

    // Store the stablecoin mint in a global variable for use in tests
    globalThis.stablecoin_mint = stablecoin_mint;

    const mint = await createMint(
      provider.connection,
      creator.payer,
      creator.publicKey,
      null,
      6
    );

    let contributorTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      creator.payer,
      stablecoin_mint,
      contributor.publicKey
    );

    campaignTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      creator.payer,
      stablecoin_mint,
      creator.publicKey,
      true
    );

    feeWalletTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoin_mint,
      feeWallet.publicKey,
      true
    );

    await mintTo(
      provider.connection,
      creator.payer,
      mint,
      contributorTokenAccount.address,
      creator.publicKey,
      1_000_000_000 // 1000 tokens
    );
  });
  const stablecoin_mint = globalThis.stablecoin_mint as PublicKey;

  const program = anchor.workspace.chainFundMe as Program<ChainFundMe>;
  const factoryKeypair = anchor.web3.Keypair.generate();
  const start_time = Math.floor(Date.now() / 1000) + 60; // Start in 1 minute
  const end_time = start_time + 3600; // End in 1 hour
  const [campaignPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("campaign"),
      creator.publicKey.toBuffer(),
      new anchor.BN(start_time).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  const [contributionPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("contribution"), creator.publicKey.toBuffer(), campaignPda.toBuffer()],
    program.programId
  );
  const [spenderPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("spender"), contributor.publicKey.toBuffer()],
    program.programId
  );

  const [factoryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory")],
    program.programId
  );


  it("Initialize factory", async () => {
    // Add your test here.
    const tx = await program.methods.initializeFactory(20, stablecoin_mint, feeWallet.publicKey).rpc();
    console.log("Initialize Factory tx signature", tx);
  });

  it("Create Campaign", async () => {
    const start_time = Math.floor(Date.now() / 1000) + 60; // Start in 1 minute
    const end_time = start_time + 3600; // End in 1 hour
    const metadata_uri = "https://example.com/campaign_metadata.json";
    const other_token_mints: anchor.web3.PublicKey[] = [];

    const tx = await program.methods.createCampaign(
      new anchor.BN(start_time),
      new anchor.BN(end_time),
      metadata_uri,
      other_token_mints
    ).accounts({
      factory: factoryKeypair.publicKey,
      creator: creator.publicKey,
      //@ts-ignore
      campaign: campaignPda,
      systemProgram: SystemProgram.programId

    }).rpc();
    console.log("Create Campaign tx signature", tx);
  });

  it("Contribute to Campaign", async () => {
    const contributionAmount = new anchor.BN(1000000); // 1 stablecoin (assuming 6 decimals)

    const tx = await program.methods.contribute(contributionAmount, false).accounts({
      campaign: campaignPda,
      //@ts-ignore
      contribution: contributionPda,
      contributor: contributor.publicKey,
      factory: factoryPda,
      feeWallet: feeWallet.publicKey,
      campaignToken: campaignTokenAccount.address,
      contributorToken: contributorTokenAccount.address,
      feeWalletToken: feeWalletTokenAccount.address,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      spender: spenderPda,
    }).signers([factoryKeypair]).rpc();
    const contribution = await program.account.contribution.fetch(contributionPda);
    assert.equal(contribution.tokenAmount.toNumber(), 500_000_000);
  });

  it("Withdraw", async()=>{
    const tx = await program.methods.withdraw(true).accounts({
      factory: factoryPda,
      campaign: campaignPda,
      owner: creator.publicKey,
      campaignToken: campaignTokenAccount.address,
      spender: spenderPda,
      ownerToken: contributorTokenAccount.address,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("Withdraw tx signature", tx);
  })



});