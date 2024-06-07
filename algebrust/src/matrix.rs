use rand::{distributions::Uniform, Rng};

pub struct AlgebrustMatrix {
    matrix: Vec<Vec<f64>>
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

    pub fn new_identity(n: usize) -> Self {
        let matrix = Self::get_identity_matrix(n);
        AlgebrustMatrix { matrix }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.matrix.len(), self.matrix[0].len())
    }

    pub fn matrix_ref(&self) -> &Vec<Vec<f64>> {
        &self.matrix
    }

    fn get_column(&self, column_index: usize, other: &AlgebrustMatrix) -> Vec<f64> {
        other.matrix.iter().map(|row| row[column_index]).collect()
    }

    fn get_identity_matrix(n: usize) -> Vec<Vec<f64>> {
        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
        for i in 0..n { matrix[i][i] = 1.0; }
        matrix
    }

    fn get_diagonal_product(&self, matrix: &Vec<Vec<f64>>, n: usize) -> f64 {
        (0..n)
            .map(|i| matrix[i][i])
            .product()
    }

    fn forward_substitution(&self, l: &Vec<Vec<f64>>, b: &Vec<f64>) -> Vec<f64> {
        let n = l.len();
        let mut x = vec![0.0; n];
        for i in 0..n {
            let summation: f64 = (0..i)
                .map(|j| l[i][j] * x[j])
                .sum();
            x[i] = (b[i] - summation) / l[i][i];
        }
        
        x
    }

    fn backward_substitution(&self, u: &Vec<Vec<f64>>, x: &Vec<f64>) -> Vec<f64> {
        let n = u.len();
        let mut y = vec![0.0; n];
        for i in (0..n).rev() {
            let summation: f64 = ((i + 1)..n)
                .map(|j| u[i][j] * y[j])
                .sum();
            y[i] = (x[i] - summation) / u[i][i];
        }
        
        y
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
        let (n, o) = self.size();
        let (p, m) = other.size();
        assert_eq!(o, p);
        let mut matrix = vec![vec![0.0; m]; n];
        for i in 0..n {
            for j in 0..m {
                matrix[i][j] = self.get_column(j, other)
                    .iter()
                    .zip(&self.matrix[i])
                    .map(|(a, b)| a * b)
                    .sum();
            }
        }
        AlgebrustMatrix { matrix }
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

    pub fn transpose(&self) -> AlgebrustMatrix {
        let (n, m) = self.size();
        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; m];
        for i in 0..n {
            for j in 0..m {
                matrix[j][i] = self.matrix[i][j];
            }
        }
        AlgebrustMatrix { matrix }
    }

    pub fn lu_decomposition(&self) -> (AlgebrustMatrix, AlgebrustMatrix) {
        let (n, m) = self.size();
        assert_eq!(n, m);
        let mut l: Vec<Vec<f64>> = Self::get_identity_matrix(n);
        let mut u: Vec<Vec<f64>> = self.matrix.clone();
        for j in 0..n {
            for i in j+1..n {
                l[i][j] = u[i][j] / u[j][j];
                for k in j..n {
                    u[i][k] = u[i][k] - l[i][j] * u[j][k];
                }
            }
        }
        (AlgebrustMatrix{matrix: l}, AlgebrustMatrix{matrix: u})
    }

    pub fn determinant(&self) -> f64 {
        let (n, m) = self.size();
        assert_eq!(n, m);
        let (l, u) = self.lu_decomposition();
        let determinant_l: f64 = self.get_diagonal_product(l.matrix_ref(), n);
        let determinant_u: f64 = self.get_diagonal_product(u.matrix_ref(), n);
        determinant_l * determinant_u
    }

    pub fn inverse(&self) -> AlgebrustMatrix {
        let (n, m) = self.size();
        assert_eq!(n, m);
        let (l, u) = self.lu_decomposition();
        let identity_matrix = AlgebrustMatrix { matrix: Self::get_identity_matrix(n) };
        let mut inverse_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            let e_i = self.get_column(i, &identity_matrix);
            let x_i = self.forward_substitution(l.matrix_ref(), &e_i);
            let y_i = self.backward_substitution(u.matrix_ref(), &x_i);
            for j in 0..n {
                inverse_matrix[j][i] = y_i[j];
            }
        }
        AlgebrustMatrix { matrix: inverse_matrix }
    }
}