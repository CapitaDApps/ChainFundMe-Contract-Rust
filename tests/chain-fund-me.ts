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

    // Derive campaign PDA
    [campaignPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("campaign"),
        creator.publicKey.toBuffer(),
        start_time.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    console.log("Campaign PDA:", campaignPda.toBase58());

    // Campaign token account (PDA owns it)
    campaignTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      campaignPda,
      true // allow owner to be PDA
    );

    // Contribution PDA
    [contributionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("contribution"),
        contributor.publicKey.toBuffer(),
        campaignPda.toBuffer(),
      ],
      program.programId
    );

    // Spender PDA
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

    // Wait until campaign starts
    await new Promise((res) => setTimeout(res, 6000));
  });

  it("Contribute to Campaign", async () => {
    // Airdrop SOL to contributor for fees
    const sig = await connection.requestAirdrop(
      contributor.publicKey,
      1_000_000_000
    );
    await connection.confirmTransaction(sig, "confirmed");

    const contributionAmount = new anchor.BN(1_000_000); 

    const tx = await program.methods
      .contribute(contributionAmount, true)
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

    const contribution = await program.account.contribution.fetch(
      contributionPda
    );
    assert.equal(contribution.tokenAmount.toNumber(), 1_000_000);
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
      .withdraw(true)
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
