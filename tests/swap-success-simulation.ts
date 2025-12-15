import * as anchor from "@coral-xyz/anchor";
import {BN, Program} from "@coral-xyz/anchor";
import {SwapSuccessSimulation} from "../target/types/swap_success_simulation";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction} from "@solana/web3.js";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";

describe("swap-success-simulation", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = provider.connection
  const payer = provider.wallet.payer

  const program = anchor.workspace.swapSuccessSimulation as Program<SwapSuccessSimulation>;

  let vault: Keypair
  let wallet_b: Keypair

  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM_ID,
  }

  before("Initialize...", async () => {
    vault = Keypair.generate()
    wallet_b = Keypair.generate()

    for (const user of [vault, wallet_b]) {
      await connection.requestAirdrop(
          user.publicKey,
          2 * LAMPORTS_PER_SOL
      );
    }

    const token_mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        null,
        6
    );

    const vault_token_account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        token_mint,
        vault.publicKey
    );
    const wallet_b_token_account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        token_mint,
        wallet_b.publicKey
    );

    await mintTo(
        connection,
        payer,
        token_mint,
        vault_token_account.address,
        payer,
        1_000_000_000_000
    );

    await mintTo(
        connection,
        payer,
        token_mint,
        wallet_b_token_account.address,
        payer,
        1_000_000_000_000
    );

    accounts.vault = vault.publicKey
    accounts.tokenMint = token_mint
    accounts.vaultTokenAccount = vault_token_account.address
    accounts.walletB = wallet_b.publicKey
    accounts.wallet_b_token_account = wallet_b_token_account.address
  })

  it("Swap success simulate", async ()=>{
    const input_amount = new BN(1_000_000_000)
    const min_profit = new BN(10_000_000)
    // const min_profit = new BN(1_000_000_000)
    accounts.walletA = PublicKey.findProgramAddressSync(
        [
          Buffer.from("wallet_seed_a"),
          accounts.vault.toBuffer(),
        ],
        program.programId
    )[0]

    accounts.walletATokenAccount = getAssociatedTokenAddressSync(
        accounts.tokenMint,
        accounts.walletA,
        true,
        TOKEN_PROGRAM_ID,
    )

    const ix1 = await program.methods
        .sendToWallet(input_amount, min_profit)
        .accounts({...accounts})
        .instruction()

    const ix2 = await program.methods
        .swapSimulate(true)
        .accounts({...accounts})
        .instruction()

    const ix3 = await program.methods
        .sendToVault()
        .accounts({...accounts})
        .instruction()

    const tx = new Transaction()

    tx.add(ix1, ix2, ix3)

    const txSig = await provider.sendAndConfirm(tx, [vault, wallet_b])
    const txInfo = await connection.getTransaction(txSig, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });

    console.log(txSig, txInfo?.meta?.logMessages)

    const vaultBalance = await connection.getTokenAccountBalance(accounts.vaultTokenAccount)
    console.log("Vault Token Balance", vaultBalance.value.amount)
  })
});

describe("swap-failed-simulation", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = provider.connection
  const payer = provider.wallet.payer

  const program = anchor.workspace.swapSuccessSimulation as Program<SwapSuccessSimulation>;

  let vault: Keypair
  let wallet_b: Keypair

  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM_ID,
  }

  before("Initialize...", async () => {
    vault = Keypair.generate()
    wallet_b = Keypair.generate()

    for (const user of [vault, wallet_b]) {
      await connection.requestAirdrop(
          user.publicKey,
          2 * LAMPORTS_PER_SOL
      );
    }

    const token_mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        null,
        6
    );

    const vault_token_account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        token_mint,
        vault.publicKey
    );
    const wallet_b_token_account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        token_mint,
        wallet_b.publicKey
    );

    await mintTo(
        connection,
        payer,
        token_mint,
        vault_token_account.address,
        payer,
        1_000_000_000_000
    );

    await mintTo(
        connection,
        payer,
        token_mint,
        wallet_b_token_account.address,
        payer,
        1_000_000_000_000
    );

    accounts.vault = vault.publicKey
    accounts.tokenMint = token_mint
    accounts.vaultTokenAccount = vault_token_account.address
    accounts.walletB = wallet_b.publicKey
    accounts.wallet_b_token_account = wallet_b_token_account.address
  })

  it("Swap failed simulate", async ()=>{
    const input_amount = new BN(1_000_000_000)
    const min_profit = new BN(10_000_000)
    accounts.walletA = PublicKey.findProgramAddressSync(
        [
          Buffer.from("wallet_seed_a"),
          accounts.vault.toBuffer(),
        ],
        program.programId
    )[0]

    accounts.walletATokenAccount = getAssociatedTokenAddressSync(
        accounts.tokenMint,
        accounts.walletA,
        true,
        TOKEN_PROGRAM_ID,
    )

    const ix1 = await program.methods
        .sendToWallet(input_amount, min_profit)
        .accounts({...accounts})
        .instruction()

    const ix2 = await program.methods
        .swapSimulate(false)
        .accounts({...accounts})
        .instruction()

    const ix3 = await program.methods
        .sendToVault()
        .accounts({...accounts})
        .instruction()

    const tx = new Transaction()

    tx.add(ix1, ix2, ix3)

    const txSig = await provider.sendAndConfirm(tx, [vault, wallet_b])
    const txInfo = await connection.getTransaction(txSig, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });

    console.log(txSig, txInfo?.meta?.logMessages)

    const vaultBalance = await connection.getTokenAccountBalance(accounts.vaultTokenAccount)
    console.log("Vault Token Balance", vaultBalance.value.amount)
  })
});