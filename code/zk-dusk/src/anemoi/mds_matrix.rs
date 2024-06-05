use super::NUM_COLUMNS;

// Width = 4
/* pub const MDS_MATRIX: [[u64; NUM_COLUMNS]; NUM_COLUMNS] = [
    [1, 7],
    [7, 50],
]; */

// Width = 6
//pub const MDS_MATRIX: [[u64; NUM_COLUMNS]; NUM_COLUMNS] = [[8, 1, 8], [1, 1,
// 7], [7, 1, 1]];

// Width = 8
pub const MDS_MATRIX: [[u64; NUM_COLUMNS]; NUM_COLUMNS] =
    [[1, 8, 7, 7], [49, 56, 8, 15], [49, 49, 1, 8], [8, 15, 7, 8]];
