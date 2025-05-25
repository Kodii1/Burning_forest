use crate::forest::Forest;
mod forest;
mod plot;
use std::collections::HashMap;
use ordered_float::OrderedFloat;

struct Config {
    size: (usize, usize),
    iterations: usize,
    density: f32,
}

fn main() {
    let cfg = setup();
    let size = cfg.size;
    let mut data: Vec<(f32, f32)> = Vec::with_capacity(cfg.iterations);

    for iter in 0..cfg.iterations {
        println!("Iteration: {}", iter);
        let mut density = 0.0;
        while density < 100.0 {
            density += cfg.density;
            let mut forest = Forest::new(size.0, size.1);
            forest.create_alive_trees(density);
            forest.fire_random_tree();

            while !forest.trees_last_burned_positions.is_empty() {
                forest.spread_fire();
            }
            let burned_percent =
                forest.burned as f32 / (forest.size.0 as f32 * forest.size.1 as f32) * 100.0;
            data.push((density, burned_percent));
        }
    }

    let avg_data = calculate_avg_data(&data);
    calculate_most_optimal_density(&size, &avg_data);

    if let Err(e) = plot::create_plot(&avg_data) {
        eprintln!("ERROR: {}", e);
    } else {
        println!("Chart generated");
    }
}


fn calculate_avg_data(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut groups: HashMap<OrderedFloat<f32>, Vec<f32>> = HashMap::new();

    for (density, value) in data {
        groups.entry(OrderedFloat(*density)).or_default().push(*value);
    }

    let mut avg_data: Vec<(f32, f32)> = groups
        .into_iter()
        .map(|(density, values)| {
            let sum: f32 = values.iter().sum();
            let avg = sum / values.len() as f32;
            (density.into_inner(), avg)
        })
        .collect();

    avg_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    avg_data
}

fn calculate_most_optimal_density(size: &(usize, usize), avg_data: &Vec<(f32, f32)>) {
    let total_cells = (size.0 * size.1) as f32;

    let mut most_optimal = {
        let (coverage_ratio, burned_ratio) = avg_data[0];
        let tree_count = (coverage_ratio / 100.0) * total_cells;
        let burned_count = (burned_ratio / 100.0) * total_cells;
        let trees_alive = tree_count - burned_count;
        (trees_alive, coverage_ratio)
    };

    for (coverage_ratio, burned_ratio) in &avg_data[1..] {

        let tree_count = (coverage_ratio / 100.0) * total_cells;
        let burned_count = (burned_ratio / 100.0) * total_cells;
        let trees_alive = tree_count - burned_count;

        if trees_alive > most_optimal.0 {
            most_optimal = (trees_alive, *coverage_ratio);
        }
    }

    println!(
        "Most optimal coverage is {:.2}% with {:.0} trees surviving.",
        most_optimal.1,
        most_optimal.0
    );
}

fn setup() -> Config {
    let mut cfg: Config = Config {
        size: (4000, 4000),
        iterations: 10,
        density: 5.0,
    };
    let args: Vec<String> = std::env::args().collect();
    let mut opts = args.iter().skip(1);
    while let Some(arg) = opts.next() {
        match arg.as_str() {
            "-i" => {
                if let Some(value) = opts.next() {
                    println!("-i = {}", value);
                    cfg.iterations = value.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("ERROR: -i is not a usize");
                        std::process::exit(1);
                    });
                } else {
                    println!("Don't see any arguments after -i");
                    std::process::exit(1);
                }
            }
            "-d" => {
                if let Some(value) = opts.next() {
                    println!("-d = {}", value);
                    cfg.density = value.parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("ERROR: -d is not a f32");
                        std::process::exit(1);
                    });
                } else {
                    println!("Don't see any arguments after -d");
                    std::process::exit(1);
                }
            }
            "-x" => {
                if let Some(value) = opts.next() {
                    println!("-s = {}", value);
                    let x = value.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("ERROR: -n is not a usize");
                        std::process::exit(1);
                    });
                    cfg.size.0 = x;
                } else {
                    println!("Don't see any arguments after -s");
                    std::process::exit(1);
                }
            }
            "-y" => {
                if let Some(value) = opts.next() {
                    println!("-s = {}", value);
                    let y = value.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("ERROR: -n is not a usize");
                        std::process::exit(1);
                    });
                    cfg.size.1 = y;
                } else {
                    println!("Don't see any arguments after -s");
                    std::process::exit(1);
                }
            }
            _ => {
                println!("args: {}", arg);
            }
        }
    }
    cfg
}
