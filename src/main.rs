use ndarray::{ShapeBuilder, IntoDimension, Dimension, StrideShape, Order};
fn main() {
    let shape = (2, 4, 2).strides((1, 4, 2));
    println!("{:?}", can_be_viewed_reshape(shape.clone(), (2, 4, 2), Order::RowMajor).1.unwrap());
    println!("{:?}", can_be_viewed_reshape(shape.clone(), (1, 2, 4, 2), Order::RowMajor).1.unwrap());
    println!("{:?}", can_be_viewed_reshape(shape.clone(), (2, 2, 2, 2), Order::RowMajor).1.unwrap());
}
fn can_be_viewed_reshape<D: Dimension, Sh: Into<StrideShape<D>>, I: IntoDimension>(from: Sh, to: I, order: Order) -> (bool, Option<I::Dim>) {
    let shape = from.into();
    let mut dim1 = shape.dim.clone();
    let mut strides_from = shape.strides.strides_for_dim(&dim1);
    let mut dim2 = to.into_dimension();
    let mut strides_to = I::Dim::zeros(dim2.ndim());
    if dim1.size() != dim2.size() {
        return (false, None)
    }
    if dim1.size() == 0 {
        return (true, None)
    }
    if !matches!(order, Order::RowMajor) {
        return (false, None)
    }
    let s1 = dim1.slice_mut();
    let s2 = dim2.slice_mut();
    let stride_s = strides_from.slice_mut();
    let mut i = s1.len() - 1;
    let mut j = s2.len() - 1;
    while i as isize >= 0 && j as isize >= 0 {
        if s1[i] == 1 { // incoming is 1
            i = (i as isize - 1) as usize;
        } else if s2[j] == 1 { // outgoing is 1
            strides_to[j] = 0;
            j = (j as isize - 1) as usize;
        } else if s1[i] == s2[j] { // incoming and outgoing are equal
            strides_to[j] = stride_s[i];
            i = (i as isize - 1) as usize;
            j = (j as isize - 1) as usize;
        } else if s1[i] % s2[j] == 0 { // incoming is divisible by outgoing
            strides_to[j] = stride_s[i];
            s1[i] /= s2[j];
            stride_s[i] *= s2[j];
            j = (j as isize - 1) as usize;
        } else { // incoming cannot be divisible by outgoing
            let offset = stride_s[i - 1] - stride_s[i] * (s1[i] - 1);
            if offset != stride_s[i] {
                return (false, None)
            }
            s1[i - 1] *= s1[i];
            stride_s[i - 1] = stride_s[i];
            i = (i as isize - 1) as usize;
        }
    }
    (true, Some(strides_to))
}