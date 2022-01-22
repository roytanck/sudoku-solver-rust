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
	println!("Hello, sudoku!");
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

	//render( board_solved );

	//while solved == 0 {
	for _z in 0..50000 {
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
	//let nrof_empty = count_empty_positions( empty );
	
	//let nrof_empty = empty.len();

	if empty.len() < 1 {
		println!("success");
		return true;
	}
	//println!( "{:?} empty squares", empty.len() );

	// loop through the empty positions, to find the number of possible answers
	let mut i = 0;
	let mut lowest = 10;
	let mut best_empty_position = Position { x:-1, y:-1 };
	let mut best_empty_position_values:[i8; 9] = [-1; 9];
	let mut best_empty_position_nrof_values:u8 = 10;
	for pos in empty.iter() {
		//println!( "{:?}", empty[i] );
		let possible_values = get_possible_values( *board, *pos );
		//println!( "{:?}", possible_values );
		let nrof_values = count_possible_values( possible_values );

		//println!( "{:?}", nrof_values );
		if nrof_values < lowest {
			best_empty_position = *pos;
			best_empty_position_values = possible_values;
			best_empty_position_nrof_values = nrof_values;
			lowest = nrof_values;
		}
		//i += 1;
	}
	//println!( "{:?}", best_empty_position );

	if best_empty_position_nrof_values < 1 {
		println!("Oops");
		*board = board_solved.clone();
	} else {

		if best_empty_position_nrof_values > 1 {
			*mode = 2;
		}
		let val = select_random_value( best_empty_position_values );
		board.squares[ best_empty_position.y as usize ][ best_empty_position.x as usize ] = val;
		if *mode == 1 {
			// if there's only one option (mode 1), store the guess
			*board_solved = board.clone();
		}
		//println!("{:?} at {:?}", val, best_empty_position);
	}

	//render( board );
	return false;
}


fn render( board:Board ){
	//println!("{:?}", board);
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


fn get_possible_values( board:Board, pos:Position ) -> [i8; 9] {
	let mut values: [i8; 9] = [1,2,3,4,5,6,7,8,9];
	// check row
	for x in 0..9 {
		if board.squares[ pos.y as usize ][ x as usize ] != 0 {
			let val = board.squares[ pos.y as usize ][ x as usize ];
			values[ val as usize - 1 ] = -1;
		}
	}
	//check column
	for y in 0..9 {
		if board.squares[ y as usize ][ pos.x as usize ] != 0 {
			let val = board.squares[ y as usize ][ pos.x as usize ];
			values[ val as usize - 1 ] = -1;
		}
	}
	// check square
	let startx:i8 = pos.x - ( pos.x % 3 );
	let starty:i8 = pos.y - ( pos.y % 3 );
	for y in starty..(starty+3) {
		for x in startx..(startx+3) {
			if board.squares[ y as usize ][ x as usize ] != 0 {
				let val = board.squares[ y as usize ][ x as usize ];
				values[ val as usize - 1 ] = -1;
			}
		}
	}
	return values;
}


fn count_possible_values( values:[i8; 9] ) -> u8 {
	let mut counter:u8 = 0;
	for i in 0..values.len() {
		if values[i] != -1 {
			counter += 1;
		}
	}
	return counter;
}


fn select_random_value( values:[i8; 9] ) -> i8 {
	let mut counter:u8 = 0;
	let max = count_possible_values( values );
	let mut rng = rand::thread_rng();
	let random_nr = rng.gen_range(0..max);
	for i in 0..values.len() {
		if values[i] != -1 {
			if counter == random_nr {
				return values[i];
			}
			counter += 1;
		}
	}
	return 1;
}