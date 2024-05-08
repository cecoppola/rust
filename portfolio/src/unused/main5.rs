use rand::Rng;
mod functions;

fn main() {

  // Parameters
  let num_assets = 30;
  let mean_returns_filename = "/home/machinus/rust/portfolio/means.txt";
  let covariance_matrix_filename = "/home/machinus/rust/portfolio/covars.txt";

  //Test matrix arithmetic functions
  let mut random_matrix = vec![vec![0.0; num_assets]; num_assets];
  let mut rng = rand::thread_rng();
  for i in 0..num_assets { for j in 0..num_assets { random_matrix[i][j] = rng.gen::<f64>(); } }
  functions::display_matrix(&random_matrix);

  let inverse_random = match functions::inverse(&random_matrix) {
    Some(matrix) => matrix,
    None => panic!("Cannot compute inverse of covariance matrix"),
  };
  functions::display_matrix(&inverse_random);

  match functions::multiply_matrices(&random_matrix, &inverse_random) {
    Some(result) => {
        println!("Result:");
        functions::display_matrix(&result);
    }
    None => println!("Matrices cannot be multiplied"),
  }


  // Generate synthetic financial data
  let mean_returns = functions::generate_mean_returns(num_assets);
  let covariance_matrix = functions::generate_covariance_matrix(num_assets);

  // Write data to files
  let _ = functions::write_mean_returns(mean_returns_filename, &mean_returns);
  let _ = functions::write_covariance_matrix(covariance_matrix_filename, &covariance_matrix);

  let means = functions::read_means("/home/machinus/rust/portfolio/means.txt");
  let covariances = functions::read_covariances("/home/machinus/rust/portfolio/covars.txt");
  let risk_free_rate = 0.00; // Example risk-free rate

  println!("Means: {:?}", means);
  println!("Covariances: {:?}", covariances);

  let tangency_weights = functions::compute_tangency_portfolio(&means, &covariances, risk_free_rate);

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
    //println!("Asset {}: {}", idx + 1, weight_percent);
    // Calculate the mean return value for the asset
    let mean_return_value = means[idx];
    // Print the asset index, weight, and mean return value
    println!("Asset {:>2}: {:>3}  Mean Return: {:.2}%", idx + 1, weight_percent, mean_return_value * 100.0);
  }

  // Calculate weighted return and variance
  let weighted_return = functions::calculate_portfolio_return(&tangency_weights, &means);
  let weighted_variance = functions::calculate_portfolio_variance(&tangency_weights, &covariances);

  // Calculate the average value of the means
  //let mean_sum: f64 = mean_returns.iter().sum();
  //let mean_average = mean_sum / num_assets as f64;

  //let cov_sum: f64 = covariance_matrix.iter().flatten().sum();
  //let denom = (num_assets*num_assets) as f64;
  //let cov_average = cov_sum / denom as f64;

  //println!("Average value of the means: {:.2}% \u{00b1} {:.2}%", mean_average * 100.0, cov_average * 100.0);   

  println!("Weighted Return of Solution Portfolio: {:.2}% \u{00b1} {:.2}%", weighted_return * 100.0, weighted_variance * 100.0);

}
