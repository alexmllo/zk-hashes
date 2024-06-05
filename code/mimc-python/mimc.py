from math import gcd, ceil, log
from sys import argv
from Crypto.Hash import SHAKE256


class Mimc:
    def __init__(self, p: int, nonce: bytes) -> None:
        self.nonce = nonce
        self.p = p
        self.d = self.smallest_permutation()
        self.r = self.number_of_rounds()
        self.c = self.generate_constants()


    def smallest_permutation(self) -> int:
        d = 3
        for i in range(self.p):
            if gcd(d, self.p - 1) == 1:
                return d
            d += 2

        return 0

    def number_of_rounds(self) -> int:
        return ceil(log(self.p, self.d))

    def generate_constants(self) -> tuple[list[int], list[int]]:
        msk = (1 << self.p.bit_length()) - 1
        xof = SHAKE256.new(self.nonce)
        bytes_n = self.p.bit_length() // 8 + (self.p.bit_length() % 8 != 0)
        c: list[int] = []

        # Read B bytes, discard extra bits, reject-sample: <50% chance of failure at each call
        while len(c) < self.r:
            n = int.from_bytes(xof.read(bytes_n), byteorder="big") & msk
            if n < self.p:
                c.append(n)

        return c

    def enc(self, x: int, k: int) -> int:
        if k >= self.p:
            print("Key is not a field element, will be reduced modulo p")
            k %= self.p

        if x >= self.p:
            print("Msg is not a field element, will be reduced modulo p")
            x %= self.p

        for i in range(self.r):
            x = (x + k) % self.p
            x = (x + self.c[i]) % self.p
            x = pow(x, self.d, self.p)

        return x

    def enc_feistel(self, xl: int, xr: int, k: int) -> int:
        if k >= self.p:
            print("Key is not a field element, will be reduced modulo p")
            k %= self.p

        if xl >= self.p:
            print("Left Msg is not a field element, will be reduced modulo p")
            xl %= self.p
        if xr >= self.p:
            print("Right Msg is not a field element, will be reduced modulo p")
            xr %= self.p

        for i in range(self.r):
            t = xl
            t = (t + k) % self.p
            t = (t + self.c[i]) % self.p
            t = pow(t, self.d, self.p)
            xl, xr = (xr + t) % self.p, xl

        return xl


def main():
    p = 2 ** 64 - 2 ** 32 + 1
    nonce = b"I know you: your past, your future."  # SHAKE-256 Nonce
    E = Mimc(p, nonce)

    if len(argv) < 4:
        print(f"Syntax: {argv[0]} <key> <xl> <xr>")
    else:
        key = int(argv[1])
        xl = int(argv[2])
        xr = int(argv[3])
        cip = E.enc_feistel(xl, xr, key)
        print(f"Msg: {xl:016x}, {xr:016x}")
        print(f"Key: {key:016x}")
        print(f"Cip: {cip:016x}")

    if True:
        print(f"Field:  {E.p}")
        print(f"Degree: {E.d}")
        print(f"Rounds: {E.r}")
        print(f"Constants:")
        print("{")
        for c in E.c:
            print(f"\t0x{c:x},")
        print("}")


if __name__ == "__main__":
    main()
