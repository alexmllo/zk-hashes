#include "mimc.h"

#include <stdio.h>
#include <string.h>

#define lengthof(x) (sizeof(x) / sizeof((x)[0]))

int main(void)
{
    bn254_64_t key[] = {
        {REVERSE64(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)},
        {REVERSE64(0x0000746869732069, 0x7320746865207279, 0x74686d206f662074, 0x6865206e69676874)},
        {REVERSE64(0x00446f2c206f7220, 0x646f206e6f742c20, 0x7468657265206973, 0x206e6f2074727921)},
        {REVERSE64(0x0c0ffeec0ffeec0f, 0xfeec0ffeec0ffeec, 0x0ffeec0ffeec0ffe, 0xec0ffeec0ffeec0f)},
        {REVERSE64(0x30644e72e131a029, 0xb85045b68181585d, 0x2833e84879b97091, 0x43e1f593f0000000)},
        {REVERSE64(0x1000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)},
        {REVERSE64(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000001)},
    };
    bn254_64_t msg[] = {
        {REVERSE64(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)},
        {REVERSE64(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000001)},
        {REVERSE64(0x1000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)},
        {REVERSE64(0x30644e72e131a029, 0xb85045b68181585d, 0x2833e84879b97091, 0x43e1f593f0000000)},
        {REVERSE64(0x0c0ffeec0ffeec0f, 0xfeec0ffeec0ffeec, 0x0ffeec0ffeec0ffe, 0xec0ffeec0ffeec0f)},
        {REVERSE64(0x00446f2c206f7220, 0x646f206e6f742c20, 0x7468657265206973, 0x206e6f2074727921)},
        {REVERSE64(0x0000746869732069, 0x7320746865207279, 0x74686d206f662074, 0x6865206e69676874)},
    };

    bn254_64_t exp[] = {
        {REVERSE64(0x16b65105f9c0d309, 0x2d8e896b4bb3680c, 0xed2ab52096d6f70a, 0x7e8581954f59aa9d)},
        {REVERSE64(0x27156fc8d1316988, 0xbb371f0b9997c9d6, 0x0d7d683ae15f7b9f, 0x7d1db6aee0318a3a)},
        {REVERSE64(0x0b452cd0f0f600c2, 0x7d75a4850e2a207e, 0xee6c9534aaaf7419, 0x2738c13f852e5b67)},
        {REVERSE64(0x23676df46d7429e0, 0xc5745bfd7b29ed17, 0xb12576f4750cc8b4, 0x197513d4d99384b3)},
        {REVERSE64(0x1fa9e8807e169103, 0x53e6741b30c96610, 0x99650957e568884c, 0xf020d4895b453137)},
        {REVERSE64(0x24b12aa498c191b8, 0x379e303a4207899a, 0x24d63fd93d94474d, 0xb55339cc552392f5)},
        {REVERSE64(0x06e66741035992b5, 0xddaa744aff7b996e, 0xed4611d29c90e6df, 0xdfb0a0b7616084f7)},
    };


    for (size_t i = 0; i < lengthof(msg); ++i)
    {
        bn254_64_t cip = msg[i];
        mimc_enc64(&cip, &key[i]);

        bn254_64_t mip = msg[i];
        mimc_enc64_v2(&mip, &key[i]);

        printf("Msg%zu: ", i);
        for (size_t j = 0; j < 4; ++j)
            printf("%016" PRIx64, msg[i].v[3 - j]);
        printf("\n");

        printf("Key%zu: ", i);
        for (size_t j = 0; j < 4; ++j)
            printf("%016" PRIx64, key[i].v[3 - j]);
        printf("\n");

        printf("Cip%zu: ", i);
        for (size_t j = 0; j < 4; ++j)
            printf("%016" PRIx64, cip.v[3 - j]);

        int monty_check = !memcmp(&cip, &mip, sizeof(cip));
        int exp_check = !memcmp(&cip, &exp[i], sizeof(cip));

        printf(" (monty = %d, check = %d)\n\n", monty_check, exp_check);
    }

    return 0;
}
