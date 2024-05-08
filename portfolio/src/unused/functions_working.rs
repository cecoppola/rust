use std::fs::File;
use csv::ReaderBuilder;

pub fn read_csv_data(filename: &str) -> (Vec<String>, Vec<f64>, Vec<Vec<f64>>, i64) {

  let file = File::open(filename).expect("Error opening file");
  let mut reader = ReaderBuilder::new()
    .has_headers(false)
    .from_reader(file);

  let mut string_data: Vec<String> = Vec::new();
  let mut float_data:  Vec<f64> = Vec::new();
  let mut matrix_data: Vec<Vec<f64>> = Vec::new();
  let mut size: i64 = 0;

  for record in reader.records() {
    size += 1;
    let record = record.expect("Error reading record");
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

    (string_data, float_data, matrix_data, size)
}

pub fn _display_matrix(matrix: &Vec<Vec<f64>>) {
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

pub fn _display_vector(vector: &Vec<f64>) {
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

  let mut mean_vector = means.to_vec();
  //for element in mean_vector.iter_mut() { *element -= risk_free_rate; }  

  let cov_matrix = covariances.to_vec();
  let cov_matrix_inv = match inverse(&cov_matrix) {
      Some(cov_matrix_inv) => cov_matrix_inv,
      None => panic!("Cannot compute inverse of covariance matrix"),
  };
  let numerator = multiply_matrix_vector(&cov_matrix_inv, &mean_vector);
  let portfolio_variance = multiply_vector_vector(&mean_vector, &numerator);

  println!("variance = {:2.5}", portfolio_variance);

  let mut tangency_weights: Vec<f64> = numerator
      .iter()
      .map(|num| num / portfolio_variance)
      .collect();

      
  //enforce condition of being unable to short programs
  tangency_weights.iter_mut().for_each(|w| *w = w.max(0.0));
	let total_tangency_weight: f64 = tangency_weights.iter().sum();
	tangency_weights.iter_mut().for_each(|w| *w /= total_tangency_weight);
	//for i in tangency_weights.iter() { println!("{i}") }
  tangency_weights

}

pub fn calculate_portfolio_return(weights: &[f64], means: &[f64]) -> f64 {
  weights.iter().zip(means.iter()).map(|(&w, &mu)| w * mu).sum()
}

pub fn calculate_portfolio_variance(weights: &[f64], cov_inv: &[Vec<f64>], size: &i64) -> f64 {
  let weights_covariance = multiply_matrix_vector(cov_inv, weights);
  let mut portfolio_variance = multiply_vector_vector(weights, &weights_covariance);
  //portfolio_variance /= *size as f64;
  //portfolio_variance
  portfolio_variance.sqrt()
}

pub fn sort_with_indices(data: &[f64]) -> (Vec<f64>, Vec<usize>) {
	let mut indices: Vec<usize> = (0..data.len()).collect();
	indices.sort_by(|&a, &b| data[b].partial_cmp(&data[a]).unwrap());
	let mut sorted_data: Vec<f64> = data.iter().cloned().collect();
	sorted_data.sort_by(|a, b| b.partial_cmp(a).unwrap());
	(sorted_data, indices)
}
