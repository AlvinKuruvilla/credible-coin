import sys
import os
import pandas as pd

if len(sys.argv) != 2:
    print("Usage: python3 dataset_processor.py <dir_path>")
    sys.exit(1)
path = sys.argv[1]
dfs = []

if not os.path.exists(path) or not os.path.isdir(path):
    print(f"The provided path '{path}' either doesn't exist or is not a directory.")
    sys.exit(1)

for entry in os.listdir(path):
    full_path = os.path.join(path, entry)
    if os.path.isfile(full_path):
        df = pd.read_csv(full_path, sep="\t", header=0)
        dfs.append(df)
combined_df = pd.concat(dfs, ignore_index=True)

satoshi_values = combined_df[["source_address", "satoshi"]]
script_dir = os.path.dirname(os.path.abspath(__file__))
# This will ensure that no matter from what relative location we run the script from
# we will always save to the folder in the same directory as the script
output_path = os.path.join(script_dir, "generated/exchange_secret.csv")

satoshi_values.to_csv(output_path, index=False)
