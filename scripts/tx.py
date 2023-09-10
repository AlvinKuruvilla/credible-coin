import pandas as pd
import os

df = pd.read_csv(os.path.join(os.getcwd(), "scripts", "5635771.tsv"), sep="\t")
satoshi_valuees = df["satoshi"]
print(satoshi_valuees)
satoshi_valuees.to_csv("out.txt", sep=" ", index=False)
