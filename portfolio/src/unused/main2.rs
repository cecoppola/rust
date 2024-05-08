use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_means(filename: &str) -> Vec<f64> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut means = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }
        let value: f64 = line
            .trim()
            .parse()
            .expect("Failed to parse float from line");
        means.push(value);
    }

    means
}

fn read_covariances(filename: &str) -> Vec<Vec<f64>> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut covariances = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }
        let values: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse float from line"))
            .collect();
        covariances.push(values);
    }

    covariances
}

fn compute_tangency_portfolio(means: &[f64], covariances: &[Vec<f64>]) -> Vec<f64> {
    let num_assets = means.len();
    let cov_matrix = covariances.to_vec();
    let mean_vector = means.to_vec();

    // Compute inverse of covariance matrix
    let cov_matrix_inv = match inverse(&cov_matrix) {
        Some(matrix) => matrix,
        None => panic!("Cannot compute inverse of covariance matrix"),
    };

    // Compute weights for the tangency portfolio
    let ones = vec![1.0; num_assets];
    let numerator = multiply_matrix_vector(&cov_matrix_inv, &mean_vector);
    let denominator = multiply_matrix_vector(&cov_matrix_inv, &ones);
    let mut tangency_weights: Vec<f64> = numerator
        .iter()
        .zip(denominator.iter())
        .map(|(num, den)| num / den)
        .collect();

    // Normalize weights
    let total_weight: f64 = tangency_weights.iter().sum();
    tangency_weights.iter_mut().for_each(|w| *w /= total_weight);

    tangency_weights
}

fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .zip(vector.iter())
                .map(|(a, b)| a * b)
                .sum::<f64>()
        })
        .collect()
}

// Function to compute the inverse of a matrix (assuming square matrix)
fn inverse(matrix: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = matrix.len();
    let mut a = matrix.to_vec();
    let mut b = vec![vec![0.0; n]; n];
    for i in 0..n {
        b[i][i] = 1.0;
    }
    for i in 0..n {
        let mut max_row = i;
        for j in (i + 1)..n {
            if a[j][i].abs() > a[max_row][i].abs() {
                max_row = j;
            }
        }
        if max_row != i {
            a.swap(i, max_row);
            b.swap(i, max_row);
        }
        if a[i][i] == 0.0 {
            return None; // Matrix is singular
        }
        let scale = 1.0 / a[i][i];
        for j in 0..n {
            a[i][j] *= scale;
            b[i][j] *= scale;
        }
        for j in 0..n {
            if j != i {
                let scale = a[j][i];
                for k in 0..n {
                    a[j][k] -= a[i][k] * scale;
                    b[j][k] -= b[i][k] * scale;
                }
            }
        }
    }
    Some(b)
}

fn main() {
    let means = read_means("/home/machinus/rust/portfolio/src/mu.txt");
    let covariances = read_covariances("/home/machinus/rust/portfolio/src/sigma.txt");

    println!("Means: {:?}", means);
    println!("Covariances: {:?}", covariances);

    let tangency_weights = compute_tangency_portfolio(&means, &covariances);

    println!("Tangency Portfolio Weights: {:?}", tangency_weights);
}