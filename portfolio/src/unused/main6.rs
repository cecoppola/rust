mod functions;

fn main() {

  let(col1, col2, mat1)  = functions::read_csv_data("/home/machinus/rust/portfolio/assets2.csv");
  functions::display_vector(&col2);
  functions::display_matrix(&mat1);

  let means = functions::read_means("/home/machinus/rust/portfolio/means.txt");
  let covariances = functions::read_covariances("/home/machinus/rust/portfolio/covars.txt");
  //functions::display_vector(&means);
  //functions::display_matrix(&covariances);

  let tangency_weights = functions::compute_tangency_portfolio(&means, &covariances, 0.0);

  println!("Tangency Portfolio Weights:");
  let max_len = tangency_weights.iter()
    .map(|weight| format!("{:.2}%", weight * 100.0).len())
    .max()
    .unwrap_or(0);

  for (idx, weight) in tangency_weights.iter().enumerate() {
    let weight_percent = format!("{:>width$.2}%", weight * 100.0, width = max_len);
    let mean_return_value = format!("{:>width$.2}%", means[idx] * 100.0, width = max_len);
    println!("Asset {:2}: {:2.5}  Mean Return: {:2.5}%", idx + 1, weight_percent, mean_return_value);
  }

  let weighted_return = functions::calculate_portfolio_return(&tangency_weights, &means);
  let weighted_variance = functions::calculate_portfolio_variance(&tangency_weights, &covariances);

  println!("Weighted Return of Solution Portfolio: {:.2}% \u{00b1} {:.2}%",
    weighted_return   * 100.0,
    weighted_variance * 100.0);

  //tangency_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let sorted_data: Vec<f64>;
  let indices: Vec<usize>;
  (sorted_data, indices) = functions::sort_with_indices(&tangency_weights);

  for (idx, weight) in sorted_data.iter().enumerate() {
    let weight_percent = format!("{:>width$.2}%", weight * 100.0, width = max_len);
    let mean_return_value = format!("{:>width$.2}%", means[indices[idx]] * 100.0, width = max_len);
    if weight > &0.0 {
      println!("Asset {:2}: {:2.5}  Mean Return: {:2.5}%", idx + 1, weight_percent, mean_return_value);
    }
  }  

}
