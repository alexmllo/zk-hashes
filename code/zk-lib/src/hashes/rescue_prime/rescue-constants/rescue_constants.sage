from CompactFIPS202 import SHAKE256

def get_round_constants( security_level ):
    assert(security_level == 128 or security_level == 160)
    # basic parameters
    if security_level == 128:
        m = 12
        capacity = 4
    else:
        m = 16
        capacity = 6
    p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
    N = 7
    # generate pseudorandom bytes
    bytes_per_int = ceil(len(bin(p)[2:]) / 8) + 1
    num_bytes = bytes_per_int * 2 * m * N
    seed_string = "RPO(%i,%i,%i,%i)" % (p, m, capacity, security_level)
    byte_string = SHAKE256(bytes(seed_string, "ascii"), num_bytes)

    # process byte string in chunks
    round_constants = []
    Fp = FiniteField(p)
    for i in range(2*m*N):
        chunk = byte_string[bytes_per_int*i : bytes_per_int*(i+1)]
        integer = sum(256^j * ZZ(chunk[j]) for j in range(len(chunk)))
        round_constants.append(Fp(integer % p))

    #file = open("round_constants.bin", "wb")
    #for constant in round_constants:
    #    file.write(constant.to_bytes("little"))
    #file.close()

    print(round_constants)
    print(len(round_constants))
    return round_constants
#get_round_constants(128)

def get_alphas( ):
    p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
    alpha = 5
    g, alphainv, garbage = xgcd(alpha, p-1)

    #print((alpha, (alphainv % (p-1))))
    return (alphainv % (p-1))

#alpha_inv = get_alphas()
#round_constants = get_round_constants(128)

#file = open("alpha_inv.bin", "wb")
#file.write(alpha_inv.to_bytes(32, "little"))
#file.close()

#file = open("round_constants.bin", "wb")
#for constant in round_constants:
    #file.write(constant.to_bytes("little"))
#file.close()


def get_number_of_rounds( p, m, capacity , security_level , alpha ):
    # get number of rounds for Groebner basis attack
    rate = m - capacity
    dcon = lambda N : floor(0.5 * (alpha-1) * m * (N-1) + 2) 
    v = lambda N : m*(N-1) + rate
    target = 2^security_level
    for l1 in range(1, 25):
        if binomial(v(l1) + dcon(l1), v(l1))^2 > target: 
            break
    
    # set a minimum value for sanity and add 50%
    print(ceil(1.5 * max(5, l1)))
#get_number_of_rounds(52435875175126190479447740508185965837690552500527637822603658699938581184513,
#12,4,128,7)

def get_mds( m ):
    assert(m == 12 or m == 16)
    p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
    Fp = FiniteField(p)
    if m == 12:
        return [Fp(i) for i in [7, 23, 8, 26, 13, 10, 9, 7, 6, 22, 21, 8]]
    if m == 16:
        return [Fp(i) for i in [256, 2, 1073741824, 2048, 16777216, 128, 8, 16, 524288, 4194304, 1, 268435456, 1, 1024, 2, 8192]]

def get_number_of_rounds_bls( p, m, capacity , security_level , alpha ):
    # get number of rounds for Groebner basis attack
    rate = m - capacity
    dcon = lambda N : floor(0.5 * (alpha-1) * m * (N-1) + 2) 
    v = lambda N : m*(N-1) + rate
    target = 2^security_level
    for l1 in range(1, 25):
        if binomial(v(l1) + dcon(l1), v(l1))^2 > target: 
            break
    # set a minimum value for sanity and add 50%
    print(ceil(1.5 * max(5, l1)))
    return ceil(1.5 * max(5, l1))#

get_number_of_rounds_bls(52435875175126190479447740508185965837690552500527637822603658699938581184513, 8, 4, 128, 5)

def get_alphas_bls( p ):
    for alpha in range(3, p):
        if gcd(alpha, p-1) == 1: 
            break
    g, alphainv, garbage = xgcd(alpha, p-1) 
    print(alpha)
    print(alphainv % (p-1))

    file = open("alpha_inv_bls.bin", "wb")
    file.write(alphainv.to_bytes(32, "little"))
    file.close()

    return (alpha, (alphainv % (p-1)))

#get_alphas_bls(52435875175126190479447740508185965837690552500527637822603658699938581184513)

def get_round_constants_bls( p, m, capacity , security_level , N ): # generate pseudorandom bytes
    bytes_per_int = ceil(len(bin(p)[2:]) / 8) + 1
    num_bytes = bytes_per_int * 2 * m * N
    seed_string = "Rescue -XLIX(%i,%i,%i,%i)" % (p, m, capacity, security_level)
    byte_string = SHAKE256(bytes(seed_string, "ascii"), num_bytes)
    # process byte string in chunks
    round_constants = [] 
    Fp = FiniteField(p) 
    for i in range(2*m*N):
        chunk = byte_string[bytes_per_int*i : bytes_per_int*( i+1)]
        integer = sum(256^j * ZZ(chunk[j]) for j in range(len (chunk)))
        round_constants.append(Fp(integer % p)) 
    print(round_constants)
    print(len(round_constants))

    file = open("round_constants_6.bin", "wb")
    for constant in round_constants:
        file.write(constant.to_bytes("little"))
    file.close()

    return round_constants

#get_round_constants_bls(52435875175126190479447740508185965837690552500527637822603658699938581184513, 8, 4, 128, 8)

def get_mds_matrix_bls( p, m ):
    # get a primitive element 
    Fp = FiniteField(p)
    g = Fp(2)
    while g.multiplicative_order() != p-1: 
        g=g+1
    # get a systematic generator matrix for the code
    V = matrix([[g^(i*j) for j in range(0, 2*m)] for i in range(0, m)])
    V_ech = V.echelon_form()
    # the MDS matrix is the transpose of the right half of this matrix
    MDS = V_ech[:, m:].transpose()

    matrix_array = []
    for i in range(0, m):
        for j in range(0, m):
            matrix_array.append(MDS[i, j])
    
    file = open("mds_matrix_4.bin", "wb")
    for value in matrix_array:
        file.write(value.to_bytes("little"))
    file.close()

    print(matrix_array)
    print(MDS)
    return MDS

get_mds_matrix_bls(52435875175126190479447740508185965837690552500527637822603658699938581184513, 8)