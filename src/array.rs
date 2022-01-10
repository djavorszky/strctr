use std::ops::{Index, IndexMut};

/// Simple implementation of an array. Uses a fixed-size slice for storage.

/// List of errors that could occur when dealing with Arrays
pub enum ArrayError {
    /// Signals that an overflow has happened; Most probably more elements were pushed
    /// onto the array than its underlying size.
    Overflow,
}

/// An array implementation. Uses compile-time constant size [`std::array`] as the underlying data structure.
pub struct Array<T, const N: usize> {
    elements: [T; N],
    cursor: usize,
}

impl<T, const N: usize> Default for Array<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Array<T, N>
where
    T: Default + Copy,
{
    /// Constructs a new Array of types T and size N
    pub fn new() -> Self {
        Self {
            elements: [T::default(); N],
            cursor: 0,
        }
    }

    /// Returns whether the Array is empty. It is considered to be empty if no values have been
    /// pushed into it.
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<_, 15> = Array::new();
    /// // Initially it is empty
    /// assert!(a.is_empty());
    /// a.push(12);
    /// // The array is no longer empty
    /// assert_eq!(a.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cursor == 0
    }

    /// Returns the number of elements inside the array. This is in contrast to [size()](`Self::size()`), which
    /// returns the number of elements that the array can hold.
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 15> = Array::new();
    /// // Initial length is 0
    /// assert_eq!(a.len(), 0);
    /// a.push(12);
    /// // Length is now 1
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.cursor
    }

    /// Returns the size of the underlying data structure. This is in contrast to [len()](`Self::len()`), which
    /// returns the number of elements within the array.    
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 15> = Array::new();
    /// // Size is 15, Length is 0
    /// assert_eq!(a.size(), 15);
    /// assert_eq!(a.len(), 0);
    ///
    /// a.push(10);
    /// // Size is still 15, Length is now 1
    /// assert_eq!(a.size(), 15);
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn size(&self) -> usize {
        N
    }

    /// Adds a new element to the array. The item is appened to the end. Adding more elements than can fit into the
    /// array will result in an error being returned.
    ///
    /// For a more convenient (but less safe) method, see [push()](`Self::push()`)
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 1> = Array::new();
    /// let res = a.try_push(1);
    /// assert!(res.is_ok());
    /// let res2 = a.try_push(2);
    /// assert!(res2.is_err());
    /// ```
    pub fn try_push(&mut self, elem: T) -> Result<(), ArrayError> {
        if self.cursor >= N {
            return Err(ArrayError::Overflow);
        }

        self.elements[self.cursor] = elem;
        self.cursor += 1;

        Ok(())
    }

    /// Adds an element to the end of the array.
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 1> = Array::new();
    /// a.push(1);
    /// assert_eq!(a.len(), 1);
    /// ```
    /// Panics if the resulting [len()](`Self::len()`) > [size()](`Self::size()`).
    /// For a non-panicing version, see [try_push()](`Self::try_push()`)
    /// ```should_panic
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 1> = Array::new();
    /// a.push(1);
    /// a.push(2);
    /// ```
    pub fn push(&mut self, elem: T) {
        let res = self.try_push(elem);
        if res.is_err() {
            panic!(
                "Overflow: Wanted to add element {}, but size is {}",
                self.cursor, N
            );
        }
    }
}

impl<T, const N: usize> Index<usize> for Array<T, N>
where
    T: Default + Copy,
{
    type Output = T;

    /// Returns the element at the specified index.
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 5> = Array::new();
    /// a.push(1);
    /// a.push(2);
    /// assert_eq!(a[0], 1);
    /// assert_eq!(a[1], 2);
    /// ```
    ///
    /// Panics if index > [len()](`Self::len()`).
    /// ```should_panic
    /// # use strctr::array::Array;
    /// let a: Array<usize, 5> = Array::new();
    /// let x = a[3];
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        if index > self.len() {
            panic!(
                "OutOfBounds: Wanted index {}, but length is {}",
                index,
                self.len()
            )
        }
        &self.elements[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Array<T, N>
where
    T: Default + Copy,
{
    /// Allows updating the values within the array.
    /// ```
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 5> = Array::new();
    /// a.push(1); // a[0] is now 1
    /// a[0] = 20;
    /// assert_eq!(a[0], 20);
    /// ```
    ///
    /// Panics if index > [len()](`Self::len()`).
    /// ```should_panic
    /// # use strctr::array::Array;
    /// let mut a: Array<usize, 5> = Array::new();
    /// a[3] = 25;
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.len() {
            panic!(
                "OutOfBounds: Wanted index {}, but length is {}",
                index,
                self.len()
            )
        }
        &mut self.elements[index]
    }
}
