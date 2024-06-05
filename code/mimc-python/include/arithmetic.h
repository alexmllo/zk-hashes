#pragma once
#include "intrinsics.h"

typedef struct
{
    uint64_t l;
    uint64_t h;
} uint64x2_t;

// useful to convert from big endian to little endian
#define REVERSE32(x0, x1, x2, x3, x4, x5, x6, x7) (x7), (x6), (x5), (x4), (x3), (x2), (x1), (x0)
#define REVERSE64(x0, x1, x2, x3) (x3), (x2), (x1), (x0)

#ifdef __cplusplus
    #include <type_traits>
    #define CONSTEXPR constexpr
    #define RESTRICT __restrict__
#else
    #define CONSTEXPR
    #define RESTRICT restrict
#endif

#ifdef __cplusplus
extern "C" {
#endif

CONSTEXPR static inline uint32_t addc32(uint32_t x, uint32_t y, uint8_t c, uint8_t *d)
{
#ifdef __cplusplus
    if (std::is_constant_evaluated())
    {
        x += y;
        *d = x < y;
        x += c;
        *d |= x < c;

        return x;
    }
    else
    {
#endif
        *d = _addc32(c, x, y, &x);
        return x;
#ifdef __cplusplus
    }
#endif
}

CONSTEXPR static inline uint64_t addc64(uint64_t x, uint64_t y, uint8_t c, uint8_t *d)
{
#ifdef __cplusplus
    if (std::is_constant_evaluated())
    {
        x += y;
        *d = x < y;
        x += c;
        *d |= x < c;

        return x;
    }
    else
    {
#endif
        *d = _addc64(c, x, y, &x);
        return x;
#ifdef __cplusplus
    }
#endif
}

CONSTEXPR static inline uint32_t subb32(uint32_t x, uint32_t y, uint8_t c, uint8_t *d)
{
#ifdef __cplusplus
    if (std::is_constant_evaluated())
    {
        uint32_t z = x;

        x -= y;
        *d = x > z;
        z = x;
        x -= c;
        *d |= x > z;

        return x;
    }
    else
    {
#endif
        *d = _subb32(c, x, y, &x);
        return x;
#ifdef __cplusplus
    }
#endif
}

CONSTEXPR static inline uint64_t subb64(uint64_t x, uint64_t y, uint8_t c, uint8_t *d)
{
#ifdef __cplusplus
    if (std::is_constant_evaluated())
    {
        uint32_t z = x;

        x -= y;
        *d = x > z;
        z = x;
        x -= c;
        *d |= x > z;

        return x;
    }
    else
    {
#endif
        *d = _subb64(c, x, y, &x);
        return x;
#ifdef __cplusplus
    }
#endif
}

CONSTEXPR static inline uint64x2_t mulx64(uint64_t x, uint64_t y)
{
    uint64x2_t z;

    z.l = _mulx64(x, y, &z.h);

    return z;
}
#ifdef __cplusplus
}
#endif
