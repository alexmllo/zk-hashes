use dusk_bls12_381::BlsScalar;

// WIDTH=8 -> 6
// WIDTH=4 -> 2
pub const ALPHAS: [BlsScalar; 6] = {
    let bytes = include_bytes!("../../assets/griffin/alphas_8.bin");

    let mut cnst = [BlsScalar::zero(); 6];

    let mut j = 0;
    let mut i = 0;
    while i < 6 * 32 {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j] = BlsScalar::from_raw([a, b, c, d]);
        j += 1;

        i += 32;
    }
    cnst
};

// WIDTH=8 -> 6
// WIDTH=4 -> 2
pub const BETAS: [BlsScalar; 6] = {
    let bytes = include_bytes!("../../assets/griffin/betas_8.bin");

    let mut cnst = [BlsScalar::zero(); 6];

    let mut j = 0;
    let mut i = 0;
    while i < 6 * 32 {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j] = BlsScalar::from_raw([a, b, c, d]);
        j += 1;

        i += 32;
    }
    cnst
};

// WIDTH=8 -> 64
// WIDTH=4 -> 40
pub const ROUND_CONSTANTS: [BlsScalar; 64] = {
    let bytes = include_bytes!("../../assets/griffin/round_constants_8.bin");

    let mut cnst = [BlsScalar::zero(); 64];

    let mut j = 0;
    let mut i = 0;
    while i < 64 * 32 {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j] = BlsScalar::from_raw([a, b, c, d]);
        j += 1;

        i += 32;
    }
    cnst
};