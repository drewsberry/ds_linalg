extern crate ds_linalg;

#[cfg(test)]
mod tests {
    use ds_linalg::conversions::{ToMatrix, ToMatrixWithStride};

    #[test]
    fn does_vec_to_matrix_conversion_give_expected_results() {
        let matrix_values = vec![vec![1, 2], vec![3, 4]];
        let matrix_result = matrix_values.to_matrix();

        assert!(matrix_result.is_ok(), "Failed to convert vector to matrix: '{}'.", matrix_result.err().unwrap());

        let mut expected_output = ds_linalg::DSMatrix::<u32>::new(2, 2);
        expected_output.set_value(0, 0, 1);
        expected_output.set_value(0, 1, 2);
        expected_output.set_value(1, 0, 3);
        expected_output.set_value(1, 1, 4);

        assert_eq!(matrix_result.unwrap(), expected_output, "Vector to matrix conversion did not produce expected results.");
    }

    #[test]
    fn does_vec_to_matrix_conversion_handle_different_size_vectors() {
        let matrix_values = vec![vec![1, 2], vec![3, 4, 5]];
        let matrix_result = matrix_values.to_matrix();

        assert!(matrix_result.is_err(), "Vector to matrix conversion produced ok when input vector was invalid.");
    }

    #[test]
    fn does_vec_with_stride_to_matrix_conversion_give_expected_results() {
        let matrix_values = vec![1, 2, 3, 4];
        let matrix_result = matrix_values.to_matrix_with_stride(2);

        assert!(matrix_result.is_ok(), "Unable to convert vector with stride to matrix: '{}'.", matrix_result.err().unwrap());

        let mut expected_matrix = ds_linalg::DSMatrix::<u32>::new(2, 2);
        expected_matrix.set_value(0, 0, 1);
        expected_matrix.set_value(0, 1, 2);
        expected_matrix.set_value(1, 0, 3);
        expected_matrix.set_value(1, 1, 4);

        assert_eq!(matrix_result.unwrap(), expected_matrix, "Vector with stride to matrix conversion did not produce expected results.");
    }

    #[test]
    fn does_vec_with_stride_to_matrix_conversion_handle_incorrect_stride() {
        let matrix_values = vec![1, 2, 3, 4, 5];
        let matrix_result = matrix_values.to_matrix_with_stride(2);

        assert!(matrix_result.is_err(), "Vector with stride to matrix conversion produced ok when vector length wasn't multiple of stride.");
    }
}