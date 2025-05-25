# Burning_forest


How to use ?
cargo build --release

./target/release/BurningForest -d (usize) -i (usize) -x (usize) -y (usize)

-d -> how much will the forestation increase

-i -> number of iterations

-x -> forest x size

-y -> forest y size

Example:

./target/release/BurningForest -d 1 -i 200 -x 20000 -y 20000

default values:

x: 4000,
y: 4000,
iterations: 10,
density: 5,

