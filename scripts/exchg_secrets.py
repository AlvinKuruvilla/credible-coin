import sys
import hashlib
import shutil
import binascii
import os
import random
import pandas as pd
import ecdsa
import base58
import yaml


def generate_bitcoin_address():
    ecdsaPrivateKey = ecdsa.SigningKey.generate(curve=ecdsa.SECP256k1)
    # print("ECDSA Private Key: ", ecdsaPrivateKey.to_string().hex())
    ecdsaPublicKey = "04" + ecdsaPrivateKey.get_verifying_key().to_string().hex()
    # print("ECDSA Public Key: ", ecdsaPublicKey)
    hash256FromECDSAPublicKey = hashlib.sha256(
        binascii.unhexlify(ecdsaPublicKey)
    ).hexdigest()
    # print("SHA256(ECDSA Public Key): ", hash256FromECDSAPublicKey)
    ridemp160FromHash256 = hashlib.new(
        "ripemd160", binascii.unhexlify(hash256FromECDSAPublicKey)
    )
    # print("RIDEMP160(SHA256(ECDSA Public Key)): ", ridemp160FromHash256.hexdigest())
    prependNetworkByte = "00" + ridemp160FromHash256.hexdigest()
    # print(
    #     "Prepend Network Byte to RIDEMP160(SHA256(ECDSA Public Key)): ",
    #     prependNetworkByte,
    # )
    hash = prependNetworkByte
    for x in range(1, 3):
        hash = hashlib.sha256(binascii.unhexlify(hash)).hexdigest()
        # print("\t|___>SHA256 #", x, " : ", hash)
    checksum = hash[:8]
    # print("Checksum(first 4 bytes): ", checksum)
    appendChecksum = prependNetworkByte + checksum
    # print("Append Checksum to RIDEMP160(SHA256(ECDSA Public Key)): ", appendChecksum)
    bitcoinAddress = base58.b58encode(binascii.unhexlify(appendChecksum))
    # print("Bitcoin Address: ", bitcoinAddress.decode("utf8"))
    return bitcoinAddress.decode("utf8")


def generate_value():
    return random.getrandbits(32)


def next_power_of_2(n):
    """
    Returns the next power of 2 that is greater than or equal to n.

    Parameters:
    n (int): Input number

    Returns:
    int: Next power of 2 greater than or equal to n
    """
    if n <= 0:
        raise ValueError("Input must be a positive integer")

    # Decrement n since we need to find the next power of 2
    n -= 1

    # Set all bits to the right of the leftmost set bit
    n |= n >> 1
    n |= n >> 2
    n |= n >> 4
    n |= n >> 8
    n |= n >> 16

    # Increment n to get the next power of 2
    return n + 1


def is_power_of_two(n):
    return n > 0 and (n & (n - 1)) == 0


def generate_unique_addresses(length):
    address_set = set()
    while len(address_set) < length:
        address = generate_bitcoin_address()
        address_set.add(address)
    return list(address_set)


def generate_unique_values(length):
    value_set = set()
    while len(value_set) < length:
        value = generate_value()
        value_set.add(value)
    return list(value_set)


def write_address_file(final_df):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    # # This will ensure that no matter from what relative location we run the script from
    # # we will always save to the folder in the same directory as the script
    output_path = os.path.join(script_dir, "generated/exchange_secret.csv")
    final_df.to_csv(output_path, index=False)
    with open(
        os.path.join(script_dir, "generated/out.txt"), "w", encoding="utf-8"
    ) as f:
        for element in final_df["satoshi"]:
            f.write("%s\n" % str(element))


def finalize_address_value_pairings(directory):
    dfs = []

    for entry in os.listdir(directory):
        full_path = os.path.join(directory, entry)
        if os.path.isfile(full_path):
            df = pd.read_csv(full_path, header=0)
            dfs.append(df)

    combined_df = pd.concat(dfs, ignore_index=True)
    unique_addresses = combined_df["source_address"].unique()
    unique_values = generate_unique_values(len(unique_addresses))

    temp_df = pd.DataFrame(
        {"source_address": unique_addresses, "satoshi": unique_values}
    )
    current_row_count = len(unique_addresses)
    project_config = yaml.safe_load(
        open(os.path.join(os.getcwd(), "credible_config.yaml"), "r")
    )

    needed_row_count = next_power_of_2(project_config["exchange_secret_set_size"])

    extra_addresses = generate_unique_addresses(needed_row_count - current_row_count)
    extra_values = generate_unique_values(needed_row_count - current_row_count)

    extra_rows = list(zip(extra_addresses, extra_values))
    small_df = pd.DataFrame(extra_rows, columns=["source_address", "satoshi"])

    final_df = pd.concat([temp_df, small_df], ignore_index=True)
    assert is_power_of_two(
        final_df.shape[0]
    ), "Resulting DataFrame does not have a number of rows that is a power of two."
    return final_df


def main(directory):
    final_df = finalize_address_value_pairings(directory)
    write_address_file(final_df)
    project_config = yaml.safe_load(
        open(os.path.join(os.getcwd(), "credible_config.yaml"), "r")
    )
    # Should be able to overwrite the file if it exists
    shutil.copy(
        os.path.join(os.path.dirname(os.path.abspath(__file__)), "generated/out.txt"),
        project_config["emp_root_path"],
    )


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("Usage: python3 exchg_secrets.py <directory_path>")
    if not os.path.exists(sys.argv[1]) or not os.path.isdir(sys.argv[1]):
        sys.exit(f"Error: The path {sys.argv[1]} does not exist or is not a directory.")
    project_config = yaml.safe_load(
        open(os.path.join(os.getcwd(), "credible_config.yaml"), "r")
    )
    if project_config["exchange_secret_set_size"] < 1024:
        sys.exit("Error: exchange_secret_set_size must be at least 1024.")
    if is_power_of_two(project_config["exchange_secret_set_size"]) == False:
        sys.exit("Error: exchange_secret_set_size must be a power of 2.")
    print(
        "Generating exchange secrets of size",
        project_config["exchange_secret_set_size"],
        "...",
    )
    main(sys.argv[1])
