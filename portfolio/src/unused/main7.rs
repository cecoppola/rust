#[allow(non_snake_case)]
use std::env;
mod functions;

fn main() {

  let args: Vec<String> = env::args().collect();
  let fund: f64 = args[1].parse::<f64>().unwrap();
  let size: usize = args[2].parse::<usize>().unwrap();

  println!("size = {}", size);
 
  //let costs = functions::gen_costs(size);
  //for c in &costs { println!("costs {:>5.2}", c); }

  let means = functions::gen_means(size);
  //for m in &means { println!("means {:>5.2}", m); }

  let covariances = functions::gen_covariances(size);
  //functions::write_csv_data("random.csv", &costs, &means, &covariances);
  functions::write_csv_data("random.csv", &means, &covariances);

  //let(names, means, covariances, size)  = functions::read_csv_data("assets.csv");
  let(names, means, covariances, size) = functions::read_csv_data("random.csv");
  println!("Found {size} assets.");
  let tangency_weights = functions::compute_tangency_portfolio(&means, &covariances, 0.0);
  //for w in &tangency_weights { println!("weight {:>5.2}", 100.0 * w); }

  let (sorted_data, indices) = functions::sort_with_indices(&tangency_weights);
  for (idx, weight) in sorted_data.iter().enumerate() {
    if weight > &0.0 {
      println!("Asset: {:4} \t \t  ${:>width$.0}", names[indices[idx]], weight * fund,width = 9);
    }
  }  
  
  let weighted_return = functions::calculate_portfolio_return(&tangency_weights, &means);
  let weighted_variance = functions::calculate_portfolio_variance(&tangency_weights, &covariances);
  println!("_________________________________________________");
  println!("Portfolio return: \t $ {:>width$.0} \u{00b1} {:>width$.0}",
  weighted_return   * fund, weighted_variance * fund, width = 9);
  
}
