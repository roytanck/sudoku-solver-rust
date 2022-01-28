use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use clap::{Arg,App};



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

	// default puzzle
	let mut board = Board {
		squares: [
			[2,3,5,0,0,0,0,7,0],
			[0,0,8,0,0,0,0,0,0],
			[0,0,0,0,2,3,0,4,0],
			[8,6,4,0,0,0,0,0,0],
			[0,0,7,0,0,6,0,8,5],
			[0,0,0,0,7,2,0,0,0],
			[0,5,0,0,6,7,0,1,8],
			[0,0,1,0,0,0,0,0,0],
			[9,0,0,1,0,0,0,2,3],
		]
	};

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
	]).get_matches();

	let filename = args.value_of("input").unwrap_or("extreme.txt");
	let verbose:bool = args.is_present("verbose");
	let benchmark:u32 = args.value_of("benchmark").unwrap_or( "0" ).parse::<u32>().unwrap();

	// read the input file
	let file = File::open( filename ).expect("file not found!");
	let reader = BufReader::new(file);
	// iterate over the loaded file by line
	for ( y, line ) in reader.lines().enumerate() {
		let row = line.unwrap();
		if y < 9 {
			// iterate ovet the characters of the current line
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

	if verbose && benchmark <= 1 {
		println!("\nInput:\n");
		render( board, verbose );
	}

	let mut steptotal:u32 = 0;
	let start = SystemTime::now();

	// if benchmark is set, run that number (minus one) extra solves
	if benchmark > 1 {
		for _run in 1..benchmark {
			let ( _solution, stepcounter ) = solve( board );
			steptotal += stepcounter;
		}
	}

	let ( solution, stepcounter ) = solve( board );
	steptotal += stepcounter;

	let end = SystemTime::now();
	let elapsed = end.duration_since( start ).unwrap_or_default().as_millis();

	if benchmark > 1 {
		if verbose {
			println!( "Solved {} times\nTotal time: {} ms\nTotal steps: {}", benchmark, elapsed, steptotal );
			let avg_ms = ( elapsed as f64 ) / ( benchmark as f64 );
			let avg_steps = ( steptotal as f64 ) / ( benchmark as f64 );
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
			println!( "\nSolved in {} ms ({} steps).\n", elapsed, stepcounter );
		}		
	}
}


fn render( board:Board, verbose:bool ){
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

	while solved == false {
		solved = step( &mut board, &mut board_solved, &mut mode );
		stepcounter += 1;
	}

	// return the solution, steps required and milliseconds as tuple
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

	if possible_values.len() < 1 {
		// return board to previously know correct state
		*board = board_solved.clone();
		return false;
	}

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

	return false;
}



fn get_empty_positions( board:Board ) -> Vec<Position> {
	// loop through the empty positions, to find the number of possible answers
	let mut empty = Vec::<Position>::with_capacity(81);
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

	return empty;
}


fn get_best_empty_position( empty:&Vec<Position>, board:&Board ) -> ( Position, Vec<i8> ) {
	let mut lowest = 10;
	let mut best_empty_position = Position { x:-1, y:-1 };
	let mut best_empty_position_values:Vec<i8> = Vec::with_capacity(81);
	
	for pos in empty.iter() {
		let possible_values:Vec<i8> = get_possible_values( *board, *pos );

		if possible_values.len() < lowest {
			best_empty_position = *pos;
			best_empty_position_values = Vec::from( possible_values );
			lowest = best_empty_position_values.len();
		}
	}

	( best_empty_position, best_empty_position_values )
}


fn get_possible_values( board:Board, pos:Position ) -> Vec<i8> {

	let mut excluded_values: Vec<i8> = Vec::with_capacity(9);

	// check row
	for x in 0..9 {
		if board.squares[ pos.y as usize ][ x as usize ] != 0 {
			let val = board.squares[ pos.y as usize ][ x as usize ];
			//values[ val as usize - 1 ] = -1;
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

	return result;
}



fn select_random_value( values:&Vec<i8> ) -> i8 {
	let mut rng = rand::thread_rng();
	let random_nr = rng.gen_range( 0..values.len() );
	return values[ random_nr ];
}