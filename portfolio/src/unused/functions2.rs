use std::fs::File;
use std::io::{BufReader, BufRead};
//use std::convert::TryInto;
use std::error::Error;
use csv::ReaderBuilder;
use csv::Error as CsvError;

pub fn read_csv_data(filename: &str) -> (Vec<String>, Vec<f64>, Vec<Vec<f64>>) {

  let file = File::open(filename).expect("Error opening file");
  let mut reader = ReaderBuilder::new()
    .has_headers(false)
    //.delimiter(b'\t')
    .from_reader(file);

  let mut string_data: Vec<String> = Vec::new();
  let mut float_data:  Vec<f64> = Vec::new();
  let mut matrix_data: Vec<Vec<f64>> = Vec::new();

  for record in reader.records() {
    let record = record.expect("Error reading record");
    println!("{}", record.len());
    if record.len() < 2 {
      eprintln!("Error: Each row must have at least two columns");
      continue;
    }

    let string_value = record.get(0)
      .expect("Error getting first column")
      .to_string();
  
      let float_value = record.get(1)
        .expect("Error getting second column")
        .parse::<f64>()
        .expect("Error parsing float");

      string_data.push(string_value);
      float_data.push(float_value);

      let mut row_data: Vec<f64> = Vec::with_capacity(32);
      for field in record.iter().skip(2) {
        let float_value = field
          .parse::<f64>()
          .expect("Error parsing float in row");
        row_data.push(float_value);
      }
      matrix_data.push(row_data);
    }

    (string_data, float_data, matrix_data)
}



pub fn read_means(filename: &str) -> Vec<f64> {
  let file = File::open(filename).expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut means = Vec::new();
  for line in reader.lines() {
      let line = line.expect("Failed to read line");
      if line.trim().is_empty() {
          continue;
      }
      let mut value: f64 = line
          .trim()
          .parse()
          .expect("Failed to parse float from line");
			value *= 10.0;
      means.push(value);
  }
  means
}

pub fn read_covariances(filename: &str) -> Vec<Vec<f64>> {
  let file = File::open(filename).expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut covariances = Vec::new();
  for line in reader.lines() {
      let line = line.expect("Failed to read line");
      if line.trim().is_empty() {
          continue;
      }
      let mut values: Vec<f64> = line
          .split_whitespace()
          .map(|s| s.parse().expect("Failed to parse float from line"))
          .collect();
			for i in 0..values.len() {
        values[i] *= 10.0;
			}
			for i in 0..values.len() {
				if values[i] == 0.0 { values[i] = 0.0001; }
			}
      covariances.push(values);
  }
  covariances
}

pub fn display_matrix(matrix: &Vec<Vec<f64>>) {
  for row in matrix {
      for &element in row {
          if element >= 0.0 {
              print!(" {:7.4}", element); // Adjust the width as needed
          } else {
              print!(" {:7.4}", element); // Adjust the width as needed
          }
      }
      println!();
  }
}

pub fn display_vector(vector: &Vec<f64>) {
  for &element in vector {
    if element >= 0.0 {
      println!(" {:5.4} ", element); // Adjust the width as needed
    } else {
      println!("{:5.4} ", element); // Adjust the width as needed
    }
  }
  println!();
}

/// Multiply two vectors
pub fn multiply_vector_vector(vector1: &[f64], vector2: &[f64]) -> f64 {
  vector1.iter()
      .zip(vector2.iter())
      .map(|(&a, &b)| a * b)
      .sum()
}

/// Multiply a matrix by a vector
pub fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
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

pub fn _multiply_matrices(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
  let rows1 = matrix1.len();
  let cols1 = matrix1[0].len();
  let rows2 = matrix2.len();
  let cols2 = matrix2[0].len();

  if cols1 != rows2 {
      return None; // Matrices can't be multiplied
  }

  let mut result = vec![vec![0.0; cols2]; rows1];

  for i in 0..rows1 {
      for j in 0..cols2 {
          for k in 0..cols1 {
              result[i][j] += matrix1[i][k] * matrix2[k][j];
          }
      }
  }

  Some(result)
}

pub fn inverse(matrix: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
  let n = matrix.len();
  let mut a = matrix.to_vec();
  
  println!("n = {n}");

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

pub fn compute_tangency_portfolio(means: &[f64], covariances: &[Vec<f64>], risk_free_rate: f64) -> Vec<f64> {

  let mean_vector = means.to_vec();
  let cov_matrix = covariances.to_vec();
	//display_matrix(&cov_matrix);

  let cov_matrix_inv = match inverse(&cov_matrix) {
      Some(cov_matrix_inv) => cov_matrix_inv,
      None => panic!("Cannot compute inverse of covariance matrix"),
  };
	//display_matrix(&cov_matrix_inv);

  let numerator = multiply_matrix_vector(&cov_matrix_inv, &mean_vector);
  let portfolio_variance = multiply_vector_vector(&mean_vector, &numerator);

	let mut tangency_weights: Vec<f64> = numerator
      .iter()
      .map(|num| num / portfolio_variance)
      .collect();

	adjust_tangent_condition(&mut tangency_weights, &cov_matrix_inv, &mean_vector, risk_free_rate);
  tangency_weights.iter_mut().for_each(|w| *w = w.max(0.0));

	let total_tangency_weight: f64 = tangency_weights.iter().sum();
	tangency_weights.iter_mut().for_each(|w| *w /= total_tangency_weight);

	for i in tangency_weights.iter() { println!("{i}") }

  tangency_weights
}

/// Adjust portfolio weights to satisfy tangent condition
pub fn adjust_tangent_condition(weights: &mut [f64], cov_inv: &[Vec<f64>], means: &[f64], risk_free_rate: f64) {
  let portfolio_return = calculate_portfolio_return(weights, means);
  let portfolio_variance = calculate_portfolio_variance(weights, cov_inv);
  let lambda = (portfolio_return - risk_free_rate) / portfolio_variance;
  weights.iter_mut().for_each(|w| *w -= lambda);
}

/// Calculate the return of a portfolio given weights and mean returns
pub fn calculate_portfolio_return(weights: &[f64], means: &[f64]) -> f64 {
  weights.iter().zip(means.iter()).map(|(&w, &mu)| w * mu).sum()
}

/// Calculate the variance of a portfolio given weights and the inverse covariance matrix
pub fn calculate_portfolio_variance(weights: &[f64], cov_inv: &[Vec<f64>]) -> f64 {
  let weights_covariance = multiply_matrix_vector(cov_inv, weights);
  multiply_vector_vector(weights, &weights_covariance)
}

pub fn sort_with_indices(data: &[f64]) -> (Vec<f64>, Vec<usize>) {
	let mut indices: Vec<usize> = (0..data.len()).collect();

	// Sort indices based on data elements
	//indices.sort_by(|&a, &b| data[a].partial_cmp(&data[b]).unwrap());
	indices.sort_by(|&a, &b| data[b].partial_cmp(&data[a]).unwrap());

	let mut sorted_data: Vec<f64> = data.iter().cloned().collect();
	//sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
	sorted_data.sort_by(|a, b| b.partial_cmp(a).unwrap());
		
	(sorted_data, indices)
}




