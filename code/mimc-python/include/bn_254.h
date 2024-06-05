#pragma once

#include "arithmetic.h"
#include "rand.h"
#include <inttypes.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct
{
    uint32_t v[8]; // little endian representation
} bn254_32_t;

typedef struct
{
    uint64_t v[4]; // little endian representation
} bn254_64_t;

// Field equality comparison with 32-bit digits
CONSTEXPR static inline uint8_t bn254_eql32(const bn254_32_t *x, const bn254_32_t *y);

// Field equality comparison with 64-bit digits
CONSTEXPR static inline uint8_t bn254_eql64(const bn254_64_t *x, const bn254_64_t *y);

// Check for overflow
CONSTEXPR static inline uint8_t bn254_check_of32(const bn254_32_t *px);

// Check for overflow
CONSTEXPR static inline uint8_t bn254_check_of64(const bn254_64_t *px);

// Branchless field small overflow fix
CONSTEXPR static inline void bn254_fix_of32(bn254_32_t *px);

// Branchless field small overflow fix
CONSTEXPR static inline void bn254_fix_of64(bn254_64_t *px);

// Branchless field small underflow fix
CONSTEXPR static inline void bn254_fix_uf32(bn254_32_t *px);

// Branchless field small underflow fix
CONSTEXPR static inline void bn254_fix_uf64(bn254_64_t *px);

// Field addition without the last fix step
CONSTEXPR static inline void bn254_add_nofix32(bn254_32_t *x, const bn254_32_t *y);

// Field addition using 64-bit digits without the last fix step
CONSTEXPR static inline void bn254_add_nofix64(bn254_64_t *x, const bn254_64_t *y);

// Field addition
CONSTEXPR static inline void bn254_add32(bn254_32_t *x, const bn254_32_t *y);

// Field addition
CONSTEXPR static inline void bn254_add64(bn254_64_t *x, const bn254_64_t *y);

// Field increment without overflow fix
CONSTEXPR static inline void bn254_inc_nofix32(bn254_32_t *x);

// Field increment without overflow fix
CONSTEXPR static inline void bn254_inc_nofix64(bn254_64_t *x);

// Field increment
CONSTEXPR static inline void bn254_inc32(bn254_32_t *x);

// Field increment
CONSTEXPR static inline void bn254_inc64(bn254_64_t *x);

// Field negation
CONSTEXPR static inline void bn254_neg32(bn254_32_t *px);

// Field negation
CONSTEXPR static inline void bn254_neg64(bn254_64_t *px);

// Field subtraction without underflow fix
CONSTEXPR static inline void bn254_sub_nofix32(bn254_32_t *px, const bn254_32_t *py);

// Field subtraction without underflow fix
CONSTEXPR static inline void bn254_sub_nofix64(bn254_64_t *px, const bn254_64_t *py);

// Field subtraction
CONSTEXPR static inline void bn254_sub32(bn254_32_t *px, const bn254_32_t *py);

// Field subtraction
CONSTEXPR static inline void bn254_sub64(bn254_64_t *px, const bn254_64_t *py);

// Field decrement without overflow fix
CONSTEXPR static inline void bn254_dec_nofix32(bn254_32_t *x);

// Field decrement without overflow fix
CONSTEXPR static inline void bn254_dec_nofix64(bn254_64_t *x);

// Field decrement
CONSTEXPR static inline void bn254_dec32(bn254_32_t *x);

// Field decrement
CONSTEXPR static inline void bn254_dec64(bn254_64_t *x);

// Field multiplication using 64-bit digits
CONSTEXPR static inline void bn254_mul64(bn254_64_t *x, const bn254_64_t *y);

// Field squaring using 64-bit digits
CONSTEXPR static inline void bn254_sqr64(bn254_64_t *x);

// Perform multiplication in the Montgomery space without the last fix step
CONSTEXPR static inline void bn254_monty_mul_nofix64(bn254_64_t *px, const bn254_64_t *py);

// Perform multiplication in the Montgomery space
CONSTEXPR static inline void bn254_monty_mul64(bn254_64_t *px, const bn254_64_t *py);

// Perform squaring in the Montgomery space without the last fix step
CONSTEXPR static inline void bn254_monty_sqr_nofix64(bn254_64_t *px);

// Perform squaring in the Montgomery space
CONSTEXPR static inline void bn254_monty_sqr64(bn254_64_t *px);

// Bring x in the Montgomery space
CONSTEXPR static inline void bn254_into_the_montyverse64(bn254_64_t *x);

// Bring x back in normal form
CONSTEXPR static inline void bn254_outta_the_montyverse64(bn254_64_t *px);

// Field exponentiation
CONSTEXPR static inline void bn254_pow64(bn254_64_t *px, const bn254_64_t *py);

// Field inversion
CONSTEXPR static inline void bn254_inv64(bn254_64_t *px);

// Field RNG
static inline bn254_32_t bn254_rand32(void);

// Field RNG
static inline bn254_64_t bn254_rand64(void);

#ifdef __cplusplus
}
#endif

#include "bn_254.tcc"

#ifdef __cplusplus
constexpr static inline bn254_64_t &operator+=(bn254_64_t &x, const bn254_64_t &y)
{
    bn254_add64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t &operator+=(bn254_64_t &&x, const bn254_64_t &y)
{
    bn254_add64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t operator+(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_64_t{x} += y;
}

constexpr static inline bn254_64_t &operator++(bn254_64_t &x)
{
    bn254_inc64(&x);

    return x;
}

constexpr static inline bn254_64_t &operator++(bn254_64_t &&x)
{
    bn254_inc64(&x);

    return x;
}

constexpr static inline bn254_64_t operator++(bn254_64_t &x, int)
{
    bn254_64_t t{x};

    ++x;

    return t;
}

constexpr static inline bn254_64_t operator++(bn254_64_t &&x, int)
{
    bn254_64_t t{x};

    ++x;

    return t;
}

constexpr static inline bn254_64_t &operator-=(bn254_64_t &x, const bn254_64_t &y)
{
    bn254_sub64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t &operator-=(bn254_64_t &&x, const bn254_64_t &y)
{
    bn254_sub64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t operator-(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_64_t{x} -= y;
}

constexpr static inline bn254_64_t operator-(const bn254_64_t &x)
{
    bn254_64_t y{x};

    bn254_neg64(&y);

    return y;
}

constexpr static inline bn254_64_t &operator--(bn254_64_t &x)
{
    bn254_dec64(&x);

    return x;
}

constexpr static inline bn254_64_t &operator--(bn254_64_t &&x)
{
    bn254_dec64(&x);

    return x;
}

constexpr static inline bn254_64_t operator--(bn254_64_t &x, int)
{
    bn254_64_t t{x};

    --x;

    return t;
}

constexpr static inline bn254_64_t operator--(bn254_64_t &&x, int)
{
    bn254_64_t t{x};

    --x;

    return t;
}

constexpr static inline bn254_64_t &operator*=(bn254_64_t &x, const bn254_64_t &y)
{
    bn254_mul64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t &operator*=(bn254_64_t &&x, const bn254_64_t &y)
{
    bn254_mul64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t operator*(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_64_t{x} *= y;
}

constexpr static inline bn254_64_t &operator^=(bn254_64_t &x, const bn254_64_t &y)
{
    bn254_pow64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t &operator^=(bn254_64_t &&x, const bn254_64_t &y)
{
    bn254_pow64(&x, &y);

    return x;
}

constexpr static inline bn254_64_t operator^(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_64_t{x} ^= y;
}

constexpr static inline bn254_64_t operator~(const bn254_64_t &x)
{
    bn254_64_t y{x};

    bn254_inv64(&y);

    return y;
}


constexpr static inline bn254_64_t &operator/=(bn254_64_t &x, const bn254_64_t &y)
{
    return x *= ~y;
}

constexpr static inline bn254_64_t &operator/=(bn254_64_t &&x, const bn254_64_t &y)
{
    return x *= ~y;
}

constexpr static inline bn254_64_t operator/(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_64_t{x} /= y;
}

constexpr static inline bool operator==(const bn254_64_t &x, const bn254_64_t &y)
{
    return bn254_eql64(&x, &y);
}

#endif