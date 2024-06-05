#pragma once

#include "bn_254.h"

#ifdef __cplusplus
extern "C" {
#endif

// Encrypt in-place with MiMC
void mimc_enc64(bn254_64_t *msg, const bn254_64_t *key);

// Encrypt in-place with MiMC using montgomery multiplication
void mimc_enc64_v2(bn254_64_t *msg, const bn254_64_t *key);

#ifdef __cplusplus
}
#endif
