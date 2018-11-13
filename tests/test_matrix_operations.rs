extern crate ds_linalg;

#[cfg(test)]
mod tests {
    use ds_linalg::conversions::ToMatrix;

    #[test]
    fn does_matrix_comparison_give_expected_results() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix();
        let second_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix();
        let third_matrix = vec![vec![1, 2], vec![4, 3]].to_matrix();

        assert_eq!(first_matrix, second_matrix, "Matrix comparison operator says identical matrices are different.");
        assert_ne!(first_matrix, third_matrix, "Matrix comparison operator says differing matrices are equal.")
    }

    #[test]
    fn does_matrix_comparison_handle_different_sizes() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]];
        let second_matrix = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let third_matrix = vec![vec![1], vec![3]];

        assert_ne!(first_matrix, second_matrix, "Matrix comparison operator says matrices of different sizes are equal.");
        assert_ne!(first_matrix, third_matrix, "Matrix comparison operator says matrices of different sizes are equal.")
    }

    #[test]
    fn does_matrix_addition_give_expected_values() {
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
    fn does_matrix_addition_handle_different_dimensions() {
        let first_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix().unwrap();
        let second_matrix = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]].to_matrix().unwrap();

        let sum_matrix = &first_matrix + &second_matrix;

        assert!(sum_matrix.is_err(), "Matrix addition succeeded when it should have failed due to different dimensions.");
    }

    #[test]
    fn does_matrix_trace_give_expected_result() {
        let matrix_1x1 = vec![vec![1]].to_matrix().unwrap();
        let trace_1x1_result = matrix_1x1.calculate_trace();
        assert!(trace_1x1_result.is_ok(), "Error encountered calculating 1x1 trace: '{}'.", trace_1x1_result.err().unwrap());
        assert_eq!(trace_1x1_result.unwrap(), 1, "Incorrect value calculated for 1x1 matrix trace.");

        let matrix_2x2 = vec![vec![1, 2], vec![3, 4]].to_matrix().unwrap();
        let trace_2x2_result = matrix_2x2.calculate_trace();
        assert!(trace_2x2_result.is_ok(), "Error encountered calculating 2x2 trace: '{}'.", trace_2x2_result.err().unwrap());
        assert_eq!(trace_2x2_result.unwrap(), 5, "Incorrect value calculated for 2x2 matrix trace.");

        let matrix_3x3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].to_matrix().unwrap();
        let trace_3x3_result = matrix_3x3.calculate_trace();
        assert!(trace_3x3_result.is_ok(), "Error encountered calculating 3x3 trace: '{}'.", trace_3x3_result.err().unwrap());
        assert_eq!(trace_3x3_result.unwrap(), 15, "Incorrect value calculated for 3x3 matrix trace.");

        let matrix_4x4 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]].to_matrix().unwrap();
        let trace_4x4_result = matrix_4x4.calculate_trace();
        assert!(trace_4x4_result.is_ok(), "Error encountered calculating 4x4 trace: '{}'.", trace_4x4_result.err().unwrap());
        assert_eq!(trace_4x4_result.unwrap(), 34, "Incorrect value calculated for 4x4 matrix trace.");
    }

    #[test]
    fn does_matrix_trace_handle_non_square_matrices() {
        let non_square_matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]].to_matrix().unwrap();
        let trace_result = non_square_matrix.calculate_trace();

        assert!(trace_result.is_err(), "Trace on non-square matrix produced result when it should have failed.");
    }

    #[test]
    fn does_matrix_coord_iter_give_expected_results() {
        let square_matrix = vec![vec![1, 2], vec![3, 4]].to_matrix().unwrap();
        let square_coord_iter = square_matrix.get_coord_iter();

        let mut square_index = 0;
        for (current_row, current_col) in square_coord_iter {
            match square_index {
                0 => {
                    assert_eq!(current_row, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                1 => {
                    assert_eq!(current_row, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                2 => {
                    assert_eq!(current_row, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                3 => {
                    assert_eq!(current_row, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                _ => {
                    assert!(false, "Matrix co-ordinate iterator continued past end of matrix");
                },
            }

            square_index += 1;
        }

        let non_square_matrix = vec![vec![1, 2, 3], vec![4, 5, 6]].to_matrix().unwrap();
        let non_square_coord_iter = non_square_matrix.get_coord_iter();

        let mut non_square_index = 0;
        for (current_row, current_col) in non_square_coord_iter {
            match non_square_index {
                0 => {
                    assert_eq!(current_row, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                1 => {
                    assert_eq!(current_row, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                2 => {
                    assert_eq!(current_row, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                3 => {
                    assert_eq!(current_row, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                4 => {
                    assert_eq!(current_row, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                5 => {
                    assert_eq!(current_row, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                6 => {
                    assert_eq!(current_row, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 0, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                7 => {
                    assert_eq!(current_row, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 1, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                8 => {
                    assert_eq!(current_row, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                    assert_eq!(current_col, 2, "Incorrect co-ordinate from matrix co-ordinate iterator.");
                },
                _ => {
                    assert!(false, "Matrix co-ordinate iterator continued past end of matrix");
                },
            }

            non_square_index += 1;
        }
    }
}