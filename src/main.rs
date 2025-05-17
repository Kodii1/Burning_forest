use crate::forest::Forest;
mod forest;
mod plot;

fn main() {
    let mut data: Vec<(f32, f32)> = Vec::new();

    let mut density = 0;
    loop {
        density += 10;
        let mut forest = Forest::new(20000, 20000);
        forest.create_alive_trees2(density);
        forest.fire_random_tree();

        loop {
            forest.spread_fire();
            if forest.trees_last_burned_positions.is_empty() {
                break;
            }
        }
        let burned_percent =
            forest.burned as f32 / (forest.size.0 as f32 * forest.size.1 as f32) * 100.0;
        println!("D: {} B: {}", density as f32, burned_percent);
        data.push((density as f32, burned_percent));

        if density == 100 {
            break;
        }
    }
    if let Err(e) = plot::main(&data) {
        eprintln!("ERROR: {}", e);
    } else {
        println!("Chart generated");
    }
}
