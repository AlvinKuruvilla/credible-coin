import sys
import random
import csv


def progressbar(it, prefix="", size=60, out=sys.stdout):  # Python3.6+
    count = len(it)

    def show(j):
        x = int(size * j / count)
        print(
            f"{prefix}[{u'█'*x}{('.'*(size-x))}] {j}/{count}",
            end="\r",
            file=out,
            flush=True,
        )

    show(0)
    for i, item in enumerate(it):
        yield item
        show(i + 1)
    print("\n", flush=True, file=out)


def generate_bitcoin_address():
    address = ""
    for i in range(25, 35):
        character = random.choice(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        )
        address += character
    return address


def generate_bitcoin_value():
    value = random.randint(1, 100000000)
    return value


def write_to_csv(address, value):
    with open("bitcoin_addresses.csv", "a") as csvfile:
        writer = csv.writer(csvfile, delimiter=",")
        if csvfile.tell() == 0:
            writer.writerow(["address", "value"])
        writer.writerow([address, value])


def main():
    number_of_pairs = int(sys.argv[1])
    for i in progressbar(range(number_of_pairs)):
        address = generate_bitcoin_address()
        value = generate_bitcoin_value()
        write_to_csv(address, value)


if __name__ == "__main__":
    main()
