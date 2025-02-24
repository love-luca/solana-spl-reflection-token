import json
from solders.keypair import Keypair

# Load the keypair JSON file
with open("solami_rewards-keypair.json", "r") as f:
    secret_key = json.load(f)

# Generate the keypair and extract the public key
keypair = Keypair.from_bytes(bytes(secret_key))
print("Program ID:", keypair.pubkey())
