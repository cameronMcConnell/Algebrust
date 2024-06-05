use std::iter::Sum;

use rand::{distributions::Uniform, Rng};

pub struct AlgebrustVector {
    pub vector: Vec<f32>
}

impl AlgebrustVector {
    pub fn new(vector: &[f32]) -> Self {
        AlgebrustVector { 
            vector: vector.to_vec()
        }
    }

    pub fn new_rand(length: usize, low: f32, high: f32) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(low, high);
        let vector: Vec<f32> = (0..length).map(|_| rng.sample(&range)).collect();
        AlgebrustVector { vector }
    }

    pub fn new_zeros(length: usize) -> Self {
        let vector = vec![0.0; length];
        AlgebrustVector { vector }
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn addition(&self, other: &AlgebrustVector) -> AlgebrustVector {
        assert_eq!(self.len(), other.len());
        let vector: Vec<f32> = self.vector.iter().zip(&other.vector).map(|(a, b)| a + b).collect();
        AlgebrustVector { vector }
    }

    pub fn substraction(&self, other: &AlgebrustVector) -> AlgebrustVector {
        assert_eq!(self.len(), other.len());
        let vector: Vec<f32> = self.vector.iter().zip(&other.vector).map(|(a, b)| a - b).collect();
        AlgebrustVector { vector }
    }

    pub fn dot_product(&self, other: &AlgebrustVector) -> f32 {
        assert_eq!(self.len(), other.len());
        self.vector.iter().zip(&other.vector).map(|(a, b)| a * b).sum()
    }

    pub fn scalar_multiplication(&self, scaler: f32) -> AlgebrustVector {
        let vector: Vec<f32> = self.vector.iter().map(|a| a * scaler).collect();
        AlgebrustVector { vector }
    }

    pub fn magnitude(&self) -> f32 {
        let squared_sum: f32 = self.vector.iter().map(|a| a * a).sum();
        squared_sum.sqrt()
    }

    pub fn normalization(&self) -> AlgebrustVector {
        let magnitude = self.magnitude();
        let vector: Vec<f32> = self.vector.iter().map(|a| a / magnitude).collect();
        AlgebrustVector { vector }
    }

    pub fn cross_product(&self) -> AlgebrustVector {
        
    }
}