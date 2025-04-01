import asyncio
import json
from anchorpy import Provider, Program, Wallet
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from anchorpy import Context
from solana.rpc.async_api import AsyncClient
from anchorpy import Idl

PROGRAM_ID = Pubkey.from_string("AeDRnzQh9VJswa71LWQrcP1R1AhT2bdtKmkjsCvyUzq6")
TREASURY_ACCOUNT = Keypair()
SYS_PROGRAM_ID = Pubkey.from_string("11111111111111111111111111111111")

# Load the IDL manually and convert to Idl object
with open("token_rewards.json") as f:
    idl = Idl.from_json(json.load(f))

program = Program(idl, PROGRAM_ID, provider)


async def main():
    client = AsyncClient("https://api.mainnet-beta.solana.com")
    wallet = Wallet.local()
    provider = Provider(client, wallet)

    # Load the IDL manually
    with open("token_rewards.json") as f:
        idl = json.load(f)

    program = Program(idl, PROGRAM_ID, provider)

    # Derive Treasury Account
    print(f"Treasury Account: {TREASURY_ACCOUNT.pubkey()}")

    # Send 'initialize' instruction
    try:
        tx = await program.rpc["initialize"](
            ctx=Context(
                accounts={
                    "treasuryAccount": TREASURY_ACCOUNT.pubkey(),
                    "authority": wallet.payer.pubkey(),
                    "systemProgram": SYS_PROGRAM_ID
                },
                signers=[wallet.payer, TREASURY_ACCOUNT]
            )
        )
        print(f"Transaction Signature: {tx}")
    except Exception as e:
        print(f"Transaction failed: {e}")

asyncio.run(main())
