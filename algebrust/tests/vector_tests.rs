#[cfg(test)]
mod tests {
    use algebrust::AlgebrustVector;

    #[test]
    fn test_vector_new() {
        let vec = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
        assert_eq!(vec.vector_ref(), &vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_vector_new_rand() {
        let vec = AlgebrustVector::new_rand(3, 0.0, 1.0);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_vector_new_zeros() {
        let vec = AlgebrustVector::new_zeros(3);
        assert_eq!(vec.vector_ref(), &vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_vector_addition() {
        let vec1 = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
        let vec2 = AlgebrustVector::new(&[4.0, 5.0, 6.0]);
        let result = vec1.addition(&vec2);
        assert_eq!(result.vector_ref(), &vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_vector_subtraction() {
        let vec1 = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
        let vec2 = AlgebrustVector::new(&[4.0, 5.0, 6.0]);
        let result = vec1.subtraction(&vec2);
        assert_eq!(result.vector_ref(), &vec![-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_vector_dot_product() {
        let vec1 = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
        let vec2 = AlgebrustVector::new(&[4.0, 5.0, 6.0]);
        let result = vec1.dot_product(&vec2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let vec = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
        let result = vec.scalar_multiplication(2.0);
        assert_eq!(result.vector_ref(), &vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_vector_magnitude() {
        let vec = AlgebrustVector::new(&[3.0, 4.0]);
        let result = vec.magnitude();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vector_normalization() {
        let vec = AlgebrustVector::new(&[3.0, 4.0]);
        let result = vec.normalization();
        assert_eq!(result.vector_ref(), &vec![0.6, 0.8]);
    }

    #[test]
    fn test_vector_cross_product() {
        let vec1 = AlgebrustVector::new(&[1.0, 0.0, 0.0]);
        let vec2 = AlgebrustVector::new(&[0.0, 1.0, 0.0]);
        let result = vec1.cross_product(&vec2);
        assert_eq!(result.vector_ref(), &vec![0.0, 0.0, 1.0]);
    }
}