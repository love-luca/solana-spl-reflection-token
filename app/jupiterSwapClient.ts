import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import fetch from "node-fetch";

const PROGRAM_ID = new PublicKey("AeDRnzQh9VJswa71LWQrcP1R1AhT2bdtKmkjsCvyUzq6");
const TREASURY_ACCOUNT = Keypair.generate();
const SYS_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");
const JUPITER_PROGRAM_ID = new PublicKey("JUPITER_PROGRAM_PUBLIC_KEY");

const connection = new Connection(clusterApiUrl("mainnet-beta"));
const wallet = Wallet.local();
const provider = new AnchorProvider(connection, wallet, {});

async function getJupiterSwapRoute(inputMint: string, outputMint: string, amount: number, slippage: number) {
    const response = await fetch(`https://quote-api.jup.ag/v1/quote?inputMint=${inputMint}&outputMint=${outputMint}&amount=${amount}&slippage=${slippage}`);
    const data = await response.json();
    return data;
}

async function main() {
    const idl = require("./solami_rewards.json");
    const program = new Program(idl, PROGRAM_ID, provider);

    console.log(`Treasury Account: ${TREASURY_ACCOUNT.publicKey}`);

    try {
        const route = await getJupiterSwapRoute("SOLAMI_MINT_ADDRESS", "SOL_MINT_ADDRESS", 1000000, 0.5);
        const tx = await program.methods.swapTaxedTokens(route)
            .accounts({
                treasuryAccount: TREASURY_ACCOUNT.publicKey,
                authority: wallet.payer.publicKey,
                systemProgram: SYS_PROGRAM_ID,
                jupiterProgram: JUPITER_PROGRAM_ID
            })
            .signers([wallet.payer, TREASURY_ACCOUNT])
            .rpc();

        console.log(`Transaction Signature: ${tx}`);
    } catch (e) {
        console.error(`Transaction failed: ${e}`);
    }
}

main().catch(console.error);
