pub mod grid;
pub mod infinite_grid;

use std::cmp::PartialEq;
use std::ops::Index;
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug)]
pub enum HorizontalVerticalDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Debug)]
pub enum HorizontalVerticalDiagonalDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub trait Neighbors {
    type Index: GridIndex;

    /// Gets the horizontal and vertical neighbors
    fn hv_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> Vec<(Self::Index, Self::Index, HorizontalVerticalDirection)>;

    /// Gets the horizontal, vertical, and diagonal neighbors
    fn hvd_neighbors(
        &self,
        row_index: Self::Index,
        column_index: Self::Index,
    ) -> Vec<(
        Self::Index,
        Self::Index,
        HorizontalVerticalDiagonalDirection,
    )>;
}

pub trait GridIter {
    type GridRow;

    fn get_grid(&self) -> &Vec<Self::GridRow>;
    fn get_row_length(&self) -> usize;
    fn get_column_length(&self) -> usize;

    fn row_iter(&self) -> Iter<Self::GridRow> {
        self.get_grid().iter()
    }

    fn column_iter(&self) -> ColumnIter<Self>
    where
        Self: Sized,
    {
        ColumnIter::new(self)
    }

    fn row_column_index_value_iter(&self) -> RowColumnIndexValueIter<Self>
    where
        Self: Sized,
    {
        RowColumnIndexValueIter::new(self)
    }

    fn find<T>(&self, value: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
        Self: Sized + Index<usize, Output = Self::GridRow>,
        Self::GridRow: Index<usize>,
        <Self::GridRow as Index<usize>>::Output: PartialEq<T>,
    {
        for ((row_index, column_index), v) in self.row_column_index_value_iter() {
            if v == value {
                return Some((row_index, column_index));
            }
        }

        None
    }
}

pub trait GridIndex {}

impl GridIndex for usize {}
impl GridIndex for isize {}

#[must_use]
pub struct ColumnIter<'g, G> {
    grid: &'g G,
    column_index: usize,
    column_length: usize,
}

impl<'g, G> Iterator for ColumnIter<'g, G>
where
    G: GridIter,
    G::GridRow: Index<usize>,
{
    type Item = Vec<&'g <G::GridRow as Index<usize>>::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index < self.column_length {
            let column = self
                .grid
                .get_grid()
                .iter()
                .map(|row| &row[self.column_index])
                .collect();

            self.column_index += 1;

            Some(column)
        } else {
            None
        }
    }
}

impl<'g, G: GridIter> ColumnIter<'g, G> {
    fn new(grid: &'g G) -> ColumnIter<'g, G> {
        Self {
            grid,
            column_index: 0,
            column_length: grid.get_column_length(),
        }
    }
}

#[must_use]
pub struct RowColumnIndexValueIter<'g, G> {
    grid: &'g G,
    row_index: usize,
    row_length: usize,
    column_index: usize,
    column_length: usize,
}

impl<'g, G: GridIter> RowColumnIndexValueIter<'g, G> {
    fn new(grid: &'g G) -> RowColumnIndexValueIter<'g, G> {
        Self {
            grid,
            row_index: 0,
            row_length: grid.get_row_length(),
            column_index: 0,
            column_length: grid.get_column_length(),
        }
    }
}

impl<'g, G> Iterator for RowColumnIndexValueIter<'g, G>
where
    G: GridIter + Index<usize, Output = G::GridRow>,
    G::GridRow: Index<usize>,
{
    type Item = ((usize, usize), &'g <G::GridRow as Index<usize>>::Output);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.row_length {
            let old = (
                (self.row_index, self.column_index),
                &self.grid[self.row_index][self.column_index],
            );

            // and go next
            if self.column_index + 1 == self.column_length {
                self.column_index = 0;

                self.row_index += 1;
            } else {
                self.column_index += 1;
            }

            Some(old)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let first: Vec<char> = vec!['a', 'b', 'c'];

        let iter = first.iter();

        assert_eq!(vec![&'a', &'b', &'c'], iter.collect::<Vec<_>>());
    }
}
