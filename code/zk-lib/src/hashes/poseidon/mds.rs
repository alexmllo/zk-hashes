// The MDS matrix we use is C + D, where C is the circulant matrix whose first row is given by
// `MDS_MATRIX_CIRC`, and D is the diagonal matrix whose diagonal is given by `MDS_MATRIX_DIAG`.
#[allow(dead_code)]
const MDS_MATRIX_CIRC: [u64; 12] = [17, 15, 41, 16, 2, 28, 13, 13, 39, 18, 34, 20];
#[allow(dead_code)]
const MDS_MATRIX_DIAG: [u64; 12] = [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// The MDS matrix calculated with the plonky2 poseidon requirements
pub const MDS_MATRIX: [[u64; 12]; 12] = [
    [25, 15, 41, 16, 2, 28, 13, 13, 39, 18, 34, 20],
    [20, 17, 15, 41, 16, 2, 28, 13, 13, 39, 18, 34],
    [34, 20, 17, 15, 41, 16, 2, 28, 13, 13, 39, 18],
    [18, 34, 20, 17, 15, 41, 16, 2, 28, 13, 13, 39],
    [39, 18, 34, 20, 17, 15, 41, 16, 2, 28, 13, 13],
    [13, 39, 18, 34, 20, 17, 15, 41, 16, 2, 28, 13],
    [13, 13, 39, 18, 34, 20, 17, 15, 41, 16, 2, 28],
    [28, 13, 13, 39, 18, 34, 20, 17, 15, 41, 16, 2],
    [2, 28, 13, 13, 39, 18, 34, 20, 17, 15, 41, 16],
    [16, 2, 28, 13, 13, 39, 18, 34, 20, 17, 15, 41],
    [41, 16, 2, 28, 13, 13, 39, 18, 34, 20, 17, 15],
    [15, 41, 16, 2, 28, 13, 13, 39, 18, 34, 20, 17],
];