#include "rand.h"
#include <random>

extern "C" {
uint32_t rand32()
{
    static std::random_device rng;

    return rng();
}

uint64_t rand64()
{
    return (uint64_t)rand32() << 32 | rand32();
}
}
