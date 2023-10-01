import sys
import hashlib
import binascii
import os
import random
import pandas as pd
import ecdsa
import base58


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


def pad_fake_data(length):
    rows = []
    address_set = set()

    while len(address_set) < length:
        address = generate_bitcoin_address()
        if address not in address_set:
            address_set.add(address)
    value_set = make_value_set(length)

    for addr, value in zip(list(address_set), list(value_set)):
        rows.append([addr, value])
    return rows


def make_value_set(length):
    value_set = set()
    while len(value_set) < length:
        val = generate_value()
        if val not in value_set and val not in value_set:
            value_set.add(val)
    return list(value_set)


if __name__ == "__main__":
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
    combined_addresses = combined_df["source_address"].tolist()
    combined_values = make_value_set(len(combined_addresses))
    assert len(combined_values) == len(combined_addresses)
    temp_df = df = pd.DataFrame(
        {"source_address": combined_addresses, "satoshi": combined_values}
    )
    current_row_count = combined_df.shape[0]
    needed_row_count = next_power_of_2(current_row_count)
    extra_rows = pad_fake_data(needed_row_count - current_row_count)
    cols = ["source_address", "satoshi"]
    small_df = pd.DataFrame(extra_rows, columns=cols)

    script_dir = os.path.dirname(os.path.abspath(__file__))
    # # This will ensure that no matter from what relative location we run the script from
    # # we will always save to the folder in the same directory as the script
    output_path = os.path.join(script_dir, "generated/exchange_secret.csv")
    df = pd.concat([temp_df, small_df])
    assert is_power_of_two(df.shape[0]), "Result df does not have power of two rows"
    df.to_csv(output_path, index=False)
    with open(
        os.path.join(script_dir, "generated/out.txt"), "w", encoding="utf-8"
    ) as f:
        data = list(df["satoshi"])
        for element in data:
            f.write("%s\n" % str(element))
    # TODO: the generated out.txt above should be copied to the emp-folder by
    # reading the YAML configuration file
