use super::NUM_COLUMNS;

/// MDS MATRIX
pub const MDS_MATRIX: [[usize; NUM_COLUMNS]; NUM_COLUMNS] = [
    [1, 1, 3, 4, 5, 6],
    [6, 1, 1, 3, 4, 5],
    [5, 6, 1, 1, 3, 4],
    [4, 5, 6, 1, 1, 3],
    [3, 4, 5, 6, 1, 1],
    [1, 3, 4, 5, 6, 1],
];
