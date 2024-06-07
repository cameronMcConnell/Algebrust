# Algebrust
Algebrust is a high-performance linear algebra library for Rust, tailored for efficient mathematical operations on matrices and vectors. Leveraging the safety and expressiveness of Rust, Algebrust provides a reliable toolkit for numerical computing tasks in fields such as machine learning, scientific computing, and computer graphics.

## Features

- Vector addition, subtraction, dot product, scalar multiplication, magnitude, normalization, and cross product.
- Matrix addition, subtraction, multiplication, scalar multiplication, transpose, LU decomposition, determinant calculation, and inversion.

## Usage

To use Algebrust in your project, add it as a dependency in your `Cargo.toml` file. You can do this manually or by using the `cargo add` command.

### Using `cargo add`

```sh
cargo add algebrust
```

### Manually Editing `Cargo.toml`

Add the following line to your `Cargo.toml` under `[dependencies]`:

```t
[dependencies]
algebrust = "0.1.0"
```

## Usage

Here are some examples to get you started:

### Vectors

```rust
use algebrust::AlgebrustVector;

// Creating vectors
let v1 = AlgebrustVector::new(&[1.0, 2.0, 3.0]);
let v2 = AlgebrustVector::new_rand(3, 0.0, 10.0);
let v3 = AlgebrustVector::new_zeros(3);

// Vector operations
let v4 = v1.addition(&v2);
let v5 = v1.subtraction(&v2);
let dot = v1.dot_product(&v2);
let cross = v1.cross_product(&v2);
let scalar_mult = v1.scalar_multiplication(2.0);
let magnitude = v1.magnitude();
let normalized = v1.normalization();
```

### Matrices

```rust
use algebrust::AlgebrustMatrix;

// Creating matrices
let m1 = AlgebrustMatrix::new(&[
    &[1.0, 2.0],
    &[3.0, 4.0]
]);
let m2 = AlgebrustMatrix::new_rand((2, 2), 0.0, 10.0);
let m3 = AlgebrustMatrix::new_zeros((2, 2));
let m4 = AlgebrustMatrix::new_identitiy(2);

// Matrix operations
let m5 = m1.addition(&m2);
let m6 = m1.subtraction(&m2);
let m7 = m1.multiplication(&m2);
let scalar_mult = m1.scalar_multiplication(2.0);
let transpose = m1.transpose();

// LU decomposition and matrix inversion
let (l, u) = m1.lu_decomposition();
let inverse = m1.inverse();
let determinant = m1.determinant();
```

## Testing

To run tests, use the following command:

```sh
cargo test
```

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.