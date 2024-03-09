# Dijkstra's Prime Finding Algorithm, Implemented in Rust
A simple Rust implementation of Dijkstra's prime finding algorithm, without any dependencies.
## Usage
Download or clone this repo and run `cargo run`. Alternatively, run `cargo build --release` and open the generated executable binary, located in `./target/release`. By default, this will output a pretty-printed `Vec` with all found primes within a given range of `1..=count`, where count is set to 100000 by default. To change this, open `main.rs`:
```rust
fn main() {
    const COUNT : usize = 100_000; // Change for more/less prime-finding
	
	let instant = std::time::Instant::now();
    println!("{:#?}", primes(COUNT));
	println!("Took {}ms", instant.elapsed().as_millis());
}

fn primes(count : usize) -> Vec<usize> {
    // Function Code
}
```
The `COUNT : usize` const variable dictates the upper bound for the prime-finding range. Change it to your liking.
