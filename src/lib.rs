pub mod ds_lin_alg {
    use std::iter;
    use std::ops::Add;
    use std::fmt;
    use std::default::Default;

    pub struct DSMatrix<T> {
        num_rows: u32,
        num_columns: u32,
        values: Vec<T>,
    }

    pub struct DSMatrixCoordIter {
        num_rows: u32,
        num_columns: u32,
        current_row: u32,
        current_column: u32,
    }

    pub type MatrixOpResult<T> = Result<T, &'static str>; // TODO: Improve error type.

    impl<T> DSMatrix<T> where T: Default + Clone {
        pub fn new(in_num_rows: u32, in_num_columns: u32) -> DSMatrix<T> {
            let num_elements = (in_num_rows * in_num_columns) as usize;
            let value_default: T = Default::default();
            let new_values = iter::repeat(value_default).take(num_elements).collect();

            DSMatrix::<T> {
                num_rows: in_num_rows,
                num_columns: in_num_columns,
                values: new_values,
            }
        }

        pub fn get_coord_iter(&self) -> DSMatrixCoordIter {
            DSMatrixCoordIter {
                num_rows: self.num_rows,
                num_columns: self.num_columns,
                current_row: 0,
                current_column: 0,
            }
        }

        pub fn get_num_rows(&self) -> u32 {
            self.num_rows
        }

        pub fn get_num_columns(&self) -> u32 {
            self.num_columns
        }

        pub fn get_value<'a>(&'a self, row_number: u32, column_number: u32) -> &'a T {
            let value_index = self.get_index(row_number, column_number);
            &self.values[value_index]
        }

        pub fn set_value(&mut self, row_number: u32, column_number: u32, value: T) {
            let value_index = self.get_index(row_number, column_number);
            self.values[value_index] = value;
        }

        fn get_index(&self, row_number: u32, column_number: u32) -> usize {
            (row_number * self.num_columns + column_number) as usize
        }
    }

    pub trait ToMatrix<T> {
        fn to_matrix(self) -> MatrixOpResult<T>;
    }

    pub trait ToMatrixWithStride<T> {
        fn to_matrix_with_stride(self, stride: u32) -> MatrixOpResult<T>;
    }

    /// Implement conversion between 2d vector and matrix. Note that it is interpreted
    /// as a vec of rows as opposed to a vec of columns.
    impl<T> ToMatrix<DSMatrix<T>> for Vec<Vec<T>>
    where
        T: Default + Clone
    {
        fn to_matrix(self) -> MatrixOpResult<DSMatrix<T>> {
            if self.len() == 0 || self[0].len() == 0 {
                // Original vec is empty, so resulting matrix is empty.
                return Err("Cannot convert empty vec to matrix.");
            }

            let num_rows = self.len();
            let num_columns = self[0].len();
            let mut out_matrix = DSMatrix::<T>::new(num_rows as u32, num_columns as u32);

            let mut row_index = 0;
            let mut col_index = 0;
            for row in self {
                if row.len() != num_columns {
                    return Err("Inner vectors are of different sizes.");
                }

                for value in row {
                    out_matrix.set_value(row_index, col_index, value);
                    col_index += 1;
                }

                col_index = 0;
                row_index += 1;
            }

            Ok(out_matrix)
        }
    }

    /// Implement conversion between 2d vector and matrix. Note that it is interpreted
    /// as a vec of rows as opposed to a vec of columns.
    impl<T> ToMatrixWithStride<DSMatrix<T>> for Vec<T>
    where
        T: Default + Clone
    {
        fn to_matrix_with_stride(self, stride: u32) -> MatrixOpResult<DSMatrix<T>> {
            if self.len() == 0 {
                // Original vec is empty, so resulting matrix is empty.
                return Err("Cannot convert empty vec to matrix.");
            }

            let num_elements = self.len() as u32;
            if num_elements % stride != 0 {
                return Err("Vector size is not whole multiple of stride.");
            }

            let num_rows = num_elements / stride;
            let mut out_matrix = DSMatrix::<T>::new(num_rows as u32, stride as u32);

            let mut i = 0;
            for value in self {
                let row_index = i / stride;
                let column_index = i % stride;

                out_matrix.set_value(row_index, column_index, value);

                i += 1;
            }

            Ok(out_matrix)
        }
    }

    impl Iterator for DSMatrixCoordIter {
        type Item = (u32, u32);

        fn next(&mut self) -> Option<(u32, u32)> {
            match (self.current_row, self.current_column) {
                (row, col) if self.current_row < self.num_rows => {
                    self.current_column += 1;

                    if self.current_column >= self.num_columns {
                        self.current_column = 0;
                        self.current_row += 1;
                    }

                    Some((row, col))
                },
                _ if self.current_column >= self.num_columns => unreachable!(),
                _ => None,
            }
        }
    }

    impl<T> PartialEq for DSMatrix<T> where T: Default + Clone + PartialEq {
        fn eq(&self, other: &DSMatrix<T>) -> bool {
            if self.get_num_rows() != other.get_num_rows() || self.get_num_columns() != other.get_num_columns() {
                return false;
            }

            for (i, j) in self.get_coord_iter() {
                if self.get_value(i, j) != other.get_value(i, j) {
                    return false
                }
            }

            true
        }
    }

    // TODO: Generic iterator closure method.

    impl<'a, 'b, T> Add<&'b DSMatrix<T>> for &'a DSMatrix<T>
    where
        T: Default + Clone,
        for<'c> &'c T: Add<Output=T> // This is called a "higher ranked trait bound" (HRTB)
    {
        type Output = MatrixOpResult<DSMatrix<T>>;

        fn add(self, other: &'b DSMatrix<T>) -> MatrixOpResult<DSMatrix<T>> {
            if self.num_rows != other.num_rows || self.num_columns != other.num_columns {
                return Err("Incorrect dimensions for addition.");
            }

            let mut output = DSMatrix::<T>::new(self.num_rows, self.num_columns);

            // TODO: Does this autovectorise? If not, how can we make it?
            for (i, j) in self.get_coord_iter() {
                let left_ref = self.get_value(i, j);
                let right_ref = other.get_value(i, j);
                let add_value = left_ref + right_ref; // This is where the HRTB is required.
                output.set_value(i, j, add_value);
            }

            Ok(output)
        }
    }

    impl<T> fmt::Debug for DSMatrix<T>
    where
        T: Default + Clone + fmt::Debug + fmt::Display
    {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Matrix: \n[ ")?;

            let mut current_row = 0;
            let num_columns = self.get_num_columns();
            for (i, j) in self.get_coord_iter() {
                if i != current_row {
                    write!(formatter, "[ ")?;
                    current_row = i;
                }

                write!(formatter, "{:4}", self.get_value(i, j))?;

                if j < num_columns - 1 {
                    write!(formatter, ", ")?;
                } else {
                    write!(formatter, " ]\n")?;
                }
            }

            write!(formatter, "")
        }
    }

    // TODO: Macro for creating new matrices
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_lin_alg::ToMatrix;
    use ds_lin_alg::ToMatrixWithStride;

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
    fn does_u32_vec_to_matrix_conversion_give_expected_results() {
        let matrix_values = vec![vec![1, 2], vec![3, 4]];
        let matrix_result = matrix_values.to_matrix();

        assert!(matrix_result.is_ok(), "Failed to convert vector to matrix: '{}'.", matrix_result.err().unwrap());

        let mut expected_output = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        expected_output.set_value(0, 0, 1);
        expected_output.set_value(0, 1, 2);
        expected_output.set_value(1, 0, 3);
        expected_output.set_value(1, 1, 4);

        assert_eq!(matrix_result.unwrap(), expected_output, "Vector to matrix conversion did not produce expected results.");
    }

    #[test]
    fn does_u32_vec_to_matrix_conversion_handle_different_size_vectors() {
        let matrix_values = vec![vec![1, 2], vec![3, 4, 5]];
        let matrix_result = matrix_values.to_matrix();

        assert!(matrix_result.is_err(), "Vector to matrix conversion produced ok when input vector was invalid.");
    }

    #[test]
    fn does_u32_vec_with_stride_to_matrix_conversion_give_expected_results() {
        let matrix_values = vec![1, 2, 3, 4];
        let matrix_result = matrix_values.to_matrix_with_stride(2);

        assert!(matrix_result.is_ok(), "Unable to convert vector with stride to matrix: '{}'.", matrix_result.err().unwrap());

        let mut expected_matrix = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        expected_matrix.set_value(0, 0, 1);
        expected_matrix.set_value(0, 1, 2);
        expected_matrix.set_value(1, 0, 3);
        expected_matrix.set_value(1, 1, 4);

        assert_eq!(matrix_result.unwrap(), expected_matrix, "Vector with stride to matrix conversion did not produce expected results.");
    }

    #[test]
    fn does_u32_vec_with_stride_to_matrix_conversion_handle_incorrect_stride() {
        let matrix_values = vec![1, 2, 3, 4, 5];
        let matrix_result = matrix_values.to_matrix_with_stride(2);

        assert!(matrix_result.is_err(), "Vector with stride to matrix conversion produced ok when vector length wasn't multiple of stride.");
    }

    #[test]
    fn does_u32_matrix_addition_give_expected_values() {
        let mut first_matrix = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        first_matrix.set_value(0, 0, 1);
        first_matrix.set_value(0, 1, 2);
        first_matrix.set_value(1, 0, 3);
        first_matrix.set_value(1, 1, 4);

        let mut second_matrix = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        second_matrix.set_value(0, 0, 5);
        second_matrix.set_value(0, 1, 6);
        second_matrix.set_value(1, 0, 7);
        second_matrix.set_value(1, 1, 8);

        let sum_matrix = &first_matrix + &second_matrix;

        let mut expected_matrix = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        expected_matrix.set_value(0, 0, 6);
        expected_matrix.set_value(0, 1, 8);
        expected_matrix.set_value(1, 0, 10);
        expected_matrix.set_value(1, 1, 12);

        println!("Output matrix: {:?}", sum_matrix);
        println!("Expected matrix: {:?}", expected_matrix);

        assert!(sum_matrix.is_ok(), "Matrix addition failed.");
        assert_eq!(sum_matrix.unwrap(), expected_matrix, "Matrix addition did not produce expected result.");
    }

    #[test]
    fn does_u32_matrix_addition_handle_different_dimensions() {
        let mut first_matrix = ds_lin_alg::DSMatrix::<u32>::new(2, 2);
        first_matrix.set_value(0, 0, 1);
        first_matrix.set_value(0, 1, 2);
        first_matrix.set_value(1, 0, 3);
        first_matrix.set_value(1, 1, 4);

        let mut second_matrix = ds_lin_alg::DSMatrix::<u32>::new(3, 3);
        second_matrix.set_value(0, 0, 5);
        second_matrix.set_value(0, 1, 6);
        second_matrix.set_value(0, 2, 6);
        second_matrix.set_value(1, 0, 7);
        second_matrix.set_value(1, 1, 8);
        second_matrix.set_value(1, 2, 9);
        second_matrix.set_value(2, 0, 10);
        second_matrix.set_value(2, 1, 11);
        second_matrix.set_value(2, 2, 12);

        let sum_matrix = &first_matrix + &second_matrix;

        assert!(sum_matrix.is_err(), "Matrix addition succeeded when it should have failed due to different dimensions.");
    }
}