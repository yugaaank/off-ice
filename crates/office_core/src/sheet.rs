//! Spreadsheet model and cell operations.

use crate::{Cell, DocumentId};
use serde::{Deserialize, Serialize};

/// A spreadsheet document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: DocumentId,
    pub title: String,
    pub cells: Vec<Vec<Cell>>,
    pub column_widths: Vec<u16>,
    pub row_heights: Vec<u16>,
    max_row: usize,
    max_col: usize,
}

impl Sheet {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: DocumentId::new_v4(),
            title: title.into(),
            cells: vec![],
            column_widths: vec![],
            row_heights: vec![],
            max_row: 0,
            max_col: 0,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row)?.get_mut(col)
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        while self.cells.len() <= row {
            self.cells.push(vec![]);
        }
        while self.cells[row].len() <= col {
            self.cells[row].push(Cell::empty());
        }
        self.cells[row][col] = cell;
        self.max_row = self.max_row.max(row + 1);
        self.max_col = self.max_col.max(col + 1);
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.max_row, self.max_col)
    }

    pub fn ensure_dimensions(&mut self, rows: usize, cols: usize) {
        while self.cells.len() < rows {
            self.cells.push(vec![]);
        }
        for row in &mut self.cells {
            while row.len() < cols {
                row.push(Cell::empty());
            }
        }
        self.max_row = self.max_row.max(rows);
        self.max_col = self.max_col.max(cols);
    }
}
