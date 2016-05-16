extern crate time;
extern crate rayon;

use std::io;

const OUTPUT_TIMES : bool = false;

fn factorsums(sums: &mut [u64], start: u64) {
	// Either split in two or calculate current part of array
	// TODO make splitting condition depend on how big numbers are
	if sums.len() > 50 {
		let split = sums.len()/2;
		let (left, right) = sums.split_at_mut(split);
		let split = split as u64;
		rayon::join(|| factorsums(left, start),
					|| factorsums(right, start+split));
	} else {
		for i in 0..sums.len() {
			sums[i] = 0;
			let cur_number = i as u64 + start;
			
			let limit = (cur_number as f64).sqrt() as u64 + 1;
			for factor in 1..limit {
				if cur_number % factor == 0 {
					sums[i] += factor;
					if factor*factor != cur_number {
						sums[i] += cur_number/factor;
					}
				}
			}
		}
	}
}

fn find_matching(sums: &[u64], mystart: u64, myend: u64, globalstart: u64, globalend: u64) -> Vec<(u64, u64)> {
	if myend-mystart > 50 {
		let split = (myend-mystart) / 2 + mystart;
		let (mut a, b) = rayon::join(|| find_matching(sums, mystart, split, globalstart, globalend),
								 || find_matching(sums, split, myend, globalstart, globalend));
		a.extend(b);
		return a;
	} else {
		let mut v = Vec::new();
		for a in mystart..myend {
			for b in (a+1)..globalend {
				let ia = (a - globalstart) as usize;
				let ib = (b - globalstart) as usize;
				
				if sums[ia]*b == sums[ib]*a {v.push((a, b));}
			}
		}
		return v;
	}
}

fn find_mut_friendly(start: u64, end: u64) {
	let begin = time::now();

	let mut sums : Vec<u64> = vec![0; (end-start) as usize];
	factorsums(sums.as_mut_slice(), start);
	
	let sums = sums;
	
	let before_search = time::now();
	if OUTPUT_TIMES {println!("{} - Start searching", before_search - begin);}
	
	let pairs = find_matching(sums.as_slice(), start, end, start, end);
	
	if OUTPUT_TIMES {println!("{} - Total", time::now() - before_search);}
	else {
		for item in pairs {
			println!("{} and {} are FRIENDLY", item.0, item.1);
		}
	}
}

fn main() {

	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Could not read stdin");
		let input = input;
		let v : Vec<u64> = input.trim().split(' ').map(|x| x.parse::<u64>().unwrap() ).collect();
		
		if v.len() < 2 {
			println!("Please enter at least two numbers!");
			continue;
		}
		
		let start = v[0];
		let end = v[1]+1;
		
		if end == 1 && start == 0 {break;}
		
		find_mut_friendly(start, end);
	}
}
