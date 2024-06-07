#[cfg(test)]
mod tests {
    use algebrust::AlgebrustMatrix;
    
    #[test]
    fn test_matrix_new() {
        let mat = AlgebrustMatrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        assert_eq!(mat.matrix_ref(), &vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    }

    #[test]
    fn test_matrix_new_rand() {
        let mat = AlgebrustMatrix::new_rand((2, 3), 0.0, 1.0);
        assert_eq!(mat.size(), (2, 3));
    }

    #[test]
    fn test_matrix_new_zeros() {
        let mat = AlgebrustMatrix::new_zeros((2, 3));
        assert_eq!(mat.matrix_ref(), &vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
    }

    #[test]
    fn test_matrix_new_identity() {
        let mat = AlgebrustMatrix::new_identity(3);
        assert_eq!(mat.matrix_ref(), &vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]]);
    }

    #[test]
    fn test_matrix_addition() {
        let mat1 = AlgebrustMatrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let mat2 = AlgebrustMatrix::new(&[&[5.0, 6.0], &[7.0, 8.0]]);
        let result = mat1.addition(&mat2);
        assert_eq!(result.matrix_ref(), &vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_matrix_subtraction() {
        let mat1 = AlgebrustMatrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let mat2 = AlgebrustMatrix::new(&[&[5.0, 6.0], &[7.0, 8.0]]);
        let result = mat1.subtraction(&mat2);
        assert_eq!(result.matrix_ref(), &vec![vec![-4.0, -4.0], vec![-4.0, -4.0]]);
    }

    #[test]
    fn test_matrix_multiplication() {
        let mat1 = AlgebrustMatrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let mat2 = AlgebrustMatrix::new(&[&[5.0, 6.0], &[7.0, 8.0]]);
        let result = mat1.multiplication(&mat2);
        assert_eq!(result.matrix_ref(), &vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let mat = AlgebrustMatrix::new(&[&[1.0, 2.0], &[3.0, 4.0]]);
        let result = mat.scalar_multiplication(2.0);
        assert_eq!(result.matrix_ref(), &vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
    }

    #[test]
    fn test_matrix_transpose() {
        let mat = AlgebrustMatrix::new(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
        let result = mat.transpose();
        assert_eq!(result.matrix_ref(), &vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
    }

    #[test]
    fn test_matrix_lu_decomposition() {
        let mat = AlgebrustMatrix::new(&[&[2.0, 3.0], &[5.0, 4.0]]);
        let (l, u) = mat.lu_decomposition();
        assert_eq!(l.matrix_ref(), &vec![vec![1.0, 0.0], vec![2.5, 1.0]]);
        assert_eq!(u.matrix_ref(), &vec![vec![2.0, 3.0], vec![0.0, -3.5]]);
    }

    #[test]
    fn test_matrix_determinant() {
        let mat = AlgebrustMatrix::new(&[&[2.0, 3.0], &[5.0, 4.0]]);
        let result = mat.determinant();
        assert_eq!(result, -7.0);
    }

    #[test]
    fn test_matrix_inverse() {
        let mat = AlgebrustMatrix::new(&[&[4.0, 7.0], &[2.0, 6.0]]);
        let result = mat.inverse();
        assert_eq!(result.matrix_ref(), &vec![vec![0.6000000000000001, -0.7000000000000001], vec![-0.2, 0.4]]);
    }
}