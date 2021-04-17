
/// Array order
///
/// Order refers to indexing order, or how a linear sequence is translated
/// into a two-dimensional or multi-dimensional array.
///
/// - `RowMajor` means that the index along the row is the most rapidly changing
/// - `ColumnMajor` means that the index along the column is the most rapidly changing
///
/// Given a sequence like: 1, 2, 3, 4, 5, 6
///
/// If it is laid it out in a 2 x 3 matrix using row major ordering, it results in:
///
/// ```text
/// 1  2  3
/// 4  5  6
/// ```
///
/// If it is laid using column major ordering, it results in:
///
/// ```text
/// 1  3  5
/// 2  4  6
/// ```
///
/// It can be seen as filling in "rows first" or "columns first".
///
/// `Order` can be used both to refer to logical ordering as well as memory ordering or memory
/// layout. The orderings have common short names, also seen in other environments, where
/// row major is called "C" order (after the C programming language) and column major is called "F"
/// or "Fortran" order.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Order {
    /// Row major or "C" order
    RowMajor,
    /// Column major or "F" order
    ColumnMajor,
}

impl Order {
    /// "C" is an alias for row major ordering
    pub const C: Order = Order::RowMajor;

    /// "F" (for Fortran) is an alias for column major ordering
    pub const F: Order = Order::ColumnMajor;
}
