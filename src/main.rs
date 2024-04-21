fn main() {
    const SEARCH_RANGE: usize = 100;

	let instant = std::time::Instant::now();
    println!("{:?}", primes(SEARCH_RANGE));
	println!("Took {}ms", instant.elapsed().as_millis());
}

fn primes(search_range: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut pool = Vec::<(usize, usize)>::new();

    for index in 1..=search_range {

	    let mut applicable_pool = Vec::<(usize, usize)>::new();

	    if index == 1 {
		    continue
	    }
	    if pool.is_empty() {
		    primes.push(index);
		    pool.push((index, index.pow(2)));
		    continue
	    }

	    let smallest_multiple: usize = pool
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