/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place

    // Transpose then reverse: transpose only apply for square matrix
    // let mut rows = matrix.len();
    // let mut cols = matrix[0].len();

    // for i in 0..rows {
    //     for j in i + 1..cols {
    //         let (row_i, row_j) = matrix.split_at_mut(j);
    //         swap(&mut row_i[i][j], &mut row_j[0][i]);
    //     }
    // }
    // for row in matrix.iter_mut() {
    //     row.reverse();
    // }

    // Method 2, still work for square matrix only. Tips: ask for AI copilot to visualise it and it is amazing
    // let n = matrix.len();
    // for layer in 0..n / 2 {
    //     let first = layer;
    //     let last = n - 1 - layer;

    //     for i in first..last {
    //         let offset = i - first;

    //         let top = matrix[first][i];
    //         matrix[first][i] = matrix[last - offset][first];
    //         matrix[last - offset][first] = matrix[last][last - offset];
    //         matrix[last][last - offset] = matrix[i][last];
    //         matrix[i][last] = top;
    //     }
    // }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new matrix with swapped dimensions
    let mut rotated_matrix: Vec<Vec<i32>> = vec![vec![0; rows]; cols];

    // Copy elements from original matrix to rotated matrix
    for i in 0..rows {
        for j in 0..cols {
            rotated_matrix[j][rows - 1 - i] = matrix[i][j];
        }
    }

    // Replace original matrix with rotated matrix
    *matrix = rotated_matrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }
}
