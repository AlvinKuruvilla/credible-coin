import pandas as pd
import collections
import os


def print_frequencies():
    df = pd.read_csv(
        "/Users/alvinkuruvilla/Dev/solvency-research/credible_coin/credible-coin/BigQuery Bitcoin Historical Data - outputs.csv"
    )
    address_series = df.iloc[:, 9]
    print(len(set(address_series.unique().tolist())))
    all_addresses = address_series.tolist()

    frequency = collections.Counter(all_addresses)

    d = dict(frequency)
    for k, v in d.items():
        if v > 1:
            print(k + " : " + str(v))


def write_new_csv(filepath: str):
    df = pd.read_csv(filepath)
    os.remove(filepath)
    sub = df[["addresses", "value"]]
    sub.to_csv(filepath)


write_new_csv(
    "/Users/alvinkuruvilla/Dev/solvency-research/credible_coin/credible-coin/BigQuery Bitcoin Historical Data - outputs.csv"
)
