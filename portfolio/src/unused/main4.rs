use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};
//use rand::distributions::{Distribution, Standard};
use rand::Rng;


fn generate_mean_returns(num_assets: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..num_assets)
        .map(|_| rng.gen::<f64>() * 0.05) // Adjust the factor as needed
        .collect()
}

fn generate_covariance_matrix(num_assets: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut cov_matrix = vec![vec![0.0; num_assets]; num_assets];
    for i in 0..num_assets {
        for j in 0..num_assets {
            if i == j {
                cov_matrix[i][j] = rng.gen::<f64>() * 0.1; // Adjust the factor as needed
            } else {
                cov_matrix[i][j] = rng.gen::<f64>() * 0.05; // Adjust the factor as needed
            }
        }
    }
    cov_matrix
}

fn write_mean_returns(filename: &str, means: &[f64]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for mean in means {
        writeln!(writer, "{}", mean)?;
    }
    Ok(())
}

fn write_covariance_matrix(filename: &str, cov_matrix: &[Vec<f64>]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for row in cov_matrix {
        for value in row {
            write!(writer, "{:.8} ", value)?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

/// Read means from a file into a vector
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

/// Read covariances from a file into a matrix
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

/// Compute the tangency portfolio weights based on CAPM
fn compute_tangency_portfolio(means: &[f64], covariances: &[Vec<f64>], risk_free_rate: f64) -> Vec<f64> {
    //let num_assets = means.len();
    let cov_matrix = covariances.to_vec();
    let mean_vector = means.to_vec();

    // Compute inverse of covariance matrix
    let cov_matrix_inv = match inverse(&cov_matrix) {
        Some(matrix) => matrix,
        None => panic!("Cannot compute inverse of covariance matrix"),
    };

    // Compute weights for the tangency portfolio
    // let ones = vec![1.0; num_assets];
    let numerator = multiply_matrix_vector(&cov_matrix_inv, &mean_vector);
    let portfolio_variance = multiply_vector_vector(&numerator, &mean_vector);
    let mut tangency_weights: Vec<f64> = numerator
        .iter()
        .map(|num| num / portfolio_variance)
        .collect();

    // Ensure weights are non-negative
    tangency_weights.iter_mut().for_each(|w| *w = w.max(0.0));

    // Normalize weights
    let total_weight: f64 = tangency_weights.iter().sum();
    tangency_weights.iter_mut().for_each(|w| *w /= total_weight);

    // Adjust weights based on tangent condition
    adjust_tangent_condition(&mut tangency_weights, &cov_matrix_inv, &mean_vector, risk_free_rate);

    // Ensure weights are non-negative again after adjustment
    tangency_weights.iter_mut().for_each(|w| *w = w.max(0.0));

    // Normalize tangent portfolio weights
    let total_tangency_weight: f64 = tangency_weights.iter().sum();
    tangency_weights.iter_mut().for_each(|w| *w /= total_tangency_weight);

    tangency_weights
}


/// Adjust portfolio weights to satisfy tangent condition
fn adjust_tangent_condition(weights: &mut [f64], cov_inv: &[Vec<f64>], means: &[f64], risk_free_rate: f64) {
    let portfolio_return = calculate_portfolio_return(weights, means);
    let portfolio_variance = calculate_portfolio_variance(weights, cov_inv);

    // Compute lambda factor
    let lambda = (portfolio_return - risk_free_rate) / portfolio_variance;

    // Adjust weights to meet tangent condition
    weights.iter_mut().for_each(|w| *w -= lambda);
}

/// Calculate the return of a portfolio given weights and mean returns
fn calculate_portfolio_return(weights: &[f64], means: &[f64]) -> f64 {
    weights.iter().zip(means.iter()).map(|(&w, &mu)| w * mu).sum()
}

/// Calculate the variance of a portfolio given weights and the inverse covariance matrix
fn calculate_portfolio_variance(weights: &[f64], cov_inv: &[Vec<f64>]) -> f64 {
    let weights_covariance = multiply_matrix_vector(cov_inv, weights);
    multiply_vector_vector(weights, &weights_covariance)
}

/// Multiply a matrix by a vector
fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .zip(vector.iter())
                .map(|(&a, &b)| a * b)
                .sum::<f64>()
        })
        .collect()
}

/// Multiply two vectors
fn multiply_vector_vector(vector1: &[f64], vector2: &[f64]) -> f64 {
    vector1.iter()
        .zip(vector2.iter())
        .map(|(&a, &b)| a * b)
        .sum()
}

/// Compute the inverse of a matrix
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

    // Parameters
    let num_assets = 10;
    let mean_returns_filename = "mu.txt";
    let covariance_matrix_filename = "sigma.txt";

    // Generate synthetic financial data
    let mean_returns = generate_mean_returns(num_assets);
    let covariance_matrix = generate_covariance_matrix(num_assets);

    // Write data to files
    let _ = write_mean_returns(mean_returns_filename, &mean_returns);
    let _ = write_covariance_matrix(covariance_matrix_filename, &covariance_matrix);

    println!("Synthetic financial data generated successfully!");

    let means = read_means("/home/machinus/rust/portfolio/mu.txt");
    let covariances = read_covariances("/home/machinus/rust/portfolio/sigma.txt");
    let risk_free_rate = 0.05; // Example risk-free rate

    println!("Means: {:?}", means);
    println!("Covariances: {:?}", covariances);

    let tangency_weights = compute_tangency_portfolio(&means, &covariances, risk_free_rate);

    println!("Tangency Portfolio Weights:");
    // Find the maximum length of the formatted percentage strings
    let max_len = tangency_weights.iter()
        .map(|weight| format!("{:.2}%", weight * 100.0).len())
        .max()
        .unwrap_or(0);

    // Print the weights with padding to align decimal points
    for (idx, weight) in tangency_weights.iter().enumerate() {
        // Format weight as percentage to two decimal places
        let weight_percent = format!("{:>width$.2}%", weight * 100.0, width = max_len);
        println!("Asset {}: {}", idx + 1, weight_percent);
    }

    // Calculate weighted return and variance
    let weighted_return = calculate_portfolio_return(&tangency_weights, &means);
    let weighted_variance = calculate_portfolio_variance(&tangency_weights, &covariances);

    println!("Weighted Return of Solution Portfolio: {:.2}%", weighted_return * 100.0);
    println!("Weighted Variance of Solution Portfolio: {:.2}", weighted_variance);
}


