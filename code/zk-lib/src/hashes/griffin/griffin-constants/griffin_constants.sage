# Taken from https://github.com/Nashtare/griffin-hash

from CompactFIPS202 import SHAKE256

def get_round_constants(p, t, capacity, security_level, N):
    # generate pseudorandom bytes
    bytes_per_int = ceil(len(bin(p)[2:]) / 8) + 1
    # 1 value for alpha_2
    # 1 value for beta_2
    # t * (N-1) values for ARK
    num_elems = (t * (N - 1) + 2)
    num_bytes = bytes_per_int * num_elems
    seed_string = "Griffin(%i,%i,%i,%i)" % (p, t, capacity, security_level)
    byte_string = SHAKE256(bytes(seed_string, "ascii"), num_bytes)

    # process byte string in chunks
    round_constants = []
    alphas = []
    betas = []
    Fp = FiniteField(p)

    # generate alpha_2 and deduce the other ones
    chunk = byte_string[0: bytes_per_int]
    alpha = Fp(sum(256 ** j * ZZ(chunk[j]) for j in range(len(chunk))))
    alphas.append(alpha)
    for i in range(3, t):
        alphas.append(Fp(i - 1) * alpha)

    # generate beta_2 and deduce the other ones
    chunk = byte_string[bytes_per_int: bytes_per_int*2]
    beta = Fp(sum(256 ** j * ZZ(chunk[j]) for j in range(len(chunk))))
    betas.append(beta)
    for i in range(3, t):
        betas.append(Fp(i - 1)**2 * beta)

    for i in range(2, num_elems):
        chunk = byte_string[bytes_per_int*i: bytes_per_int*(i+1)]
        c = Fp(sum(256 ** j * ZZ(chunk[j]) for j in range(len(chunk))))
        round_constants.append(c)
    
    file = open("alphas_4.bin", "wb")
    for alpha in alphas:
        file.write(alpha.to_bytes("little"))
    file.close()

    file = open("betas_4.bin", "wb")
    for beta in betas:
        file.write(beta.to_bytes("little"))
    file.close()

    file = open("round_constamts_4.bin", "wb")
    for constant in round_constants:
        file.write(constant.to_bytes("little"))
    file.close()

    with open("alphas_4.bin", "rb") as file:
        data = file.read()
        # Process the binary data here
        print(data.hex())

    print(alphas)
    print(len(alphas))
    print(betas)
    print(len(betas))
    print(round_constants)
    print(len(round_constants))

    return alphas, betas, round_constants

#get_round_constants(52435875175126190479447740508185965837690552500527637822603658699938581184513, 4, 1, 128, 11)

def get_number_of_rounds(p, t, security_level, d):
    assert security_level <= min(256, floor(log(p, 2) * t/3.0))
    # get number of rounds for Groebner basis attack
    target = 2 ** (security_level // 2)
    for rgb in range(1, 25):
        left = binomial(rgb * (d + t) + 1, 1 + t * rgb)
        right = binomial(d**rgb + 1 + rgb, 1 + rgb)
        if min(left, right) >= target:
            break

    # set a minimum value for sanity and add 20%
    print(ceil(1.2 * max(6, 1 + rgb)))
    return ceil(1.2 * max(6, 1 + rgb))

get_number_of_rounds(52435875175126190479447740508185965837690552500527637822603658699938581184513, 8, 128, 5)

def get_powers(p):
    for d in range(3, p):
        if gcd(d, p-1) == 1:
            break
    g, dinv, garbage = xgcd(d, p-1)
    print(d)
    print(dinv % (p-1))

    file = open("d_inv.bin", "wb")
    file.write(dinv.to_bytes(32, "little"))
    file.close()

    return (d, (dinv % (p-1)))

#get_powers(52435875175126190479447740508185965837690552500527637822603658699938581184513)

def get_matrix(p, t):
    # TODO: the decomposition below is overly complicated. It
    # would probably be simpler to rely on numpy.
    Fp = FiniteField(p)
    if t == 3:
        return Matrix.circulant([2, 1, 1]).change_ring(Fp)
    if t == 4:
        print(Matrix([[5, 7, 1, 3], [4, 6, 1, 1], [1, 3, 5, 7], [1, 1, 4, 6]]).change_ring(Fp))
        return Matrix([[5, 7, 1, 3], [4, 6, 1, 1], [1, 3, 5, 7], [1, 1, 4, 6]]).change_ring(Fp)

    # for larger cases, we split the matrix M as M' x M''
    # with M' a diagonal matrix and M'' a circulant one.
    # this requires t to be a multiple of 4
    assert t % 4 == 0
    tp = t // 4

    Mt = Matrix([[5, 7, 1, 3], [4, 6, 1, 1], [1, 3, 5, 7], [1, 1, 4, 6]]).change_ring(Fp)
    M1 = Matrix.zero(t, t)
    # put Mt on the diagonal of the larger matrix M1
    for i in range(tp):
        for row in range(4):
            for col in range(4):
                M1[4*i + row, 4*i + col] = Mt[row, col]

    M2 = Matrix.diagonal([1 for i in range(t)])
    # we fill up the missing non-zero coefficients so
    # that M2 looks like = circ(2I_4, I_4, ..., I_4).
    # we proceed to do so in two phases as the matrix is
    # symmetric.
    for col in range(1, tp):
        for row in range(0, col):
            for diag in range(4):
                M2[4*row + diag, 4*col + diag] = 1
    # now M2 is upper-triangular, we can transpose and add
    # it to obtain the desired matrix
    M2 = M2 + M2.transpose()

    M = M1 * M2
    print(M.change_ring(Fp))
    return M.change_ring(Fp)

get_matrix(52435875175126190479447740508185965837690552500527637822603658699938581184513, 8)