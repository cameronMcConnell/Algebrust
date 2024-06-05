use rand::{distributions::Uniform, Rng};

pub struct AlgebrustVector {
    vector: Vec<f64>
}

pub struct AlgebrustMatrix {
    matrix: Vec<Vec<f64>>
}


impl AlgebrustVector {
    pub fn new(vector: &[f64]) -> Self {
        AlgebrustVector { 
            vector: vector.to_vec()
        }
    }

    pub fn new_rand(length: usize, low: f64, high: f64) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(low, high);
        let vector: Vec<f64> = (0..length)
            .map(|_| rng.sample(&range))
            .collect();
        AlgebrustVector { vector }
    }

    pub fn new_zeros(length: usize) -> Self {
        let vector = vec![0.0; length];
        AlgebrustVector { vector }
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn vector_ref(&self) -> &Vec<f64> {
        &self.vector
    }

    pub fn addition(&self, other: &AlgebrustVector) -> AlgebrustVector {
        assert_eq!(self.len(), other.len());
        let vector: Vec<f64> = self.vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a + b)
            .collect();
        AlgebrustVector { vector }
    }

    pub fn subtraction(&self, other: &AlgebrustVector) -> AlgebrustVector {
        assert_eq!(self.len(), other.len());
        let vector: Vec<f64> = self.vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a - b)
            .collect();
        AlgebrustVector { vector }
    }

    pub fn dot_product(&self, other: &AlgebrustVector) -> f64 {
        assert_eq!(self.len(), other.len());
        self.vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn scalar_multiplication(&self, scaler: f64) -> AlgebrustVector {
        let vector: Vec<f64> = self.vector
            .iter()
            .map(|a| a * scaler)
            .collect();
        AlgebrustVector { vector }
    }

    pub fn magnitude(&self) -> f64 {
        let squared_sum: f64 = self.vector
            .iter()
            .map(|a| a * a)
            .sum();
        squared_sum.sqrt()
    }

    pub fn normalization(&self) -> AlgebrustVector {
        let magnitude = self.magnitude();
        let vector: Vec<f64> = self.vector
            .iter()
            .map(|a| a / magnitude)
            .collect();
        AlgebrustVector { vector }
    }

    pub fn cross_product(&self, other: &AlgebrustVector) -> AlgebrustVector {
        assert_eq!((self.len(), other.len()), (3, 3));
        AlgebrustVector { 
            vector: vec![
                (self.vector[1] * other.vector[2]) - (self.vector[2] * other.vector[1]),
                (self.vector[2] * other.vector[0]) - (self.vector[0] * other.vector[2]),
                (self.vector[0] * other.vector[1]) - (self.vector[1] * other.vector[0])
            ] 
        }
    }
}

impl AlgebrustMatrix {
    pub fn new(matrix: &[&[f64]]) -> Self {
        let matrix: Vec<Vec<f64>> = matrix
            .iter()
            .map(|vector| vector.to_vec())
            .collect();
        AlgebrustMatrix { matrix }
    }

    pub fn new_rand(size: (usize, usize), low: f64, high: f64) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(low, high);
        let matrix: Vec<Vec<f64>> = (0..size.0)
            .map(|_| (0..size.1).map(|_| rng.sample(&range)).collect())
            .collect();
        AlgebrustMatrix { matrix }
    }

    pub fn new_zeros(size: (usize, usize)) -> Self {
        let matrix: Vec<Vec<f64>> = (0..size.0)
            .map(|_| vec![0.0; size.1])
            .collect();
        AlgebrustMatrix { matrix }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.matrix.len(), self.matrix[0].len())
    }

    pub fn matrix_ref(&self) -> &Vec<Vec<f64>> {
        &self.matrix
    }

    pub fn addition(&self, other: &AlgebrustMatrix) -> AlgebrustMatrix {
        assert_eq!(self.size(), other.size());
        let matrix: Vec<Vec<f64>> = self.matrix
            .iter()
            .zip(&other.matrix)
            .map(|(vector1, vector2)| vector1
                .iter()
                .zip(vector2)
                .map(|(a, b)| a + b)
                .collect())
            .collect();
        AlgebrustMatrix { matrix }
    }

    pub fn subtraction(&self, other: &AlgebrustMatrix) -> AlgebrustMatrix {
        assert_eq!(self.size(), other.size());
        let matrix: Vec<Vec<f64>> = self.matrix
            .iter()
            .zip(&other.matrix)
            .map(|(vector1, vector2)| vector1
                .iter()
                .zip(vector2)
                .map(|(a, b)| a - b)
                .collect())
            .collect();
        AlgebrustMatrix { matrix }
    }

    pub fn multiplication(&self, other: &AlgebrustMatrix) -> AlgebrustMatrix {
        assert_eq!(self.size().1, other.size().0);
        let mut matrix = vec![vec![0.0; other.size().1]; self.size().0];
        for i in 0..self.size().0 {
            for j in 0..other.size().1 {
                matrix[i][j] = self.get_column(j, other)
                    .iter()
                    .zip(&self.matrix[i])
                    .map(|(a, b)| a * b)
                    .sum();
            }
        }
        AlgebrustMatrix { matrix }
    }

    fn get_column(&self, column_index: usize, other: &AlgebrustMatrix) -> Vec<f64> {
        other.matrix.iter().map(|row| row[column_index]).collect()
    }

    pub fn scalar_multiplication(&self, scaler: f64) -> AlgebrustMatrix {
        let matrix: Vec<Vec<f64>> = self.matrix
            .iter()
            .map(|vector| vector
                .iter()
                .map(|a| a * scaler)
                .collect())
            .collect();
        AlgebrustMatrix { matrix }
    }
}