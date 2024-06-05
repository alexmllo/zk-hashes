use dusk_bls12_381::BlsScalar;

// Width=4 -> length=88
// Width=5 -> length=90
pub const ROUND_CONSTANTS: [BlsScalar; 128] = {
    let bytes = include_bytes!("../../assets/rescue/round_constants_8.bin");

    let mut cnst = [BlsScalar::zero(); 128];

    let mut j = 0;
    let mut i = 32;
    while i < 128 * 32 {
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