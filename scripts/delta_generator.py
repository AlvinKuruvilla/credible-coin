import sys
import os
import pandas as pd
import numpy as np

if len(sys.argv) != 3:
    print("Usage: python3 delta_generator.py <exchange_secrets_file_path> <row_count>")
    sys.exit(1)
path = sys.argv[1]
requested_rows = int(sys.argv[2])
dfs = []

if not os.path.exists(path) or not os.path.isfile(path):
    print(f"The provided path '{path}' either doesn't exist or is not a file.")
    sys.exit(1)
df = pd.read_csv(path)
max_rows = df.shape[0]
if requested_rows > max_rows:
    print(f"Wanted '{requested_rows}' rows but the max is '{max_rows}'.")
    print("Help: perhaps generate a larger csv of exchange secrets")
    sys.exit(1)
addresses = df[["source_address"]].head(requested_rows)
# Generate a random integer between -100 and 100 for each entry in addresses
random_deltas = np.random.randint(-100, 101, size=len(addresses))
addresses["delta"] = random_deltas
script_dir = os.path.dirname(os.path.abspath(__file__))
# This will ensure that no matter from what relative location we run the script from
# we will always save to the folder in the same directory as the script
output_path = os.path.join(script_dir, "generated/public_ledger.csv")
addresses.to_csv(output_path, index=False)
