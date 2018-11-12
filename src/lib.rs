pub mod conversions;

use std::iter;
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

impl<T> DSMatrix<T>
where
    T: Clone + Default + std::fmt::Display,
    for<'a> T: std::ops::AddAssign<&'a T>,
{
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

    pub fn get_value<'b>(&'b self, row_number: u32, column_number: u32) -> &'b T {
        let value_index = self.get_index(row_number, column_number);
        &self.values[value_index]
    }

    pub fn set_value(&mut self, row_number: u32, column_number: u32, value: T) {
        let value_index = self.get_index(row_number, column_number);
        self.values[value_index] = value;
    }

    pub fn calculate_trace(&self) -> MatrixOpResult<T> {
        if self.num_rows != self.num_columns {
            return Err("Cannot calculate trace of non-square matrix.");
        }

        // TODO: Implement this using a diagonal iterator.
        let mut trace: T = Default::default();
        for i in 0..self.num_rows {
            trace += self.get_value(i, i);
        }

        Ok(trace)
    }

    fn get_index(&self, row_number: u32, column_number: u32) -> usize {
        (row_number * self.num_columns + column_number) as usize
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

// TODO: Generic iterator closure method.

impl<T> fmt::Debug for DSMatrix<T>
where
    T: Clone + Default + std::fmt::Display,
    for<'a> T: std::ops::AddAssign<&'a T>,
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

impl<T> PartialEq for DSMatrix<T>
where
    T: Clone + Default + PartialEq + std::fmt::Display,
    for<'a> T: std::ops::AddAssign<&'a T>,
{
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

impl<'a, 'b, T> std::ops::Add<&'b DSMatrix<T>> for &'a DSMatrix<T>
where
    T: Clone + Default + std::fmt::Display,
    for<'c> &'c T: std::ops::Add<Output=T>, // This is called a "higher ranked trait bound" (HRTB)
    for<'d> T: std::ops::AddAssign<&'d T>,
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

// TODO: Macro for creating new matrices