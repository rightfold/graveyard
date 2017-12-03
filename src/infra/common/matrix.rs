use std::ops::{Index, IndexMut};

/// A 2D array of dynamic but immutable size.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> where T: Clone {
    /// Create a new matrix with all elements the same.
    pub fn new(element: T, width: usize, height: usize) -> Self {
        Matrix{
            elements: vec![element; width * height],
            width,
        }
    }
}

impl<T> Matrix<T> {
    /// Number of columns in the matrix.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Number of rows in the matrix.
    pub fn height(&self) -> usize {
        self.elements.len() / self.width
    }

    /// Get the element at the specified index.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.element_index(x, y);
        self.elements.get(index)
    }

    /// Get the element at the specified index.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.element_index(x, y);
        self.elements.get_mut(index)
    }

    fn element_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.element_index(index.0, index.1);
        &self.elements[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.element_index(index.0, index.1);
        &mut self.elements[index]
    }
}
