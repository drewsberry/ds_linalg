use super::DSMatrix;
use super::MatrixOpResult;

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
    T: Clone + Default + std::fmt::Display,
    for<'a> T: std::ops::AddAssign<&'a T>,
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

/// Implement conversion between 1d vector with specific stride and matrix. The stride is the
/// number of columns in the output matrix.
impl<T> ToMatrixWithStride<DSMatrix<T>> for Vec<T>
where
    T: Clone + Default + std::fmt::Display,
    for<'a> T: std::ops::AddAssign<&'a T>,
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