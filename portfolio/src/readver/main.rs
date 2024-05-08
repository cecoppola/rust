use std::env;
mod functions;

fn main() {

  let args: Vec<String> = env::args().collect();
  let size: usize = args[1].parse::<usize>().unwrap();
  let costs = functions::gen_costs(size);
  let means = functions::gen_means(size);
  let covariances = functions::gen_covariances(size);
  functions::write_csv_data("random.csv", &costs, &means, &covariances);

  let(names, means, covariances, size)  = functions::read_csv_data("assets.csv");
  println!("Found {size} assets.");
  let mut tangency_weights = functions::compute_tangency_portfolio(&means, &covariances, 0.0);
  let (sorted_data, indices) = functions::sort_with_indices(&tangency_weights);
  for (idx, weight) in sorted_data.iter().enumerate() {
    if weight > &0.0 {
      println!("Asset: {:4} {:>width$.2}, diagonal return: {:>width$.2} \u{00b1} {:>width$.2}",
      names[indices[idx]], weight * 100.0, means[indices[idx]] * 100.0,
      covariances[indices[idx]][indices[idx]] * 100.0, width = 5);
    }
  }  
  
  let weighted_return = functions::calculate_portfolio_return(&tangency_weights, &means);
  let weighted_variance = functions::calculate_portfolio_variance(&tangency_weights, &covariances, &size);
  println!("_________________________________________________");
  println!("Portfolio return: \t \t    {:>width$.2} \u{00b1} {:>width$.2}",
  weighted_return   * 100.0, weighted_variance * 100.0, width = 5);
  
}
