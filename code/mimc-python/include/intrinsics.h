#pragma once

#include <inttypes.h>
#include <limits.h>

#if defined(__INTEL_COMPILER) || defined(__INTEL_LLVM_COMPILER)
    #define INTEL_COMPILER
#endif

#if defined(__x86_64__) || defined(_M_X64)
    #define X86_64
#endif

#ifdef X86_64
    #ifdef _WIN32
        #include <intrin.h>
    #else
        #include <x86intrin.h>
        #define _rotr64 _lrotr
        #define _rotl64 _lrotl
    #endif
#endif

#ifndef CHAR_WIDTH
    #define CHAR_WIDTH CHAR_BIT
    #define UCHAR_WIDTH CHAR_WIDTH
#endif
#ifndef SHRT_WIDTH
    #if USHRT_MAX == 0xffULL
        #define SHRT_WIDTH 8
    #elif USHRT_MAX == 0xffffULL
        #define SHRT_WIDTH 16
    #elif USHRT_MAX == 0xffffffffULL
        #define SHRT_WIDTH 32
    #elif USHRT_MAX == 0xffffffffffffffffULL
        #define SHRT_WIDTH 64
    #endif
    #define USHRT_WIDTH SHRT_WIDTH
#endif
#ifndef INT_WIDTH
    #if UINT_MAX == 0xffULL
        #define INT_WIDTH 8
    #elif UINT_MAX == 0xffffULL
        #define INT_WIDTH 16
    #elif UINT_MAX == 0xffffffffULL
        #define INT_WIDTH 32
    #elif UINT_MAX == 0xffffffffffffffffULL
        #define INT_WIDTH 64
    #endif
    #define UINT_WIDTH INT_WIDTH
#endif
#ifndef LONG_WIDTH
    #if ULONG_MAX == 0xffULL
        #define LONG_WIDTH 8
    #elif ULONG_MAX == 0xffffULL
        #define LONG_WIDTH 16
    #elif ULONG_MAX == 0xffffffffULL
        #define LONG_WIDTH 32
    #elif ULONG_MAX == 0xffffffffffffffffULL
        #define LONG_WIDTH 64
    #endif
    #define ULONG_WIDTH LONG_WIDTH
#endif
#ifndef LLONG_WIDTH
    #if ULLONG_MAX == 0xffULL
        #define LLONG_WIDTH 8
    #elif ULLONG_MAX == 0xffffULL
        #define LLONG_WIDTH 16
    #elif ULLONG_MAX == 0xffffffffULL
        #define LLONG_WIDTH 32
    #elif ULLONG_MAX == 0xffffffffffffffffULL
        #define LLONG_WIDTH 64
    #endif
    #define ULLONG_WIDTH LLONG_WIDTH
#endif

// Godbolt says ARM-v7e-m supports _BitInt(128) on Clang 16/17...
#if defined(__SIZEOF_INT128__)
    #define NATIVE_UINT128 1
typedef unsigned __int128 uint128_t;
#elif !defined(_MSC_VER) && __STDC_VERSION__ >= 202000L
    #define NATIVE_UINT128 1
typedef unsigned _BitInt(128) uint128_t;
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if defined(X86_64) && !defined(_WIN32)
inline uint64_t _udiv128(uint64_t hi, uint64_t lo, uint64_t div, uint64_t *rem)
{
    // High bits go in RDX, low bits in RAX, quotient is in RAX, remainder is in RDX
    __asm__ inline( //
        "divq %4"
        : "=d"(hi), "=a"(lo)
        : "d"(hi), "a"(lo), "rm"(div));

    *rem = hi;

    return lo;
}

inline uint64_t _umul128(uint64_t x, uint64_t y, uint64_t *hi)
{
    // x goes in RAX, y goes in RDX, high bits go in RDX, low bits in RAX
    __asm__ inline( //
        "mul %1"
        : "=a"(x), "=d"(y)
        : "a"(x), "d"(y));
    *hi = y;

    return x;
}
#endif

// If it does not have addc, I doubt it has any of the others. For now we don't use inline assembly
#if defined(_MSC_VER) || !defined(__has_builtin) || !__has_builtin(__builtin_addc)
static inline unsigned int __builtin_addc(unsigned int a, unsigned int b, unsigned int c,
                                          unsigned int *d)
{
    a += b;
    *d = a < b;
    a += c;
    *d |= a < c;

    return a;
}

static inline unsigned long __builtin_addcl(unsigned long a, unsigned long b, unsigned long c,
                                            unsigned long *d)
{
    a += b;
    *d = a < b;
    a += c;
    *d |= a < c;

    return a;
}

static inline unsigned long long __builtin_addcll(unsigned long long a, unsigned long long b,
                                                  unsigned long long c, unsigned long long *d)
{
    a += b;
    *d = a < b;
    a += c;
    *d |= a < c;

    return a;
}

static inline unsigned int __builtin_subc(unsigned int a, unsigned int b, unsigned int c,
                                          unsigned int *d)
{
    unsigned int e = a;

    a -= b;
    *d = a > e;
    e = a;
    a -= c;
    *d |= a > e;

    return a;
}

static inline unsigned long __builtin_subcl(unsigned long a, unsigned long b, unsigned long c,
                                            unsigned long *d)
{
    unsigned long e = a;

    a -= b;
    *d = a > e;
    e = a;
    a -= c;
    *d |= a > e;

    return a;
}

static inline unsigned long long __builtin_subcll(unsigned long long a, unsigned long long b,
                                                  unsigned long long c, unsigned long long *d)
{
    unsigned long long e = a;

    a -= b;
    *d = a > e;
    e = a;
    a -= c;
    *d |= a > e;

    return a;
}
#endif

static inline unsigned char _addc(unsigned char c, unsigned int x, unsigned int y, unsigned int *z)
{
#ifdef X86_64
    return _addcarry_u32(c, x, y, z);
#else
    *z = __builtin_addc(x, y, c, &x);

    return (unsigned char)x;
#endif
}

static inline unsigned char _addcl(unsigned char c, unsigned long x, unsigned long y,
                                   unsigned long *z)
{
#ifdef X86_64
    #if ULONG_WIDTH == 32
    return _addcarry_u32(c, x, y, (unsigned int *)z);
    #elif ULONG_WIDTH == 64
    return _addcarry_u64(c, x, y, (unsigned long long *)z);
    #endif
#else
    *z = __builtin_addcl(x, y, c, &x);

    return (unsigned char)x;
#endif
}

static inline unsigned char _addcll(unsigned char c, unsigned long long x, unsigned long long y,
                                    unsigned long long *z)
{
#ifdef X86_64
    return _addcarry_u64(c, x, y, z);
#else
    *z = __builtin_addcll(x, y, c, &x);

    return (unsigned char)x;
#endif
}


static inline unsigned char _subb(unsigned char c, unsigned int x, unsigned int y, unsigned int *z)
{
#ifdef X86_64
    return _subborrow_u32(c, x, y, z);
#else
    *z = __builtin_subc(x, y, c, &x);

    return (unsigned char)x;
#endif
}

static inline unsigned char _subbl(unsigned char c, unsigned long x, unsigned long y,
                                   unsigned long *z)
{
#ifdef X86_64
    #if ULONG_WIDTH == 32
    return _subborrow_u32(c, x, y, (unsigned int *)z);
    #elif ULONG_WIDTH == 64
    return _subborrow_u64(c, x, y, (unsigned long long *)z);
    #endif
#else
    *z = __builtin_subcl(x, y, c, &x);

    return (unsigned char)x;
#endif
}

static inline unsigned char _subbll(unsigned char c, unsigned long long x, unsigned long long y,
                                    unsigned long long *z)
{
#ifdef X86_64
    return _subborrow_u64(c, x, y, z);
#else
    *z = __builtin_subcll(x, y, c, &x);
#endif

    return (unsigned char)x;
}

static inline unsigned int _mulx(unsigned int x, unsigned int y, unsigned int *h)
{
    uint64_t z = (uint64_t)x * y;

    *h = (unsigned int)(z >> 32);

    return (unsigned int)z;
}

static inline unsigned long _mulxl(unsigned long x, unsigned long y, unsigned long *h)
{
#if ULONG_WIDTH == 32
    uint64_t z = (uint64_t)x * y;

    *h = (unsigned long)(z >> 32);

    return (unsigned long)z;
#elif defined(X86_64)
    return (unsigned long)_mulx_u64(x, y, (unsigned long long *)h);
#elif defined(NATIVE_UINT128)
    uint128_t z = (uint128_t)x * y;

    *h = (unsigned long)(z >> 64);

    return (unsigned long)z;
#else
    unsigned long xl = x & 0xFFFFFFFF;
    unsigned long xh = x >> 32;
    unsigned long yl = y & 0xFFFFFFFF;
    unsigned long yh = y >> 32;
    uint8_t c;

    x = xl * yl;
    y = xh * yh;
    xl *= yh;
    yl *= xh;
    c = _addcl(0, x, xl << 32, &x);
    _addcll(c, y, xl >> 32, &y);
    c = _addcl(0, x, yl << 32, &x);
    _addcl(c, y, yl >> 32, h);

    return x;
#endif
}

static inline unsigned long long _mulxll(unsigned long long x, unsigned long long y,
                                         unsigned long long *h)
{
#if ULLONG_WIDTH == 32
    uint64_t z = (uint64_t)x * y;

    *h = (unsigned long)(z >> 32);

    return (unsigned long)z;
#elif defined(X86_64)
    return _mulx_u64(x, y, h);
#elif defined(NATIVE_UINT128)
    uint128_t z = (uint128_t)x * y;

    *h = (unsigned long long)(z >> 64);

    return (unsigned long long)z;
#else
    unsigned long long xl = x & 0xFFFFFFFF;
    unsigned long long xh = x >> 32;
    unsigned long long yl = y & 0xFFFFFFFF;
    unsigned long long yh = y >> 32;
    uint8_t c;

    x = xl * yl;
    y = xh * yh;
    xl *= yh;
    yl *= xh;
    c = _addcll(0, x, xl << 32, &x);
    _addcll(c, y, xl >> 32, &y);
    c = _addcll(0, x, yl << 32, &x);
    _addcll(c, y, yl >> 32, h);

    return x;
#endif
}

#ifdef __cplusplus
}
#endif

#if UINT_WIDTH == 32
    #define _addc32 _addc
    #define _subb32 _subb
    #define _mulx32 _mulx
#elif ULONG_WIDTH == 32
    #define _addc32 _addcl
    #define _subb32 _subbl
    #define _mulx32 _mulxl
#elif ULLONG_WIDTH == 32
    #define _addc32 _addcll
    #define _subb32 _subbll
    #define _mulx32 _mulxll
#endif

#if UINT_WIDTH == 64
    #define _addc64 _addc
    #define _subb64 _subb
    #define _mulx64 _mulx
#elif ULONG_WIDTH == 64
    #define _addc64 _addcl
    #define _subb64 _subbl
    #define _mulx64 _mulxl
#elif ULLONG_WIDTH == 64
    #define _addc64 _addcll
    #define _subb64 _subbll
    #define _mulx64 _mulxll
#endif


#ifdef INTEL_COMPILER
    #undef INTEL_COMPILER
#endif
