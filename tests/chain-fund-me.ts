import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChainFundMe } from "../target/types/chain_fund_me";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";

describe("chain-fund-me", () => {

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const creator = provider.wallet as anchor.Wallet;
  const connection = provider.connection;


  const feeWallet = Keypair.generate();
  const contributor = Keypair.generate();

  let stablecoinMint: PublicKey;
  let campaignTokenAccount: any;
  let feeWalletTokenAccount: any;
  let contributorTokenAccount: any;
  let creatorTokenAccount: any;

  let campaignPda: PublicKey;
  let contributionPda: PublicKey;
  let spenderPda: PublicKey;
  let factoryPda: PublicKey;

  const program = anchor.workspace.ChainFundMe as Program<ChainFundMe>;

  before(async () => {

    [factoryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("factory")],
      program.programId
    );

    stablecoinMint = await createMint(
      connection,
      creator.payer,
      creator.publicKey,
      null,
      6 // decimals
    );
    console.log("Stablecoin mint:", stablecoinMint.toBase58());

    // Create contributor token account
    contributorTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      contributor.publicKey
    );

    // Create fee wallet token account
    feeWalletTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      feeWallet.publicKey
    );

    creatorTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      creator.publicKey
    );

    // Mint tokens to contributor
    await mintTo(
      connection,
      creator.payer,
      stablecoinMint,
      contributorTokenAccount.address,
      creator.publicKey,
      1_000_000_000 // 1000 tokens
    );
  });


  it("Fund contributor with more SOL", async () => {
    const sig = await connection.requestAirdrop(
      contributor.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig, "confirmed");

    const balance = await connection.getBalance(contributor.publicKey);
    console.log("Contributor SOL balance:", balance / 1_000_000_000, "SOL");
  });

  it("Initialize factory", async () => {
    const tx = await program.methods
      .initializeFactory(20, stablecoinMint, feeWallet.publicKey)
      .accounts({
        owner: creator.publicKey,
        //@ts-ignore
        factory: factoryPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Initialize Factory tx:", tx);
  });

  it("Create Campaign", async () => {
    const metadata_uri = "https://example.com/campaign_metadata.json";
    const other_token_mints: PublicKey[] = [];

    const now = Math.floor(Date.now() / 1000);
    const start_time = new anchor.BN(now + 5);
    const end_time = new anchor.BN(now + 3600);

    [campaignPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("campaign"),
        creator.publicKey.toBuffer(),
        start_time.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    campaignTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      campaignPda,
      true
    );

    [contributionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("contribution"),
        contributor.publicKey.toBuffer(),
        campaignPda.toBuffer(),
      ],
      program.programId
    );
    [spenderPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("spender"), contributor.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createCampaign(start_time, end_time, metadata_uri, other_token_mints)
      .accounts({
        factory: factoryPda,
        creator: creator.publicKey,
        //@ts-ignore
        campaign: campaignPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Create Campaign tx:", tx);

    await new Promise((res) => setTimeout(res, 6000));
  });


  it("Initialize spender", async () => {

    const tx = await program.methods
      .initializeSpender()
      .accounts({
        spender: spenderPda,
        contributor: contributor.publicKey,
        systemProgram: SystemProgram.programId,
      }).signers([contributor])
      .rpc();

    console.log("Create Campaign tx:", tx);

    await new Promise((res) => setTimeout(res, 6000));
  });

  it("Contribute to token Campaign", async () => {

    const contributionAmount = new anchor.BN(1_000_000_000);

    const balance = await connection.getBalance(contributor.publicKey)

    console.log("Balance is HERE : ", balance);

    const tx = await program.methods
      .contribute(contributionAmount, false)
      .accounts({
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
        systemProgram: SystemProgram.programId,
        spender: spenderPda,
      })
      .signers([contributor])
      .rpc();

    console.log("Contribute tx:", tx);

    const campaign = await program.account.campaign.fetch(
      campaignPda.toString()
    );
    // assert.equal(campaign.tokenAmount.toNumber(), 1_000_000_000);
    console.log("Campaign: ", campaign)
    console.log("Campaign balance: ", await connection.getBalance(campaignPda))
  });

  it("Withdraw", async () => {
    const campaignAccount = await program.account.campaign.fetch(campaignPda);
    const startTimeBn = campaignAccount.startTime;
    console.log("Campaign start time:", startTimeBn.toString());
    console.log("Campaign token account:", campaignTokenAccount);

    const [withdrawCampaignPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("campaign"),
        creator.publicKey.toBuffer(),
        startTimeBn.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const tx = await program.methods
      .withdraw(false)
      .accountsStrict({
        factory: factoryPda,
        campaign: withdrawCampaignPda,
        owner: creator.publicKey,
        campaignToken: campaignTokenAccount.address,
        spender: spenderPda,
        ownerToken: creatorTokenAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw tx:", tx);
  });
});
