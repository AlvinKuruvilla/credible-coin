import pandas as pd
import os
# from: https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/ZLBYTZ
df = pd.read_csv(os.path.join(os.getcwd(), "scripts", "5635771.tsv"), sep="\t")
satoshi_valuees = df[["source_address", "satoshi"]]
print(satoshi_valuees)
satoshi_valuees.to_csv("out.txt", sep=" ", index=False)
