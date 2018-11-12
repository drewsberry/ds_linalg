extern crate ds_linalg;

#[cfg(test)]
mod tests {
    use ds_linalg::conversions::ToMatrix;

    #[test]
    fn does_u32_matrix_comparison_give_expected_results() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix();
        let second_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix();
        let third_matrix = vec![vec![1, 2], vec![4, 3]].to_matrix();

        assert_eq!(first_matrix, second_matrix, "Matrix comparison operator says identical matrices are different.");
        assert_ne!(first_matrix, third_matrix, "Matrix comparison operator says differing matrices are equal.")
    }

    #[test]
    fn does_u32_matrix_comparison_handle_different_sizes() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]];
        let second_matrix = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let third_matrix = vec![vec![1], vec![3]];

        assert_ne!(first_matrix, second_matrix, "Matrix comparison operator says matrices of different sizes are equal.");
        assert_ne!(first_matrix, third_matrix, "Matrix comparison operator says matrices of different sizes are equal.")
    }

    #[test]
    fn does_u32_matrix_addition_give_expected_values() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix().unwrap();
        let second_matrix = vec![vec![5, 6], vec![7, 8]].to_matrix().unwrap();

        let sum_result = &first_matrix + &second_matrix;

        assert!(sum_result.is_ok(), "Matrix addition produced error: '{}'.", sum_result.err().unwrap());

        let expected_matrix = vec![vec![6, 8], vec![10, 12]].to_matrix().unwrap();

        let sum_matrix = sum_result.unwrap();
        assert_eq!(sum_matrix, expected_matrix, "Matrix addition did not produce expected result.");

        println!("Output matrix: {:?}", sum_matrix);
        println!("Expected matrix: {:?}", expected_matrix);
    }

    #[test]
    fn does_u32_matrix_addition_handle_different_dimensions() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix().unwrap();
        let second_matrix = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]].to_matrix().unwrap();

        let sum_matrix = &first_matrix + &second_matrix;

        assert!(sum_matrix.is_err(), "Matrix addition succeeded when it should have failed due to different dimensions.");
    }
}