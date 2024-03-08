fn main() {
    const COUNT : usize = 100_000;
	
	let instant = std::time::Instant::now();
    println!("{:#?}", primes(COUNT));
	println!("Took {}ms", instant.elapsed().as_millis());
}

fn primes(count : usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut pool = Vec::<(usize, usize)>::new();
	
    'index: for index in 1..=count {
	    
	    let mut applicable_pool = Vec::<(usize, usize)>::new();

	    if index == 1 {
		    continue 'index
	    }
	    if pool.is_empty() {
		    primes.push(index);
		    pool.push((index, index.pow(2)));
		    continue 'index
	    }

	    let smallest_multiple : usize = pool
		    .iter()
		    .map(|element| element.1)
		    .min()
		    .unwrap();

	    for tuple in &pool {
		    let (_prime, multiple) = tuple;

		    if *multiple == smallest_multiple {
			    applicable_pool.push(*tuple);
		    }
	    }
	    
	    if index < smallest_multiple {
		    primes.push(index);
		    pool.push((index, index.pow(2)));
	    }
	    if index == smallest_multiple {
		    for tuple in applicable_pool {
			    let (prime, mut multiple) = tuple;

			    let tuple_index = pool
				    .iter()
				    .position(|index| index.1 == multiple)
				    .unwrap();

			    multiple += prime;

			    pool[tuple_index] = (prime, multiple);
		    }
	    }
    }
	
    primes
}