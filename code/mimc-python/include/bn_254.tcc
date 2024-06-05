#pragma once

#ifdef __cplusplus

extern "C" {
#endif

// the BN-254 elliptic curve scalar field order: https://hackmd.io/@jpw/bn254
// p = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
CONSTEXPR static const bn254_32_t P32 = {REVERSE32(0x30644e72, 0xe131a029, 0xb85045b6, 0x8181585d,
                                                   0x2833e848, 0x79b97091, 0x43e1f593, 0xf0000001)};
CONSTEXPR static const bn254_64_t P64 = {
    REVERSE64(0x30644e72e131a029, 0xb85045b68181585d, 0x2833e84879b97091, 0x43e1f593f0000001)};

// MU value for Barrett multiplication, MU = 2^{ceil(log2(P))} // P = 2^{508} // P
CONSTEXPR static const bn254_64_t MU64 = {
    REVERSE64(0x54a47462623a04a7, 0xab074a5868073014, 0x7144852009e880ae, 0x620703a6be1de925)};

// P_I value for Montgomery multiplication/reduction, P_I = -1/P mod 2^256
CONSTEXPR static const bn254_64_t PI64 = {
    REVERSE64(0x73f82f1d0d8341b2, 0xe39a982899062391, 0x6586864b4c6911b3, 0xc2e1f593efffffff)};

// R2 value for Montgomery transform, R2 = -P % P mod 2^256
CONSTEXPR static const bn254_64_t R2_64 = {
    REVERSE64(0x0216d0b17f4e44a5, 0x8c49833d53bb8085, 0x53fe3ab1e35c59e3, 0x1bb8e645ae216da7)};

CONSTEXPR static inline uint8_t bn254_eql32(const bn254_32_t *px, const bn254_32_t *py)
{
    const uint32_t *x = px->v;
    const uint32_t *y = py->v;

    return (x[0] == y[0]) & (x[1] == y[1]) & (x[2] == y[2]) & (x[3] == y[3]) //
         & (x[4] == y[4]) & (x[5] == y[5]) & (x[6] == y[6]) & (x[7] == y[7]);
}

CONSTEXPR static inline uint8_t bn254_eql64(const bn254_64_t *px, const bn254_64_t *py)
{
    const uint64_t *x = px->v;
    const uint64_t *y = py->v;

    return (x[0] == y[0]) & (x[1] == y[1]) & (x[2] == y[2]) & (x[3] == y[3]);
}

// return 1 in case of overflow, 0 otherwise
CONSTEXPR static inline uint8_t bn254_check_of32(const bn254_32_t *px)
{
    const uint32_t *x = px->v;

    uint8_t c = x[7] >= P32.v[7];

    c |= c & (x[6] >= P32.v[6]);
    c |= c & (x[5] >= P32.v[5]);
    c |= c & (x[4] >= P32.v[4]);
    c |= c & (x[3] >= P32.v[3]);
    c |= c & (x[2] >= P32.v[2]);
    c |= c & (x[1] >= P32.v[1]);
    c |= c & (x[0] >= P32.v[0]);

    return c;
}

// return 1 in case of overflow, 0 otherwise
CONSTEXPR static inline uint8_t bn254_check_of64(const bn254_64_t *px)
{
    const uint64_t *x = px->v;

    uint8_t c = x[3] >= P64.v[3];

    c |= c & (x[2] >= P64.v[2]);
    c |= c & (x[1] >= P64.v[1]);
    c |= c & (x[0] >= P64.v[0]);

    return c;
}

CONSTEXPR static inline void bn254_fix_of32(bn254_32_t *px)
{
    uint32_t f = -(uint32_t)bn254_check_of32(px);
    uint32_t *x = px->v;
    uint8_t c;

    x[0] = subb32(x[0], f & P32.v[0], 0, &c);
    x[1] = subb32(x[1], f & P32.v[1], c, &c);
    x[2] = subb32(x[2], f & P32.v[2], c, &c);
    x[3] = subb32(x[3], f & P32.v[3], c, &c);
    x[4] = subb32(x[4], f & P32.v[4], c, &c);
    x[5] = subb32(x[5], f & P32.v[5], c, &c);
    x[6] = subb32(x[6], f & P32.v[6], c, &c);
    x[7] = subb32(x[7], f & P32.v[7], c, &c);
}

CONSTEXPR static inline void bn254_fix_of64(bn254_64_t *px)
{
    uint64_t f = -(uint64_t)bn254_check_of64(px);
    uint64_t *x = px->v;
    uint8_t c;

    x[0] = subb64(x[0], f & P64.v[0], 0, &c);
    x[1] = subb64(x[1], f & P64.v[1], c, &c);
    x[2] = subb64(x[2], f & P64.v[2], c, &c);
    x[3] = subb64(x[3], f & P64.v[3], c, &c);
}

CONSTEXPR static inline void bn254_fix_uf32(bn254_32_t *px)
{
    uint32_t f = -(uint32_t)bn254_check_of32(px);
    uint32_t *x = px->v;
    uint8_t c;

    x[0] = addc32(x[0], f & P32.v[0], 0, &c);
    x[1] = addc32(x[1], f & P32.v[1], c, &c);
    x[2] = addc32(x[2], f & P32.v[2], c, &c);
    x[3] = addc32(x[3], f & P32.v[3], c, &c);
    x[4] = addc32(x[4], f & P32.v[4], c, &c);
    x[5] = addc32(x[5], f & P32.v[5], c, &c);
    x[6] = addc32(x[6], f & P32.v[6], c, &c);
    x[7] = addc32(x[7], f & P32.v[7], c, &c);
}

CONSTEXPR static inline void bn254_fix_uf64(bn254_64_t *px)
{
    uint64_t f = -(uint64_t)bn254_check_of64(px);
    uint64_t *x = px->v;
    uint8_t c;

    x[0] = addc64(x[0], f & P64.v[0], 0, &c);
    x[1] = addc64(x[1], f & P64.v[1], c, &c);
    x[2] = addc64(x[2], f & P64.v[2], c, &c);
    x[3] = addc64(x[3], f & P64.v[3], c, &c);
}

CONSTEXPR static inline void bn254_add_nofix32(bn254_32_t *px, const bn254_32_t *py)
{
    uint32_t *x = px->v;
    const uint32_t *y = py->v;
    uint8_t c;

    x[0] = addc32(x[0], y[0], 0, &c);
    x[1] = addc32(x[1], y[1], c, &c);
    x[2] = addc32(x[2], y[2], c, &c);
    x[3] = addc32(x[3], y[3], c, &c);
    x[4] = addc32(x[4], y[4], c, &c);
    x[5] = addc32(x[5], y[5], c, &c);
    x[6] = addc32(x[6], y[6], c, &c);
    x[7] = addc32(x[7], y[7], c, &c);
}

CONSTEXPR static inline void bn254_add_nofix64(bn254_64_t *px, const bn254_64_t *py)
{
    uint64_t *x = px->v;
    const uint64_t *y = py->v;
    uint8_t c;

    x[0] = addc64(x[0], y[0], 0, &c);
    x[1] = addc64(x[1], y[1], c, &c);
    x[2] = addc64(x[2], y[2], c, &c);
    x[3] = addc64(x[3], y[3], c, &c);
}

CONSTEXPR static inline void bn254_add32(bn254_32_t *px, const bn254_32_t *py)
{
    bn254_add_nofix32(px, py);
    bn254_fix_of32(px);
}

CONSTEXPR static inline void bn254_add64(bn254_64_t *px, const bn254_64_t *py)
{
    bn254_add_nofix64(px, py);
    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_inc_nofix32(bn254_32_t *px)
{
    uint32_t *x = px->v;
    uint8_t c;

    x[0] = addc32(x[0], 0, 1, &c);
    x[1] = addc32(x[1], 0, c, &c);
    x[2] = addc32(x[2], 0, c, &c);
    x[3] = addc32(x[3], 0, c, &c);
    x[4] = addc32(x[4], 0, c, &c);
    x[5] = addc32(x[5], 0, c, &c);
    x[6] = addc32(x[6], 0, c, &c);
    x[7] = addc32(x[7], 0, c, &c);
}

CONSTEXPR static inline void bn254_inc_nofix64(bn254_64_t *px)
{
    uint64_t *x = px->v;
    uint8_t c;

    x[0] = addc64(x[0], 0, 1, &c);
    x[1] = addc64(x[1], 0, c, &c);
    x[2] = addc64(x[2], 0, c, &c);
    x[3] = addc64(x[3], 0, c, &c);
}

CONSTEXPR static inline void bn254_inc32(bn254_32_t *px)
{
    bn254_inc_nofix32(px);
    bn254_fix_of32(px);
}

CONSTEXPR static inline void bn254_inc64(bn254_64_t *px)
{
    bn254_inc_nofix64(px);
    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_neg32(bn254_32_t *px)
{
    uint8_t b;
    uint32_t *x = px->v;

    x[0] = subb32(P32.v[0], x[0], 0, &b);
    x[1] = subb32(P32.v[1], x[1], b, &b);
    x[2] = subb32(P32.v[2], x[2], b, &b);
    x[3] = subb32(P32.v[3], x[3], b, &b);
    x[4] = subb32(P32.v[4], x[4], b, &b);
    x[5] = subb32(P32.v[5], x[5], b, &b);
    x[6] = subb32(P32.v[6], x[6], b, &b);
    x[7] = subb32(P32.v[7], x[7], b, &b);
}

CONSTEXPR static inline void bn254_neg64(bn254_64_t *px)
{
    uint8_t b;
    uint64_t *x = px->v;

    x[0] = subb64(P64.v[0], x[0], 0, &b);
    x[1] = subb64(P64.v[1], x[1], b, &b);
    x[2] = subb64(P64.v[2], x[2], b, &b);
    x[3] = subb64(P64.v[3], x[3], b, &b);
}

CONSTEXPR static inline void bn254_sub_nofix32(bn254_32_t *px, const bn254_32_t *py)
{
    uint8_t b;
    uint32_t *x = px->v;
    const uint32_t *y = py->v;

    x[0] = subb32(x[0], y[0], 0, &b);
    x[1] = subb32(x[1], y[1], b, &b);
    x[2] = subb32(x[2], y[2], b, &b);
    x[3] = subb32(x[3], y[3], b, &b);
    x[4] = subb32(x[4], y[4], b, &b);
    x[5] = subb32(x[5], y[5], b, &b);
    x[6] = subb32(x[6], y[6], b, &b);
    x[7] = subb32(x[7], y[7], b, &b);
}

CONSTEXPR static inline void bn254_sub_nofix64(bn254_64_t *px, const bn254_64_t *py)
{
    uint8_t b;
    uint64_t *x = px->v;
    const uint64_t *y = py->v;

    x[0] = subb64(x[0], y[0], 0, &b);
    x[1] = subb64(x[1], y[1], b, &b);
    x[2] = subb64(x[2], y[2], b, &b);
    x[3] = subb64(x[3], y[3], b, &b);
}

CONSTEXPR static inline void bn254_sub32(bn254_32_t *px, const bn254_32_t *py)
{
    bn254_sub_nofix32(px, py);
    bn254_fix_uf32(px);
}

CONSTEXPR static inline void bn254_sub64(bn254_64_t *px, const bn254_64_t *py)
{
    bn254_sub_nofix64(px, py);
    bn254_fix_uf64(px);
}

CONSTEXPR static inline void bn254_dec_nofix32(bn254_32_t *px)
{
    uint32_t *x = px->v;
    uint8_t b;

    x[0] = subb32(x[0], 0, 1, &b);
    x[1] = subb32(x[1], 0, b, &b);
    x[2] = subb32(x[2], 0, b, &b);
    x[3] = subb32(x[3], 0, b, &b);
    x[4] = subb32(x[4], 0, b, &b);
    x[5] = subb32(x[5], 0, b, &b);
    x[6] = subb32(x[6], 0, b, &b);
    x[7] = subb32(x[7], 0, b, &b);
}

CONSTEXPR static inline void bn254_dec_nofix64(bn254_64_t *px)
{
    uint64_t *x = px->v;
    uint8_t b;

    x[0] = subb64(x[0], 0, 1, &b);
    x[1] = subb64(x[1], 0, b, &b);
    x[2] = subb64(x[2], 0, b, &b);
    x[3] = subb64(x[3], 0, b, &b);
}

CONSTEXPR static inline void bn254_dec32(bn254_32_t *px)
{
    bn254_dec_nofix32(px);
    bn254_fix_uf32(px);
}

CONSTEXPR static inline void bn254_dec64(bn254_64_t *px)
{
    bn254_dec_nofix64(px);
    bn254_fix_uf64(px);
}

CONSTEXPR static inline void bn254_mul64(bn254_64_t *px, const bn254_64_t *py)
{
    /*
    We do a Barrett multiplication/reduction, from my experience it is faster than Montgomery 
    multiplication for ~256bit fields (at least on x86-64) for a single multiplication: however
    Montgomery is better for long computations if they are all done in Montgomery space...
    Barrett multiplication is based on the fact that:
        xy ~= p(xy * 2^{2n}/p)/2^{2n} (mod p)
    By precomputing m = 2^{2n}/p, then:
        xy ~= p(xym) >> 2n (mod p)
    In particular:
        xy % p == (xy - p(xym) >> 2n) & (2^n - 1)
    Several optimizations are possible by discarding extra information
    */
    uint64_t z[8];
    uint64_t q;
    uint64x2_t t;
    uint8_t c;
    uint64_t *x = px->v;
    const uint64_t *y = py->v;

    // ============== SCHOOLBOOK MULTIPLICATION z = x * y ============== //

    t = mulx64(x[0], y[0]);
    z[0] = t.l;
    z[1] = t.h;

    t = mulx64(x[0], y[1]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = c;
    z[2] += t.h; // cannot overflow
    t = mulx64(x[1], y[0]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = addc64(z[2], t.h, c, &c);
    z[3] = c;

    t = mulx64(x[0], y[2]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], y[1]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] = c;
    t = mulx64(x[2], y[0]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] += c;

    t = mulx64(x[0], y[3]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], y[2]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] = c;
    t = mulx64(x[2], y[1]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;
    t = mulx64(x[3], y[0]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;

    t = mulx64(x[1], y[3]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c); // cannot overflow
    t = mulx64(x[2], y[2]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] = c;
    t = mulx64(x[3], y[1]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] += c;

    t = mulx64(x[2], y[3]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c); // cannot overflow
    t = mulx64(x[3], y[2]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c);
    z[7] = c;

    t = mulx64(x[3], y[3]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);

    // ============== MULTIPLY BY MU, discard low 508 bits q = MU * z ============== //

    /* As we discard first 6 digits, plus the low 60 bits of the 7th, the carry can be computed 
     with no error by only computing (carrylessly) the 7th digit
    */

    // now and x[0] contains the 7th digit
    t = mulx64(z[3], MU64.v[3]);
    x[0] = t.h;
    t = mulx64(z[4], MU64.v[2]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] = c;
    t = mulx64(z[5], MU64.v[1]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] += c;
    t = mulx64(z[6], MU64.v[0]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] += c;

    t = mulx64(z[4], MU64.v[3]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c); // cannot overflow
    t = mulx64(z[5], MU64.v[2]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] = c;
    t = mulx64(z[6], MU64.v[1]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] += c;
    t = mulx64(z[7], MU64.v[0]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] += c;

    t = mulx64(z[5], MU64.v[3]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c); // cannot overflow
    t = mulx64(z[6], MU64.v[2]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] = c;
    t = mulx64(z[7], MU64.v[1]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] += c;

    t = mulx64(z[6], MU64.v[3]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c); // cannot overflow
    t = mulx64(z[7], MU64.v[2]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);
    q = c;

    t = mulx64(z[7], MU64.v[3]);
    x[3] = addc64(x[3], t.l, 0, &c);
    q = addc64(q, t.h, c, &c);


    // now we compact the high 254 bits
    x[0] >>= 60;
    x[0] |= x[1] << 4;

    x[1] >>= 60;
    x[1] |= x[2] << 4;

    x[2] >>= 60;
    x[2] |= x[3] << 4;

    x[3] >>= 60;
    x[3] |= q << 4;

    // ============== MULTIPLY BY P, z[4:] = x * P ============== //
    t = mulx64(x[0], P64.v[0]);
    z[4] = t.l;
    z[5] = t.h;

    t = mulx64(x[0], P64.v[1]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = c;
    z[6] += t.h; // cannot overflow
    t = mulx64(x[1], P64.v[0]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c);
    z[7] = c;

    t = mulx64(x[0], P64.v[2]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);
    t = mulx64(x[1], P64.v[1]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);
    t = mulx64(x[2], P64.v[0]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);

    z[7] += x[0] * P64.v[3];
    z[7] += x[1] * P64.v[2];
    z[7] += x[2] * P64.v[1];
    z[7] += x[3] * P64.v[0];


    // SUBTRACT FROM ORIGINAL VALUE
    x[0] = subb64(z[0], z[4], 0, &c);
    x[1] = subb64(z[1], z[5], c, &c);
    x[2] = subb64(z[2], z[6], c, &c);
    x[3] = subb64(z[3], z[7], c, &c);

    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_sqr64(bn254_64_t *px)
{
    /*
    We exploit the structure of squaring to speed it up compared to multiplication:
    (ax^3 + bx^2 + cx + d)^2 = 
    a^2x^6 + b^2x^4 + c^2x^2 + d^2 + 2abx^5 + 2acx^4 + 2adx^3 + 2bcx^3 + 2bdx^2 + 2cdx =
    a^2x^6 + 2abx^5 + b^2x^4 + 2acx^4 + 2adx^3 + 2bcx^3 + c^2x^2 + 2bdx^2 + 2cdx + d^2=
    */
    uint64_t y[8];
    uint64_t q;
    uint64x2_t t;
    uint8_t c;
    uint64_t *x = px->v;

    // ============== STANDARD SQUARING y = x^2 ============== //

    // d^2
    t = mulx64(x[0], x[0]);
    y[0] = t.l;
    y[1] = t.h;

    // 2cdx
    t = mulx64(x[0], x[1]);
    y[1] = addc64(y[1], t.l, 0, &c);
    y[2] = c;
    y[1] = addc64(y[1], t.l, 0, &c);
    y[2] = addc64(y[2], t.h, c, &c); // cannot overflow
    y[2] = addc64(y[2], t.h, 0, &c);
    y[3] = c;

    // c^2x^2 + 2bdx^2
    t = mulx64(x[0], x[2]);
    y[2] = addc64(y[2], t.l, 0, &c);
    y[3] = addc64(y[3], t.h, c, &c); // cannot overflow
    y[2] = addc64(y[2], t.l, 0, &c);
    y[3] = addc64(y[3], t.h, c, &c);
    y[4] = c;
    t = mulx64(x[1], x[1]);
    y[2] = addc64(y[2], t.l, 0, &c);
    y[3] = addc64(y[3], t.h, c, &c);
    y[4] += c;

    // 2adx^3 + 2bcx^3
    t = mulx64(x[0], x[3]);
    y[3] = addc64(y[3], t.l, 0, &c);
    y[4] = addc64(y[4], t.h, c, &c); // cannot overflow
    y[3] = addc64(y[3], t.l, 0, &c);
    y[4] = addc64(y[4], t.h, c, &c);
    y[5] = c;
    t = mulx64(x[1], x[2]);
    y[3] = addc64(y[3], t.l, 0, &c);
    y[4] = addc64(y[4], t.h, c, &c);
    y[5] += c;
    y[3] = addc64(y[3], t.l, 0, &c);
    y[4] = addc64(y[4], t.h, c, &c);
    y[5] += c;

    // b^2x^4 + 2acx^4
    t = mulx64(x[1], x[3]);
    y[4] = addc64(y[4], t.l, 0, &c);
    y[5] = addc64(y[5], t.h, c, &c); // cannot overflow
    y[4] = addc64(y[4], t.l, 0, &c);
    y[5] = addc64(y[5], t.h, c, &c);
    y[6] = c;
    t = mulx64(x[2], x[2]);
    y[4] = addc64(y[4], t.l, 0, &c);
    y[5] = addc64(y[5], t.h, c, &c);
    y[6] += c;

    // 2abx^5
    t = mulx64(x[2], x[3]);
    y[5] = addc64(y[5], t.l, 0, &c);
    y[6] = addc64(y[6], t.h, c, &c); // cannot overflow
    y[5] = addc64(y[5], t.l, 0, &c);
    y[6] = addc64(y[6], t.h, c, &c);
    y[7] = c;

    // a^2x^6
    t = mulx64(x[3], x[3]);
    y[6] = addc64(y[6], t.l, 0, &c);
    y[7] = addc64(y[7], t.h, c, &c);

    // ============== MULTIPLY BY MU, discard low 508 bits q = MU * y ============== //
    // now and x[0] contains the 7th digit
    t = mulx64(y[3], MU64.v[3]);
    x[0] = addc64(0, t.h, 0, &c);
    x[1] = c;
    t = mulx64(y[4], MU64.v[2]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] += c;
    t = mulx64(y[5], MU64.v[1]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] += c;
    t = mulx64(y[6], MU64.v[0]);
    x[0] = addc64(x[0], t.h, 0, &c);
    x[1] += c;

    t = mulx64(y[4], MU64.v[3]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] = c;
    t = mulx64(y[5], MU64.v[2]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] += c;
    t = mulx64(y[6], MU64.v[1]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] += c;
    t = mulx64(y[7], MU64.v[0]);
    x[0] = addc64(x[0], t.l, 0, &c);
    x[1] = addc64(x[1], t.h, c, &c);
    x[2] += c;

    t = mulx64(y[5], MU64.v[3]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] = c;
    t = mulx64(y[6], MU64.v[2]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] += c;
    t = mulx64(y[7], MU64.v[1]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] += c;

    t = mulx64(y[6], MU64.v[3]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c); // we don't have x[4]
    q = c;
    t = mulx64(y[7], MU64.v[2]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);
    q += c;

    t = mulx64(y[7], MU64.v[3]);
    x[3] = addc64(x[3], t.l, 0, &c);
    q = addc64(q, t.h, c, &c);

    // now we compact the high 254 bits
    x[0] >>= 60;
    x[0] |= x[1] << 4;

    x[1] >>= 60;
    x[1] |= x[2] << 4;

    x[2] >>= 60;
    x[2] |= x[3] << 4;

    x[3] >>= 60;
    x[3] |= q << 4;

    // ============== MULTIPLY BY P, y[4:] = x * P ============== //
    t = mulx64(x[0], P64.v[0]);
    y[4] = t.l;
    y[5] = t.h;

    t = mulx64(x[0], P64.v[1]);
    y[5] = addc64(y[5], t.l, 0, &c);
    y[6] = c;
    y[6] += t.h; // cannot overflow
    t = mulx64(x[1], P64.v[0]);
    y[5] = addc64(y[5], t.l, 0, &c);
    y[6] = addc64(y[6], t.h, c, &c);
    y[7] = c;

    t = mulx64(x[0], P64.v[2]);
    y[6] = addc64(y[6], t.l, 0, &c);
    y[7] = addc64(y[7], t.h, c, &c);
    t = mulx64(x[1], P64.v[1]);
    y[6] = addc64(y[6], t.l, 0, &c);
    y[7] = addc64(y[7], t.h, c, &c);
    t = mulx64(x[2], P64.v[0]);
    y[6] = addc64(y[6], t.l, 0, &c);
    y[7] = addc64(y[7], t.h, c, &c);

    y[7] += x[0] * P64.v[3];
    y[7] += x[1] * P64.v[2];
    y[7] += x[2] * P64.v[1];
    y[7] += x[3] * P64.v[0];


    // SUBTRACT FROM ORIGINAL VALUE
    x[0] = subb64(y[0], y[4], 0, &c);
    x[1] = subb64(y[1], y[5], c, &c);
    x[2] = subb64(y[2], y[6], c, &c);
    x[3] = subb64(y[3], y[7], c, &c);

    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_monty_mul_sos_nofix64(bn254_64_t *px, const bn254_64_t *py)
{
    /* 
    We transform x into its Montgomery representation.
    Consider:
        r = (2^{2n} - p) % p
        p' = -1/p (mod 2^n):
    Then take:
    x' = xr + (xrp'p) / 2^n
    Clearly:
    x' ~= x
    where p' = 1/p (mod 2^n)
    This then allows us to do:
    xy % p = x'y'
    */
    uint64_t w[8];
    uint64_t z[8];
    uint64x2_t t;
    uint8_t c;
    uint64_t *x = px->v;
    const uint64_t *y = py->v;

    // ============== Full multiplication: w = x * y  ============== //
    t = mulx64(x[0], y[0]);
    w[0] = t.l;
    w[1] = t.h;

    t = mulx64(x[0], y[1]);
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = c;
    w[2] += t.h; // cannot overflow
    t = mulx64(x[1], y[0]);
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = addc64(w[2], t.h, c, &c);
    w[3] = c;

    t = mulx64(x[0], y[2]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], y[1]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);
    w[4] = c;
    t = mulx64(x[2], y[0]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);
    w[4] += c;

    t = mulx64(x[0], y[3]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], y[2]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] = c;
    t = mulx64(x[2], y[1]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] += c;
    t = mulx64(x[3], y[0]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] += c;

    t = mulx64(x[1], y[3]);
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c); // cannot overflow
    t = mulx64(x[2], y[2]);
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c);
    w[6] = c;
    t = mulx64(x[3], y[1]);
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c);
    w[6] += c;

    t = mulx64(x[2], y[3]);
    w[5] = addc64(w[5], t.l, 0, &c);
    w[6] = addc64(w[6], t.h, c, &c); // cannot overflow
    t = mulx64(x[3], y[2]);
    w[5] = addc64(w[5], t.l, 0, &c);
    w[6] = addc64(w[6], t.h, c, &c);
    w[7] = c;

    t = mulx64(x[3], y[3]);
    w[6] = addc64(w[6], t.l, 0, &c);
    w[7] = addc64(w[7], t.h, c, &c);

    // ============== Partial multiplication: x = w*PI  ============== //
    t = mulx64(w[0], PI64.v[0]);
    x[0] = t.l;
    x[1] = t.h;

    t = mulx64(w[0], PI64.v[1]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = c;
    x[2] += t.h; // cannot overflow
    t = mulx64(w[1], PI64.v[0]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] = c;
    t = mulx64(w[0], PI64.v[2]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c); // cannot overflow
    t = mulx64(w[1], PI64.v[1]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);
    t = mulx64(w[2], PI64.v[0]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);

    x[3] += w[0] * PI64.v[3];
    x[3] += w[1] * PI64.v[2];
    x[3] += w[2] * PI64.v[1];
    x[3] += w[3] * PI64.v[0];

    // ============== Full multiplication: z = x*P  ============== //
    t = mulx64(x[0], P64.v[0]);
    z[0] = t.l;
    z[1] = t.h;

    t = mulx64(x[0], P64.v[1]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = c;
    z[2] += t.h; // cannot overflow
    t = mulx64(x[1], P64.v[0]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = addc64(z[2], t.h, c, &c);
    z[3] = c;

    t = mulx64(x[0], P64.v[2]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], P64.v[1]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] = c;
    t = mulx64(x[2], P64.v[0]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] += c;

    t = mulx64(x[0], P64.v[3]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], P64.v[2]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] = c;
    t = mulx64(x[2], P64.v[1]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;
    t = mulx64(x[3], P64.v[0]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;

    t = mulx64(x[1], P64.v[3]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c); // cannot overflow
    t = mulx64(x[2], P64.v[2]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] = c;
    t = mulx64(x[3], P64.v[1]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] += c;

    t = mulx64(x[2], P64.v[3]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c); // cannot overflow
    t = mulx64(x[3], P64.v[2]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c);
    z[7] = c;

    t = mulx64(x[3], P64.v[3]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);

    // ============== Final Addition and Shift  ============== //
    addc64(w[0], z[0], 0, &c);
    addc64(w[1], z[1], c, &c);
    addc64(w[2], z[2], c, &c);
    addc64(w[3], z[3], c, &c);
    x[0] = addc64(w[4], z[4], c, &c);
    x[1] = addc64(w[5], z[5], c, &c);
    x[2] = addc64(w[6], z[6], c, &c);
    x[3] = addc64(w[7], z[7], c, &c);
}

CONSTEXPR static inline void bn254_monty_mul_cios_nofix64(bn254_64_t *RESTRICT px,
                                                          const bn254_64_t *RESTRICT py)
{
    /* 
    We use the CIOS algorithm: we only need one extra word (t[4]) since 5p < 2^256
    */
    uint64_t t[5];
    uint64x2_t w;
    uint64_t m;
    uint8_t c;
    uint64_t *x = px->v;
    const uint64_t *y = py->v;

    // ============== Partial multiplication 0 ============== //

    w = mulx64(x[0], y[0]);
    t[0] = w.l;
    t[1] = w.h;

    w = mulx64(x[0], y[1]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[2] = addc64(0, w.h, c, &c);

    w = mulx64(x[0], y[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[3] = addc64(0, w.h, c, &c);

    w = mulx64(x[0], y[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(0, w.h, c, &c);

    // ============== Partial reduction 0 ============== //

    // PI64.v[0] * P64.v[0] = -1, so w.l = -x[0]  and w.l + x[0] = 0 with carry 1 unless x[0] was 0
    m = t[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    // from now on we use t[0] just to store partial carries
    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], t[1], 0, &c);
    t[1] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[0] = c;
    t[1] = addc64(t[1], t[2], 0, &c);
    t[2] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = c;
    t[2] = addc64(t[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c); // cannot overflow

    // ============== Partial multiplication 1 ============== //

    w = mulx64(x[1], y[0]);
    x[0] = addc64(w.l, x[0], 0, &c);
    t[1] = addc64(t[1], w.h, c, &c);
    t[0] = c;

    w = mulx64(x[1], y[1]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[2] = addc64(t[2], t[0], 0, &c);
    t[0] = c;

    w = mulx64(x[1], y[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[3] = addc64(t[3], t[0], 0, &c);
    t[4] = c;

    w = mulx64(x[1], y[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);

    // ============== Partial reduction 1 ============== //

    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    x[1] = c;
    x[0] = addc64(x[0], t[1], 0, &c);
    x[1] = addc64(x[1], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(x[1], w.l, 0, &c);
    t[0] = c;
    x[1] = addc64(x[1], t[2], 0, &c);
    t[2] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = c;
    t[2] = addc64(t[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c); // cannot overflow

    // ============== Partial multiplication 2 ============== //

    w = mulx64(x[2], y[0]);
    x[0] = addc64(w.l, x[0], 0, &c);
    x[1] = addc64(x[1], w.h, c, &c);
    t[0] = c;

    w = mulx64(x[2], y[1]);
    x[1] = addc64(x[1], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[2] = addc64(t[2], t[0], 0, &c);
    t[0] = c;

    w = mulx64(x[2], y[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[3] = addc64(t[3], t[0], 0, &c);
    t[4] = c;

    w = mulx64(x[2], y[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);

    // ============== Partial reduction 2 ============== //

    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], x[1], 0, &c);
    x[1] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(x[1], w.l, 0, &c);
    x[2] = c;
    x[1] = addc64(x[1], t[2], 0, &c);
    x[2] = addc64(x[2], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    x[2] = addc64(x[2], w.l, 0, &c);
    t[0] = c;
    x[2] = addc64(x[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c);


    // ============== Partial multiplication 3 ============== //

    w = mulx64(x[3], y[0]);
    x[0] = addc64(w.l, x[0], 0, &c);
    x[1] = addc64(x[1], w.h, c, &c);
    t[0] = c;

    w = mulx64(x[3], y[1]);
    x[1] = addc64(x[1], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    x[2] = addc64(x[2], t[0], 0, &c);
    t[0] = c;

    w = mulx64(x[3], y[2]);
    x[2] = addc64(x[2], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[3] = addc64(t[3], t[0], 0, &c);
    t[4] = c;

    w = mulx64(x[3], y[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);

    // ============== Partial reduction 3 ============== //

    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], x[1], 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(t[0], w.l, 0, &c);
    t[0] = c;
    x[1] = addc64(x[1], x[2], 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    x[2] = addc64(t[0], w.l, 0, &c);
    x[3] = c;
    x[2] = addc64(x[2], t[3], 0, &c);
    x[3] = addc64(x[3], w.h, c, &c);

    x[3] = addc64(x[3], t[4], 0, &c);
}

CONSTEXPR static inline void bn254_monty_mul_fios_nofix64(bn254_64_t *px, const bn254_64_t *py)
{
    /* 
    We use the FIOS algorithm, there is an error somewhere as it does not pass the tests
    */
    uint64_t t[6] = {0};
    uint64_t *x = px->v;
    const uint64_t *y = py->v;
    uint64x2_t w;
    uint8_t b;
    uint64_t c;
    uint64_t s;
    uint64_t m;

    for (size_t i = 0; i < 4; ++i)
    {
        w = mulx64(x[i], y[0]);
        s = addc64(t[0], w.l, 0, &b);
        c = b;
        c += w.h;
        t[1] = addc64(t[1], c, 0, &b);
        t[2] = addc64(t[2], 0, b, &b);
        t[3] = addc64(t[3], 0, b, &b);
        t[4] = addc64(t[4], 0, b, &b);
        t[5] = addc64(t[5], 0, b, &b);
        m = s * PI64.v[0];
        w = mulx64(m, P64.v[0]);
        s = addc64(s, w.l, 0, &b);
        c = b;
        c += w.h;

        for (size_t j = 1; j < 4; ++j)
        {
            w = mulx64(x[i], y[j]);
            s = addc64(t[j], c, 0, &b);
            c = b;
            s = addc64(s, w.l, 0, &b);
            c = addc64(c, w.h, b, &b);
            t[j + 1] = addc64(t[j + 1], c, 0, &b);
            for (size_t k = j + 2; k < 6; ++k)
                t[k] = addc64(t[k - 1], 0, b, &b);
            w = mulx64(m, P64.v[j]);
            s = addc64(s, w.l, 0, &b);
            c = b;
            c += w.h;
            t[j - 1] = s;
        }
        s = addc64(t[4], c, 0, &b);
        c = b;
        t[3] = s;
        t[4] = t[5] + c;
        t[5] = 0;
    }

    x[0] = t[0];
    x[1] = t[1];
    x[2] = t[2];
    x[3] = t[3];
}

CONSTEXPR static inline void bn254_monty_mul_nofix64(bn254_64_t *RESTRICT px,
                                                     const bn254_64_t *RESTRICT py)
{
    bn254_monty_mul_cios_nofix64(px, py);
}

CONSTEXPR static inline void bn254_monty_mul64(bn254_64_t *RESTRICT px,
                                               const bn254_64_t *RESTRICT py)
{
    bn254_monty_mul_nofix64(px, py);
    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_monty_sqr_sos_nofix64(bn254_64_t *px)
{
    uint64_t w[8];
    uint64_t z[8];
    uint64x2_t t;
    uint8_t c;
    uint64_t *x = px->v;

    // ============== Full Squaring: w = x^2  ============== //

    // d^2
    t = mulx64(x[0], x[0]);
    w[0] = t.l;
    w[1] = t.h;

    // 2cdx
    t = mulx64(x[0], x[1]);
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = c;
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = addc64(w[2], t.h, c, &c); // cannot overflow
    w[2] = addc64(w[2], t.h, 0, &c);
    w[3] = c;

    // c^2x^2 + 2bdx^2
    t = mulx64(x[0], x[2]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c); // cannot overflow
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);
    w[4] = c;
    t = mulx64(x[1], x[1]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);
    w[4] += c;

    // 2adx^3 + 2bcx^3
    t = mulx64(x[0], x[3]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c); // cannot overflow
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] = c;
    t = mulx64(x[1], x[2]);
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] += c;
    w[3] = addc64(w[3], t.l, 0, &c);
    w[4] = addc64(w[4], t.h, c, &c);
    w[5] += c;

    // b^2x^4 + 2acx^4
    t = mulx64(x[1], x[3]);
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c); // cannot overflow
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c);
    w[6] = c;
    t = mulx64(x[2], x[2]);
    w[4] = addc64(w[4], t.l, 0, &c);
    w[5] = addc64(w[5], t.h, c, &c);
    w[6] += c;

    // 2abx^5
    t = mulx64(x[2], x[3]);
    w[5] = addc64(w[5], t.l, 0, &c);
    w[6] = addc64(w[6], t.h, c, &c); // cannot overflow
    w[5] = addc64(w[5], t.l, 0, &c);
    w[6] = addc64(w[6], t.h, c, &c);
    w[7] = c;

    // a^2x^6
    t = mulx64(x[3], x[3]);
    w[6] = addc64(w[6], t.l, 0, &c);
    w[7] = addc64(w[7], t.h, c, &c);

    // ============== Partial multiplication: x = w*PI  ============== //
    t = mulx64(w[0], PI64.v[0]);
    x[0] = t.l;
    x[1] = t.h;

    t = mulx64(w[0], PI64.v[1]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = c;
    x[2] += t.h; // cannot overflow
    t = mulx64(w[1], PI64.v[0]);
    x[1] = addc64(x[1], t.l, 0, &c);
    x[2] = addc64(x[2], t.h, c, &c);
    x[3] = c;

    t = mulx64(w[0], PI64.v[2]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c); // cannot overflow
    t = mulx64(w[1], PI64.v[1]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);
    t = mulx64(w[2], PI64.v[0]);
    x[2] = addc64(x[2], t.l, 0, &c);
    x[3] = addc64(x[3], t.h, c, &c);

    x[3] += w[0] * PI64.v[3];
    x[3] += w[1] * PI64.v[2];
    x[3] += w[2] * PI64.v[1];
    x[3] += w[3] * PI64.v[0];

    // ============== Full multiplication: z = x*P  ============== //
    t = mulx64(x[0], P64.v[0]);
    z[0] = t.l;
    z[1] = t.h;

    t = mulx64(x[0], P64.v[1]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = c;
    z[2] += t.h; // cannot overflow
    t = mulx64(x[1], P64.v[0]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = addc64(z[2], t.h, c, &c);
    z[3] = c;

    t = mulx64(x[0], P64.v[2]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], P64.v[1]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] = c;
    t = mulx64(x[2], P64.v[0]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] += c;

    t = mulx64(x[0], P64.v[3]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c); // cannot overflow
    t = mulx64(x[1], P64.v[2]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] = c;
    t = mulx64(x[2], P64.v[1]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;
    t = mulx64(x[3], P64.v[0]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;

    t = mulx64(x[1], P64.v[3]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c); // cannot overflow
    t = mulx64(x[2], P64.v[2]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] = c;
    t = mulx64(x[3], P64.v[1]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] += c;

    t = mulx64(x[2], P64.v[3]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c); // cannot overflow
    t = mulx64(x[3], P64.v[2]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c);
    z[7] = c;

    t = mulx64(x[3], P64.v[3]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);

    // ============== Final Addition and Shift  ============== //
    addc64(w[0], z[0], 0, &c);
    addc64(w[1], z[1], c, &c);
    addc64(w[2], z[2], c, &c);
    addc64(w[3], z[3], c, &c);
    x[0] = addc64(w[4], z[4], c, &c);
    x[1] = addc64(w[5], z[5], c, &c);
    x[2] = addc64(w[6], z[6], c, &c);
    x[3] = addc64(w[7], z[7], c, &c);
}

CONSTEXPR static inline void bn254_monty_sqr_cios_nofix64(bn254_64_t *px)
{
    /* 
    We use the CIOS algorithm: we only need one extra word (t[4]) since 5p < 2^256
    */
    uint64_t t[5];
    uint64x2_t w;
    uint64_t m;
    uint8_t c;
    uint8_t b;
    uint64_t *x = px->v;

    // ============== Partial multiplication 0 ============== //

    w = mulx64(x[0], x[0]);
    t[0] = w.l;
    t[1] = w.h;

    w = mulx64(x[0], x[1]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[2] = addc64(0, w.h, c, &c);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[2] = addc64(t[2], w.h, c, &c);
    t[3] = c;

    w = mulx64(x[0], x[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[3] = addc64(t[3], w.h, c, &c);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[3] = addc64(t[3], w.h, c, &c);
    t[4] = c;

    w = mulx64(x[0], x[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    b = c;

    // ============== Partial reduction 0 ============== //

    m = t[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], t[1], 0, &c);
    t[1] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[0] = c;
    t[1] = addc64(t[1], t[2], 0, &c);
    t[2] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = c;
    t[2] = addc64(t[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c);
    t[4] = b + c;

    // ============== Partial multiplication 1 ============== //

    w = mulx64(x[1], x[1]);
    t[1] = addc64(t[1], w.l, 0, &c);
    t[2] = addc64(t[2], w.h, c, &c);
    t[0] = c;

    w = mulx64(x[1], x[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);
    t[3] = addc64(t[3], t[0], 0, &c);
    t[4] += c;
    t[2] = addc64(t[2], w.l, 0, &c);
    t[3] = addc64(t[3], w.h, c, &c);
    t[4] += c;

    w = mulx64(x[1], x[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    b = c;

    // ============== Partial reduction 1 ============== //

    // PI64.v[0] * P64.v[0] = -1, so w.l = -x[0]  and w.l + x[0] = 0 with carry 1
    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    x[1] = c;
    x[0] = addc64(x[0], t[1], 0, &c);
    x[1] = addc64(x[1], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(x[1], w.l, 0, &c);
    t[0] = c;
    x[1] = addc64(x[1], t[2], 0, &c);
    t[2] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[0] = c;
    t[2] = addc64(t[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c);
    t[4] = c + b;

    // ============== Partial multiplication 2 ============== //
    w = mulx64(x[2], x[2]);
    t[2] = addc64(t[2], w.l, 0, &c);
    t[3] = addc64(t[3], w.h, c, &c);
    t[4] += c;

    w = mulx64(x[2], x[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);
    b = c;

    // ============== Partial reduction 2 ============== //

    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], x[1], 0, &c);
    x[1] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(x[1], w.l, 0, &c);
    x[2] = c;
    x[1] = addc64(x[1], t[2], 0, &c);
    x[2] = addc64(x[2], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    x[2] = addc64(x[2], w.l, 0, &c);
    t[0] = c;
    x[2] = addc64(x[2], t[3], 0, &c);
    t[3] = addc64(t[0], w.h, c, &c);

    t[3] = addc64(t[3], t[4], 0, &c);
    t[4] = c + b;


    // ============== Partial multiplication 3 ============== //
    w = mulx64(x[3], x[3]);
    t[3] = addc64(t[3], w.l, 0, &c);
    t[4] = addc64(t[4], w.h, c, &c);

    // ============== Partial reduction 3 ============== //

    m = x[0] * PI64.v[0];
    w = mulx64(m, P64.v[0]);
    x[0] = w.h + !!x[0];

    w = mulx64(m, P64.v[1]);
    x[0] = addc64(x[0], w.l, 0, &c);
    t[0] = c;
    x[0] = addc64(x[0], x[1], 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[2]);
    x[1] = addc64(t[0], w.l, 0, &c);
    t[0] = c;
    x[1] = addc64(x[1], x[2], 0, &c);
    t[0] = addc64(t[0], w.h, c, &c);

    w = mulx64(m, P64.v[3]);
    x[2] = addc64(t[0], w.l, 0, &c);
    x[3] = c;
    x[2] = addc64(x[2], t[3], 0, &c);
    x[3] = addc64(x[3], w.h, c, &c);

    x[3] = addc64(x[3], t[4], 0, &c);
}

CONSTEXPR static inline void bn254_monty_sqr_nofix64(bn254_64_t *px)
{
    bn254_monty_sqr_cios_nofix64(px);
}

CONSTEXPR static inline void bn254_monty_sqr64(bn254_64_t *px)
{
    bn254_monty_sqr_nofix64(px);
    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_into_the_montyverse64(bn254_64_t *px)
{
    // To enter the Montyverse, we multiply by R2
    bn254_monty_mul64(px, &R2_64);
}

CONSTEXPR static inline void bn254_outta_the_montyverse64(bn254_64_t *px)
{
    // To come back from the Montyverse, we multiply by 1

    uint64_t w[4];
    uint64_t z[8];
    uint64x2_t t;
    uint8_t c;
    uint64_t *x = px->v;


    // ============== Partial multiplication: w = x*PI  ============== //
    t = mulx64(x[0], PI64.v[0]);
    w[0] = t.l;
    w[1] = t.h;

    t = mulx64(x[0], PI64.v[1]);
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = c;
    w[2] += t.h; // cannot overflox
    t = mulx64(x[1], PI64.v[0]);
    w[1] = addc64(w[1], t.l, 0, &c);
    w[2] = addc64(w[2], t.h, c, &c);
    w[3] = c;

    t = mulx64(x[0], PI64.v[2]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c); // cannot overflox
    t = mulx64(x[1], PI64.v[1]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);
    t = mulx64(x[2], PI64.v[0]);
    w[2] = addc64(w[2], t.l, 0, &c);
    w[3] = addc64(w[3], t.h, c, &c);

    w[3] += x[0] * PI64.v[3];
    w[3] += x[1] * PI64.v[2];
    w[3] += x[2] * PI64.v[1];
    w[3] += x[3] * PI64.v[0];


    // ============== Full multiplication: z = w*P  ============== //
    t = mulx64(w[0], P64.v[0]);
    z[0] = t.l;
    z[1] = t.h;

    t = mulx64(w[0], P64.v[1]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = c;
    z[2] += t.h; // cannot overflow
    t = mulx64(w[1], P64.v[0]);
    z[1] = addc64(z[1], t.l, 0, &c);
    z[2] = addc64(z[2], t.h, c, &c);
    z[3] = c;

    t = mulx64(w[0], P64.v[2]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c); // cannot overflow
    t = mulx64(w[1], P64.v[1]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] = c;
    t = mulx64(w[2], P64.v[0]);
    z[2] = addc64(z[2], t.l, 0, &c);
    z[3] = addc64(z[3], t.h, c, &c);
    z[4] += c;

    t = mulx64(w[0], P64.v[3]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c); // cannot overflow
    t = mulx64(w[1], P64.v[2]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] = c;
    t = mulx64(w[2], P64.v[1]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;
    t = mulx64(w[3], P64.v[0]);
    z[3] = addc64(z[3], t.l, 0, &c);
    z[4] = addc64(z[4], t.h, c, &c);
    z[5] += c;

    t = mulx64(w[1], P64.v[3]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c); // cannot overflow
    t = mulx64(w[2], P64.v[2]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] = c;
    t = mulx64(w[3], P64.v[1]);
    z[4] = addc64(z[4], t.l, 0, &c);
    z[5] = addc64(z[5], t.h, c, &c);
    z[6] += c;

    t = mulx64(w[2], P64.v[3]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c); // cannot overflow
    t = mulx64(w[3], P64.v[2]);
    z[5] = addc64(z[5], t.l, 0, &c);
    z[6] = addc64(z[6], t.h, c, &c);
    z[7] = c;

    t = mulx64(w[3], P64.v[3]);
    z[6] = addc64(z[6], t.l, 0, &c);
    z[7] = addc64(z[7], t.h, c, &c);

    // ============== Final Addition and Shift  ============== //
    addc64(x[0], z[0], 0, &c);
    addc64(x[1], z[1], c, &c);
    addc64(x[2], z[2], c, &c);
    addc64(x[3], z[3], c, &c);
    x[0] = addc64(0, z[4], c, &c);
    x[1] = addc64(0, z[5], c, &c);
    x[2] = addc64(0, z[6], c, &c);
    x[3] = addc64(0, z[7], c, &c);

    bn254_fix_of64(px);
}

CONSTEXPR static inline void bn254_pow64(bn254_64_t *px, const bn254_64_t *py)
{
    bn254_64_t z = {1};
    bn254_64_t w = *px;
    const uint64_t *y = py->v;
    uint8_t b = !!y[0] << 0 | !!y[1] << 1 | !!y[2] << 2;

#pragma unroll(4)
    for (size_t i = 0; i < 4; ++i, b >>= 1)
        for (uint64_t e = y[i], j = 64; e | (b * j); e >>= 1, --j)
        {
            if (e & 1)
                bn254_mul64(&z, &w);
            bn254_sqr64(&w);
        }

    *px = z;
}

CONSTEXPR static inline void bn254_inv64(bn254_64_t *px)
{
    bn254_64_t t[8];

    bn254_into_the_montyverse64(px);

    bn254_64_t x = *px;
    t[0] = x;
    bn254_monty_sqr64(&t[0]);
    t[4] = t[0];
    bn254_monty_mul64(&t[4], &x);
    t[2] = t[4];
    bn254_monty_mul64(&t[2], &t[0]);
    *px = t[2];
    bn254_monty_mul64(px, &t[0]);
    t[3] = *px;
    bn254_monty_mul64(&t[3], &t[0]);
    t[1] = t[3];
    bn254_monty_mul64(&t[1], &t[0]);
    t[5] = t[1];
    bn254_monty_mul64(&t[5], &t[0]);
    t[6] = t[5];
    bn254_monty_mul64(&t[6], &t[0]);
    t[7] = t[6];
    bn254_monty_mul64(&t[7], &t[3]);
    t[0] = t[7];
    bn254_monty_mul64(&t[0], px);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &x);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[3]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], px);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[1]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &x);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[3]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &x);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[5]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[5]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &x);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[1]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[5]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[1]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[2]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[4]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[0]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &x);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[7], &t[3]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_sqr64(&t[7]);
    bn254_monty_mul64(&t[6], &t[7]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_sqr64(&t[6]);
    bn254_monty_mul64(&t[5], &t[6]);
    bn254_monty_sqr64(&t[5]);
    bn254_monty_sqr64(&t[5]);
    bn254_monty_mul64(&t[4], &t[5]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_mul64(&t[4], &t[1]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_mul64(&t[4], &x);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_sqr64(&t[4]);
    bn254_monty_mul64(&t[3], &t[4]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_sqr64(&t[3]);
    bn254_monty_mul64(&t[2], &t[3]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_mul64(&t[2], &t[0]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_mul64(&t[2], &t[0]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_sqr64(&t[2]);
    bn254_monty_mul64(&t[1], &t[2]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &x);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &t[0]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &t[0]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &t[0]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &t[0]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[1], &t[0]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_sqr64(&t[1]);
    bn254_monty_mul64(&t[0], &t[1]);
    bn254_monty_sqr64(&t[0]);
    bn254_monty_sqr64(&t[0]);
    bn254_monty_sqr64(&t[0]);
    bn254_monty_mul64(px, &t[0]);

    bn254_outta_the_montyverse64(px);
}

static inline bn254_32_t bn254_rand32(void)
{
    /*
    We generate random numbers by rejection sampling. 
    Since 5.3*p ~= 2^256, we accept values between [0, 5p) (each congruence class is equiprobable) 
    and reject between [5p, 2^256) (it would skew the balance for small elements).
    It is expected to terminate after 1.06 iterations.
    We could also clear the higher bits, obtaining a value in [0, 2^254), and 
    reject between [p, 2^254), with expected 1.32 iterations.
    However, overflow correction is much faster than TRNG.
    */
    bn254_32_t x;

    do
    {
        x.v[0] = rand32();
        x.v[1] = rand32();
        x.v[2] = rand32();
        x.v[3] = rand32();
        x.v[4] = rand32();
        x.v[5] = rand32();
        x.v[6] = rand32();
        x.v[7] = rand32();

        bn254_fix_of32(&x);
        bn254_fix_of32(&x);
        bn254_fix_of32(&x);
        bn254_fix_of32(&x);
    } while (bn254_check_of32(&x));

    return x;
}

static inline bn254_64_t bn254_rand64(void)
{
    bn254_64_t x;

    do
    {
        x.v[0] = rand64();
        x.v[1] = rand64();
        x.v[2] = rand64();
        x.v[3] = rand64();

        bn254_fix_of64(&x);
        bn254_fix_of64(&x);
        bn254_fix_of64(&x);
        bn254_fix_of64(&x);
    } while (bn254_check_of64(&x));

    return x;
}

#ifdef __cplusplus
}
#endif
