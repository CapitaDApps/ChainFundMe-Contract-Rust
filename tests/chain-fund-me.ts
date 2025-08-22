import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChainFundMe } from "../target/types/chain_fund_me";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";

describe("chain-fund-me-comprehensive", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const creator = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const feeWallet = Keypair.generate();
  const contributor = Keypair.generate();

  // Token mints
  let stablecoinMint: PublicKey;
  let extraMint1: PublicKey; // e.g., USDT
  let extraMint2: PublicKey; // e.g., USDC

  // Token accounts for stablecoin
  let campaignStablecoinAccount: any;
  let feeWalletStablecoinAccount: any;
  let contributorStablecoinAccount: any;
  let creatorStablecoinAccount: any;

  // Token accounts for extra mint 1
  let campaignExtraMint1Account: any;
  let feeWalletExtraMint1Account: any;
  let contributorExtraMint1Account: any;
  let creatorExtraMint1Account: any;

  // Token accounts for extra mint 2
  let campaignExtraMint2Account: any;
  let feeWalletExtraMint2Account: any;
  let contributorExtraMint2Account: any;
  let creatorExtraMint2Account: any;

  let campaignPda: PublicKey;
  let contributionPda: PublicKey;
  let spenderPda: PublicKey;
  let factoryPda: PublicKey;

  const program = anchor.workspace.ChainFundMe as Program<ChainFundMe>;
  let factoryId = 1;
  let start_time: anchor.BN;
  let end_time: anchor.BN;

  before(async () => {
    // Calculate timing - start well in the past to avoid any clock differences
    const now = Math.floor(Date.now() / 1000);
    start_time = new anchor.BN(now);
    end_time = new anchor.BN(now + 3600);

    // Find factory PDA
    [factoryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("factory"), new anchor.BN(factoryId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    [spenderPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("spender"), contributor.publicKey.toBuffer()],
      program.programId
    );
    [campaignPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("campaign"),
        creator.publicKey.toBuffer(),
        start_time.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    [contributionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("contribution"),
        contributor.publicKey.toBuffer(),
        campaignPda.toBuffer(),
      ],
      program.programId
    );
    stablecoinMint = await createMint(
      connection,
      creator.payer,
      creator.publicKey,
      null,
      6 // USDC decimals
    );
    console.log("Stablecoin mint (USDC):", stablecoinMint.toBase58());

    extraMint1 = await createMint(
      connection,
      creator.payer,
      creator.publicKey,
      null,
      6 
    );
    console.log("Extra mint 1 (USDT):", extraMint1.toBase58());

    extraMint2 = await createMint(
      connection,
      creator.payer,
      creator.publicKey,
      null,
      9 // SOL-like decimals
    );
    console.log("Extra mint 2 (Custom Token A):", extraMint2.toBase58());

    // Create token accounts for all participants for stablecoin
    contributorStablecoinAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      contributor.publicKey
    );

    feeWalletStablecoinAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      feeWallet.publicKey
    );

    creatorStablecoinAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      creator.publicKey
    );

    // Create token accounts for extra mint 1
    contributorExtraMint1Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint1,
      contributor.publicKey
    );

    feeWalletExtraMint1Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint1,
      feeWallet.publicKey
    );

    creatorExtraMint1Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint1,
      creator.publicKey
    );

    // Create token accounts for extra mint 2
    contributorExtraMint2Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint2,
      contributor.publicKey
    );

    feeWalletExtraMint2Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint2,
      feeWallet.publicKey
    );

    creatorExtraMint2Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint2,
      creator.publicKey
    );

    // Mint tokens to contributor for all mints
    await mintTo(
      connection,
      creator.payer,
      stablecoinMint,
      contributorStablecoinAccount.address,
      creator.publicKey,
      1_000_000_000
    );

    await mintTo(
      connection,
      creator.payer,
      extraMint1,
      contributorExtraMint1Account.address,
      creator.publicKey,
      500_000_000
    );

    await mintTo(
      connection,
      creator.payer,
      extraMint2,
      contributorExtraMint2Account.address,
      creator.publicKey,
      2_000_000_000
    );

    console.log("All token mints and accounts created successfully");
  });

  it("Fund contributor with SOL", async () => {
    const sig = await connection.requestAirdrop(
      contributor.publicKey,
      3 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig, "confirmed");

    const balance = await connection.getBalance(contributor.publicKey);
    console.log("Contributor SOL balance:", balance / LAMPORTS_PER_SOL, "SOL");
  });

  it("Initialize factory", async () => {
    const tx = await program.methods
      .initializeFactory(
        new anchor.BN(1),
        20, // 2% platform fee
        stablecoinMint,
        feeWallet.publicKey,
        [{
          mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
          allowed: true
        },
        {
          mint: "So11111111111111111111111111111111111111111",
          allowed: true
        },
        {
          mint: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
          allowed: true
        },
        {
          mint: "GhE4sh64jawtzUmeQWRgkN3XrzcWN4pib5g5RcKMbonk",
          allowed: true
        },
      ] 
      )
      .accounts({
        owner: creator.publicKey,
        //@ts-ignore
        factory: factoryPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Initialize Factory tx:", tx);
  });

  it("Create Campaign with multiple token mints", async () => {
    const metadata_uri = "https://example.com/multi_token_campaign.json";
    const other_token_mints: PublicKey[] = [extraMint1, extraMint2];

    // Create campaign token accounts for all supported mints
    campaignStablecoinAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      stablecoinMint,
      campaignPda,
      true
    );

    campaignExtraMint1Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint1,
      campaignPda,
      true
    );

    campaignExtraMint2Account = await getOrCreateAssociatedTokenAccount(
      connection,
      creator.payer,
      extraMint2,
      campaignPda,
      true
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
    console.log("Campaign created with PDA:", campaignPda.toBase58());
  });

  it("Initialize Spender", async () => {
    const tx = await program.methods
      .initializeSpender() // Rust instruction
      .accounts({
        spender: spenderPda,
        contributor: contributor.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor])
      .rpc();

    console.log("Initialize Spender tx:", tx);
  });

  it("Verify campaign timing and start contributions", async () => {
    const campaignAccount = await program.account.campaign.fetch(campaignPda);
    const currentTime = Math.floor(Date.now() / 1000);

    console.log("Current time:", currentTime);
    console.log("Campaign start time:", campaignAccount.startTime.toString());
    console.log("Campaign end time:", campaignAccount.endTime.toString());
    console.log("Campaign has started:", currentTime >= campaignAccount.startTime.toNumber());
    console.log("Time difference:", currentTime - campaignAccount.startTime.toNumber(), "seconds");

    // Ensure campaign has started with a good margin
    assert.isTrue(currentTime >= campaignAccount.startTime.toNumber(), "Campaign should have started");

    // Additional wait to ensure any clock synchronization issues are resolved
    console.log("Waiting additional 3 seconds for clock sync...");
    await new Promise((resolve) => setTimeout(resolve, 10000));
  });


  it("Contribute SOL to Campaign", async () => {
    const contributionAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL); // 0.5 SOL

    const initialBalance = await connection.getBalance(contributor.publicKey);
    console.log("Contributor SOL balance before:", initialBalance / LAMPORTS_PER_SOL, "SOL");

    const tx = await program.methods
      .contributeSol(contributionAmount)
      .accounts({
        campaign: campaignPda,
        //@ts-ignore
        contribution: contributionPda,
        contributor: contributor.publicKey,
        factory: factoryPda,
        feeWallet: feeWallet.publicKey,
        spender: spenderPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor])
      .rpc();

    console.log("SOL Contribute tx:", tx);

    const campaignBalance = await connection.getBalance(campaignPda);
    console.log("Campaign SOL balance:", campaignBalance / LAMPORTS_PER_SOL, "SOL");
  });

  it("Contribute Stablecoin (USDC) to Campaign", async () => {
    const contributionAmount = new anchor.BN(100_000_000); // 100 USDC

    const tx = await program.methods
      .contributeToken(contributionAmount)
      .accounts({
        campaign: campaignPda,
        //@ts-ignore
        contribution: contributionPda,
        contributor: contributor.publicKey,
        contributorToken: contributorStablecoinAccount.address,
        factory: factoryPda,
        feeWalletToken: feeWalletStablecoinAccount.address, // Fixed: was missing feeWalletToken
        campaignToken: campaignStablecoinAccount.address,
        spender: spenderPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor])
      .rpc();

    console.log("Stablecoin Contribute tx:", tx);

    const campaignTokenBalance = await getAccount(connection, campaignStablecoinAccount.address);
    console.log("Campaign Stablecoin balance:", campaignTokenBalance.amount.toString());
  });

  it("Contribute Extra Mint 1 (USDT) to Campaign", async () => {
    const contributionAmount = new anchor.BN(75_000_000); // 75 USDT

    const tx = await program.methods
      .contributeToken(contributionAmount)
      .accounts({
        campaign: campaignPda,
        //@ts-ignore
        contribution: contributionPda,
        contributor: contributor.publicKey,
        contributorToken: contributorExtraMint1Account.address,
        factory: factoryPda,
        feeWalletToken: feeWalletExtraMint1Account.address,
        campaignToken: campaignExtraMint1Account.address, // Fixed: was pointing to creator account
        spender: spenderPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor]) // Fixed: added signers back
      .rpc();

    console.log("Extra Mint 1 Contribute tx:", tx);

    const campaignTokenBalance = await getAccount(connection, campaignExtraMint1Account.address);
    console.log("Campaign Extra Mint 1 balance:", campaignTokenBalance.amount.toString());
  });

  it("Contribute Extra Mint 2 to Campaign", async () => {
    const contributionAmount = new anchor.BN(500_000_000); // 0.5 tokens

    const tx = await program.methods
      .contributeToken(contributionAmount)
      .accounts({
        campaign: campaignPda,
        //@ts-ignore
        contribution: contributionPda,
        contributor: contributor.publicKey, // Fixed: was just contributor instead of contributor.publicKey
        contributorToken: contributorExtraMint2Account.address,
        factory: factoryPda,
        feeWalletToken: feeWalletExtraMint2Account.address,
        campaignToken: campaignExtraMint2Account.address, // Fixed: was pointing to creator account
        spender: spenderPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor]) // Fixed: added signers back
      .rpc();

    console.log("Extra Mint 2 Contribute tx:", tx);

    const campaignTokenBalance = await getAccount(connection, campaignExtraMint2Account.address);
    console.log("Campaign Extra Mint 2 balance:", campaignTokenBalance.amount.toString());
  });

  it("Withdraw SOL from Campaign", async () => {
    const campaignAccount = await program.account.campaign.fetch(campaignPda);
    const startTimeBn = campaignAccount.startTime;

    console.log("Withdrawing SOL...");
    const initialCreatorBalance = await connection.getBalance(creator.publicKey);
    const initialCampaignBalance = await connection.getBalance(campaignPda);

    const tx = await program.methods
      .withdraw()
      .accounts({
        factory: factoryPda,
        //@ts-ignore
        campaign: campaignPda,
        owner: creator.publicKey,
        spender: spenderPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("SOL Withdraw tx:", tx);

    const finalCreatorBalance = await connection.getBalance(creator.publicKey);
    const finalCampaignBalance = await connection.getBalance(campaignPda);

    console.log("Creator SOL balance change:", (finalCreatorBalance - initialCreatorBalance) / LAMPORTS_PER_SOL, "SOL");
    console.log("Campaign SOL balance after:", finalCampaignBalance / LAMPORTS_PER_SOL, "SOL");
  });

  it("Withdraw All Tokens from Campaign", async () => {
    console.log("Withdrawing all tokens...");

    // Get initial balances
    const initialStablecoinBalance = await getAccount(connection, creatorStablecoinAccount.address);
    const initialExtraMint1Balance = await getAccount(connection, creatorExtraMint1Account.address);
    const initialExtraMint2Balance = await getAccount(connection, creatorExtraMint2Account.address);

    console.log("Creator initial token balances:");
    console.log("Stablecoin:", initialStablecoinBalance.amount.toString());
    console.log("Extra Mint 1:", initialExtraMint1Balance.amount.toString());
    console.log("Extra Mint 2:", initialExtraMint2Balance.amount.toString());

    const tx = await program.methods
      .withdraw()
      .accounts({
        factory: factoryPda,
        //@ts-ignore
        campaign: campaignPda,
        owner: creator.publicKey,
        spender: spenderPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts([
        // Campaign token accounts (from)
        { pubkey: campaignStablecoinAccount.address, isSigner: false, isWritable: true },
        // Creator token accounts (to)
        { pubkey: creatorStablecoinAccount.address, isSigner: false, isWritable: true },
        // Extra Mint 1
        { pubkey: campaignExtraMint1Account.address, isSigner: false, isWritable: true },
        { pubkey: creatorExtraMint1Account.address, isSigner: false, isWritable: true },
        // Extra Mint 2
        { pubkey: campaignExtraMint2Account.address, isSigner: false, isWritable: true },
        { pubkey: creatorExtraMint2Account.address, isSigner: false, isWritable: true },
      ])
      .rpc();

    console.log("Token Withdraw tx:", tx);

    // Get final balances
    const finalStablecoinBalance = await getAccount(connection, creatorStablecoinAccount.address);
    const finalExtraMint1Balance = await getAccount(connection, creatorExtraMint1Account.address);
    const finalExtraMint2Balance = await getAccount(connection, creatorExtraMint2Account.address);

    console.log("Creator final token balances:");
    console.log("Stablecoin:", finalStablecoinBalance.amount.toString());
    console.log("Extra Mint 1:", finalExtraMint1Balance.amount.toString());
    console.log("Extra Mint 2:", finalExtraMint2Balance.amount.toString());

    // Verify campaign token accounts are empty
    const campaignStablecoinFinal = await getAccount(connection, campaignStablecoinAccount.address);
    const campaignExtraMint1Final = await getAccount(connection, campaignExtraMint1Account.address);
    const campaignExtraMint2Final = await getAccount(connection, campaignExtraMint2Account.address);

    console.log("Campaign final token balances (should be 0):");
    console.log("Stablecoin:", campaignStablecoinFinal.amount.toString());
    console.log("Extra Mint 1:", campaignExtraMint1Final.amount.toString());
    console.log("Extra Mint 2:", campaignExtraMint2Final.amount.toString());

    // Assert that tokens were transferred
    assert.equal(campaignStablecoinFinal.amount.toString(), "0");
    assert.equal(campaignExtraMint1Final.amount.toString(), "0");
    assert.equal(campaignExtraMint2Final.amount.toString(), "0");
  });

  it("Verify final campaign state", async () => {
    const campaignAccount = await program.account.campaign.fetch(campaignPda);
    const spenderAccount = await program.account.spender.fetch(spenderPda);

    console.log("Final Campaign State:");
    console.log("- Is withdrawal points minted:", campaignAccount.isWithdrawalPointsMinted);
    console.log("- Campaign paused:", campaignAccount.isPaused);

    console.log("Final Spender State:");
    console.log("- Points earned:", spenderAccount.pointsEarned.toString());

    // Verify withdrawal points were awarded
    assert.equal(campaignAccount.isWithdrawalPointsMinted, true);
  });
});