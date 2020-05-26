#![allow(dead_code)]

use std::fmt::Debug;

use crate::list::List;

#[derive(Clone)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    cells: List<List<T>>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.cells.iter().enumerate().flat_map(|(row, cells)| {
            cells
                .iter()
                .enumerate()
                .map(move |(col, cell)| (row, col, cell))
        })
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn set(&self, row: usize, col: usize, value: T) -> Option<Self> {
        let new_row = self.cells.get(row)?.set(col, value)?;

        Some(Self {
            rows: self.rows,
            cols: self.cols,
            cells: self.cells.set(row, new_row)?,
        })
    }
}

impl<T: Default> Grid<T> {
    pub fn with_dimensions(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: (0..rows).map(|_| List::with_length(cols)).collect(),
        }
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn push_row_front(&self) -> Self {
        Self {
            rows: self.rows + 1,
            cols: self.cols,
            cells: self.cells.push_front(List::with_length(self.cols)),
        }
    }

    pub fn push_row_back(&self) -> Self {
        Self {
            rows: self.rows + 1,
            cols: self.cols,
            cells: self.cells.push_back(List::with_length(self.cols)),
        }
    }

    pub fn push_col_front(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols + 1,
            cells: self
                .cells
                .iter()
                .map(|row| row.push_front(T::default()))
                .collect(),
        }
    }

    pub fn push_col_back(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols + 1,
            cells: self
                .cells
                .iter()
                .map(|row| row.push_back(T::default()))
                .collect(),
        }
    }
}

impl<T> std::default::Default for Grid<T> {
    fn default() -> Self {
        Self {
            rows: 0,
            cols: 0,
            cells: List::default(),
        }
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells.fmt(f)
    }
}

impl<T> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}
