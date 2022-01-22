use rand::Rng;


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
	//println!("Hello, sudoku!");
	let board = Board {
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
	solve( board );
}


fn solve( input:Board ) {
	let mut board:Board = input.clone();
	let mut board_solved:Board = input.clone();
	let mut solved:bool = false;
	let mut mode:u8 = 1;

	while solved == false {
	//for _z in 0..50000 {
		solved = step( &mut board, &mut board_solved, &mut mode );
		if solved {
			break;
		}
	}

	render( board );
}


fn step( board: &mut Board, board_solved: &mut Board, mode: &mut u8 ) -> bool {

	// find empty positions on the board
	let empty = get_empty_positions( *board );

	// if there are none, the board is solved
	if empty.len() < 1 {
		return true;
	}

	// loop through the empty positions, to find the number of possible answers
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

	if best_empty_position_values.len() < 1 {
		// return board to previously know correct state
		*board = board_solved.clone();
	} else {
		if best_empty_position_values.len() > 1 {
			*mode = 2;
		}
		let val = select_random_value( &best_empty_position_values );
		board.squares[ best_empty_position.y as usize ][ best_empty_position.x as usize ] = val;
		if *mode == 1 {
			// if there's only one option (mode 1), store the guess
			*board_solved = board.clone();
		}
	}

	return false;
}


fn render( board:Board ){
	for i in 0..board.squares.len() {
		println!("{:?}", board.squares[i]);
	}
}


fn get_empty_positions( board:Board ) -> Vec<Position> {
	let mut empty = Vec::<Position>::with_capacity(81);
	for y in 0..board.squares.len() {
		for x in 0..board.squares[y].len() {
			if board.squares[y][x] == 0 {
				empty.push( Position { x:x as i8, y:y as i8 } );
			}
		}
	}
	return empty;
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