use rand::{distributions::Uniform, Rng};

pub struct AlgebrustVector {
    vector: Vec<f64>
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