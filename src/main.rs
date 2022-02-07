use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use clap::{Arg,App};
use std::thread;
use std::cmp;


#[derive(Copy, Clone, Debug)]
pub struct Board {
	squares: [[i8; 9]; 9]
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
	x: i8,
	y: i8    
}


fn main() {

	// create empty puzzle
	let mut board = Board { squares: [[0; 9] ;9] };

	// get command line arguments
	let args = App::new("sudoku-solver")
	.version("0.1.0")
	.about("Solve sudoku puzzles on the command line")
	.author("Roy Tanck")
	.args(&[
		Arg::new("input")
			.short('i')
			.long("input")
			.value_name("sudoku.txt")
			.help("File name of the sudoku puzzle file to solve (.txt).")
			.required(true)
			.takes_value(true),
		Arg::new("verbose")
			.short('v')
			.long("verbose")
			.help("Output extra information."),
		Arg::new("benchmark")
			.short('b')
			.long("benchmark")
			.value_name("100")
			.help("Run a benchmark by numming the solve multiple times.")
			.takes_value(true),
		Arg::new("threads")
			.short('t')
			.long("threads")
			.value_name("8")
			.help("Number of CPU threads to use when benchmarking.")
			.takes_value(true),
	]).get_matches();

	let filename = args.value_of("input").unwrap_or("extreme.txt");
	let verbose:bool = args.is_present("verbose");
	let benchmark:u32 = args.value_of("benchmark").unwrap_or( "0" ).parse::<u32>().unwrap();
	let mut threads:u32 = args.value_of("threads").unwrap_or( "8" ).parse::<u32>().unwrap();
	if threads < 1 { threads = 1; }
	if threads > benchmark { threads = cmp::min( 256, benchmark ); }

	// read the input file
	let file = match File::open( &filename ) {
		Err( why ) => { println!( "Unable to open file {}: {}.", filename, why ); return; },
		Ok( file ) => file,
	};
	let reader = BufReader::new(file);

	// iterate over the loaded file by line
	for ( y, line ) in reader.lines().enumerate() {
		let row = line.unwrap();
		if y < 9 {
			// iterate over the characters of the current line
			for ( x, c ) in row.chars().enumerate() {
				// if not out of bounds, put the value into the board 
				if x < 9 {
					let val = c as i8 - 0x30; // 0x30 is 0's ascii table offset
					if val >= 0 && val < 9 {
						board.squares[y][x] = val;	
					} else {
						board.squares[y][x] = 0;
					}
				}
			}
		}
	}

	// pre-solve output
	if verbose && benchmark <= 1 {
		println!("\nInput:\n");
		render( board, verbose );
	}

	// initialize stats variables
	let mut steptotal:u32 = 0;
	let mut solvetotal:u32 = 0;
	let mut solution:Board = Board { squares: [[0; 9] ;9] };
	let start = SystemTime::now();

	// if benchmark is set, run that number of solves
	if benchmark > 1 {
		let workers:u32 = threads;
		let runs_per_worker_f:f64 = f64::from( benchmark ) / f64::from( workers );
		let runs_per_worker:u32 = runs_per_worker_f.ceil() as u32;
		let mut runs_remaining:u32 = benchmark;
		let mut handles = Vec::new();
		for w in 0..workers {
			// figure out how many solves to outsource to the next thread
			let mut runs_this_thread = 0;
			if w < workers -1 {
				runs_this_thread = runs_per_worker;
				runs_remaining -= runs_this_thread;
			} else {
				runs_this_thread = runs_remaining;
			}
			// spawn a new thread
			let handle = thread::spawn( move ||  {
				let mut steps:u32 = 0;
				let mut solves:u32 = 0;
				for _run in 0..runs_this_thread {
					let ( _solution, stepcounter ) = solve( board );
					steps += stepcounter;
					solves += 1;
				}
				( solves, steps )
			});
			handles.push( handle );
		}
		// wait for all threads and gather stats from them
		for handle in handles {
			let thread_stats = handle.join().unwrap();
			let ( solves, steps ) = thread_stats;
			steptotal += steps;
			solvetotal += solves;
		}
	} else {
		// solve the puzzle
		let ( sol, stepcounter ) = solve( board );
		steptotal += stepcounter;
		solution = sol;
		solvetotal += 1;		
	}

	// Calculate elapsed time
	let end = SystemTime::now();
	let elapsed = end.duration_since( start ).unwrap_or_default().as_millis();

	// post-solve output
	if benchmark > 1 {
		if verbose {
			println!( "Solved {} times\nTotal time: {} ms\nTotal steps: {}", solvetotal, elapsed, steptotal );
			let avg_ms = elapsed as f64 / benchmark as f64;
			let avg_steps = steptotal as f64 / benchmark as f64;
			println!( "Average time: {:.2} ms\nAverage steps: {:.2}", avg_ms, avg_steps );
		} else {
			println!( "{}", elapsed );
		}
	} else {
		if verbose {
			println!("\nSolution:\n");
		}
		render( solution, verbose );
		if verbose {
			println!( "\nSolved in {} ms ({} steps).\n", elapsed, steptotal );
		}		
	}
}


fn render( board:Board, verbose:bool ){
	// if non-verbose output is requested, render the same format as the input file
	if !verbose {
		for y in 0..board.squares.len() {
			let mut output = String::from("");
			for x in 0..9 {
				output.push_str( &board.squares[y][x].to_string() );
			}
			println!( "{}", output );
		}
	} else {
		for i in 0..board.squares.len() {
			println!("  {:?}", board.squares[i]);
		}
	}
}


fn solve( input:Board ) -> ( Board, u32 ) {
	// create a mutable copy of the input board
	let mut board:Board = input.clone();
	// create another copy to store partial solves
	let mut board_solved:Board = input.clone();
	// some things to keep track of
	let mut solved:bool = false;
	let mut mode:u8 = 1;
	let mut stepcounter:u32 = 0;
	// call the step function until the board is solved
	while solved == false {
		solved = step( &mut board, &mut board_solved, &mut mode );
		stepcounter += 1;
	}
	// return the solution, steps required as tuple
	( board, stepcounter )
}


fn step( board: &mut Board, board_solved: &mut Board, mode: &mut u8 ) -> bool {
	// find empty positions on the board
	let empty = get_empty_positions( *board );
	// if there are none, the board is solved
	if empty.len() < 1 {
		return true;
	}
	// find a position with the lowest number of possible values
	let ( pos, possible_values ) = get_best_empty_position( &empty, &board );
	// if a position has 0 possible values, the current attempt is invalid
	if possible_values.len() < 1 {
		// return board to previously know correct state
		*board = board_solved.clone();
		return false;
	}
	// mode 1 is known to be correct, mode 2 contains guesses
	if *mode == 1 {
		if possible_values.len() == 1 {
			board.squares[ pos.y as usize ][ pos.x as usize ] = possible_values[0];
			board_solved.squares[ pos.y as usize ][ pos.x as usize ] = possible_values[0];
		} else {
			*mode = 2;
		}
	} else {
		let guess = select_random_value( &possible_values );
		board.squares[ pos.y as usize ][ pos.x as usize ] = guess;
	}
	// return false to indicate the board has not been solved in this step
	return false;
}



fn get_empty_positions( board:Board ) -> Vec<Position> {
	// create a vector to store the empty positions
	let mut empty = Vec::<Position>::with_capacity(81);
	// loop through the board to find positions with value 0
	for y in 0..board.squares.len() {
		for x in 0..board.squares[y].len() {
			if board.squares[y][x] == 0 {
				empty.push( Position { x:x as i8, y:y as i8 } );
			}
		}
	}
	// randomize before returning the vector, to make guesses more random
	let mut rng = thread_rng();
	empty.shuffle(&mut rng);
	// return the vector containing all empty positions
	return empty;
}


fn get_best_empty_position( empty:&Vec<Position>, board:&Board ) -> ( Position, Vec<i8> ) {
	// create variables to hold the results
	let mut lowest = 10;
	let mut position = Position { x:-1, y:-1 };
	let mut values:Vec<i8> = Vec::with_capacity(81);
	// loop through the empty positions to find the one with the fewest possible values
	for pos in empty.iter() {
		// get possible values for the current position
		let possible_values:Vec<i8> = get_possible_values( *board, *pos );
		// if it is the new champoin, record its values
		if possible_values.len() < lowest {
			position = *pos;
			values = Vec::from( possible_values );
			lowest = values.len();
		}
	}

	( position, values )
}


fn get_possible_values( board:Board, pos:Position ) -> Vec<i8> {
	// create a vector to hold the values that found in a row, column or square
	let mut excluded_values: Vec<i8> = Vec::with_capacity(9);
	// check row
	for x in 0..9 {
		if board.squares[ pos.y as usize ][ x as usize ] != 0 {
			let val = board.squares[ pos.y as usize ][ x as usize ];
			if !excluded_values.contains( &val ) {
				excluded_values.push( val );
			}
		}
	}
	//check column
	for y in 0..9 {
		if board.squares[ y as usize ][ pos.x as usize ] != 0 {
			let val = board.squares[ y as usize ][ pos.x as usize ];
			if !excluded_values.contains( &val ) {
				excluded_values.push( val );
			}
		}
	}
	// check square
	let startx:i8 = pos.x - ( pos.x % 3 );
	let starty:i8 = pos.y - ( pos.y % 3 );
	for y in starty..(starty+3) {
		for x in startx..(startx+3) {
			if board.squares[ y as usize ][ x as usize ] != 0 {
				let val = board.squares[ y as usize ][ x as usize ];
				if !excluded_values.contains( &val ) {
					excluded_values.push( val );
				}
			}
		}
	}
	// create a new vector with all non-excluded values
	let mut result: Vec<i8> = Vec::with_capacity(9);
	for i in 1..10 {
		if !excluded_values.contains( &i ) {
			result.push( i );
		}
	}
	// return vector with values that have not been excluded
	return result;
}



fn select_random_value( values:&Vec<i8> ) -> i8 {
	let mut rng = rand::thread_rng();
	let random_nr = rng.gen_range( 0..values.len() );
	return values[ random_nr ];
}