use std::{marker::PhantomData, u8, collections::VecDeque, io::{self, ErrorKind}, iter::{Chain, empty, Empty}, time::SystemTime, sync::{Mutex, mpsc::{self, TryRecvError}}, clone, borrow::Borrow, thread, };

use rayon::{prelude::{IntoParallelIterator, ParallelIterator, FromParallelIterator, IntoParallelRefIterator}, collections::linked_list, iter::Either};

use fun_tools::extensions::UsadoEm;

use memory_stats::memory_stats;

type BoardSize = u8;

#[derive( Debug, PartialEq)]
pub struct QueensBoard<const board_size: BoardSize>{
	queens : VecDeque<Queen, >
}


#[derive( Clone, Debug, PartialEq )]
pub struct Queen {
	pub row: BoardSize,
	pub col: BoardSize,
}

impl Queen{
	pub fn can_attack(&self, other: &Queen) -> bool {
		fn abs_dist(lhs: BoardSize, rhs: BoardSize) -> BoardSize{
			if  	lhs >= rhs {lhs - rhs}
			else 	{rhs - lhs}
		}

		( 	(self.row == other.row)  
		|| 	(self.col == other.col)
		||	(abs_dist(self.col, other.col) == abs_dist(self.row, other.row))
		)
	}
}

impl<const board_size: BoardSize> QueensBoard<board_size> {
	pub fn new() -> Self{
		Self { queens: VecDeque::<Queen>::with_capacity(board_size as usize) }
	}

	pub fn draw(&self) -> String{
		let board_size_usize = board_size as usize;
		let tile_chars = ['░', '█', 'Q'];

		let mut table_chars = Vec::<char>::new();
		
		table_chars.resize(board_size_usize*board_size_usize*2, ' ');

		let get_table_chars_index = |col: usize, row: usize| -> usize { 
			(col * board_size_usize + row) 
		};

		for col in (0..board_size_usize){ 
			for row in (0..board_size_usize){
				table_chars[get_table_chars_index(col, row)] = tile_chars[(col + row) % 2]; 
			}
		}

		for queen in &self.queens{
			table_chars[get_table_chars_index(queen.col as usize, queen.row as usize)] = tile_chars[2];
		} 
		
		let mut res = String::with_capacity(board_size_usize * board_size_usize * 2 + board_size_usize + 1);
		
		for col in (0..board_size_usize){ 
			for row in (0..board_size_usize){
				res.push(table_chars[get_table_chars_index(col, row)]);
				res.push(table_chars[get_table_chars_index(col, row)]);
			}
			res.push('\n')
		}
		res
	}

	pub fn get_queens(&self) -> &VecDeque<Queen>{
		&self.queens
	}

	pub fn push_queen_unchecked(&mut self, new_queen: Queen) {
		self.queens.push_back(new_queen);
	}

	pub fn try_push_queen(&mut self, new_queen: Queen) -> Result<(), io::Error>{
		if !self.is_full() {
			self.queens.push_back(new_queen);
			Ok(())
		} else {
			Err(io::Error::from(ErrorKind::OutOfMemory))
		}
	}
	pub fn is_full(&self) -> bool {
		self.queens.len() >= board_size as usize
	}

	pub fn pop_queen(&mut self) -> Option<Queen> {
		self.queens.pop_back()
	}
}

impl<const board_size: BoardSize> Clone for QueensBoard<board_size>{
    fn clone(&self) -> Self {
        Self { queens: self.queens.clone() }
    }
}

pub fn try_solve_low_mem_serial_proc<const board_size: BoardSize>(queens_board: &mut QueensBoard<board_size>) -> (u128, Vec<QueensBoard<board_size>>){
	let mut res = Vec::new();
	let mut nodes = 0;
	
	if queens_board.is_full(){
		res.push(queens_board.clone());
	} else {
		let mut new_queen  = { 
			let col = queens_board.get_queens().len() as u8;
			let row = 0;
			Queen{col, row}
		};
		
		for row in (0..board_size) {
			new_queen.row = row;
			nodes += 1;
			if (!queens_board.get_queens().into_iter().any(|queen| queen.can_attack(&new_queen))){

				queens_board.push_queen_unchecked(new_queen);
				let (other_nodes, mut reses) = try_solve_low_mem_serial_proc(queens_board);
				nodes += other_nodes;
				res.append(&mut reses);
				new_queen = queens_board.pop_queen().unwrap();
			}
		}
	}
	(nodes, res)
}
pub fn try_solve_high_mem_par_proc<const board_size: BoardSize>() -> Vec<QueensBoard<board_size>>{

	#[derive(Clone)]
	struct LinkedQueen<'a> {
		queen: 		Queen,
		prev_queen: Option<&'a LinkedQueen<'a>>
	}

	
	fn linked_to_board<const board_size: BoardSize> (linked_queen: &LinkedQueen) -> QueensBoard<board_size> {
		let mut board = QueensBoard::<board_size>::new();
		let mut current_queen = Some(linked_queen);
		while let Some(queen_link) = current_queen{
			board.try_push_queen(queen_link.queen.clone());
			current_queen = queen_link.prev_queen.as_deref().map(|x| {x.borrow()});
		}
		board
	}
	
	fn worker<const board_size: BoardSize> (linked_queen: Option<&LinkedQueen>, level: BoardSize, sender: mpsc::SyncSender<QueensBoard<board_size>>){
			(0..board_size)
			.into_par_iter()
			.for_each(|row| {
				let new_queen = { 
					let col = level;
					let row = row;
					Queen{col, row}
				};

				let is_attacked  = {
					let mut is_attacked = false;
					let mut current_option_queen: Option<&LinkedQueen> = linked_queen;

					while let Some(current_queen) = current_option_queen {
						is_attacked = is_attacked | current_queen.queen.can_attack(&new_queen);
						if is_attacked{ break;}
						else{
							current_option_queen = current_queen.prev_queen;
						}
					}
					is_attacked
				};


				if !is_attacked {
					let next_level = level + 1;
					let next_linked_queen = LinkedQueen {
						queen:  	new_queen,
						prev_queen: linked_queen
					};

					if next_level == board_size{
						use mpsc::TrySendError::*;
						let mut board = linked_to_board::<board_size>(&next_linked_queen); 
						while let Err(Full(gb_board)) = sender.try_send(board) {
							board = gb_board;
							println!("writer is full!");
						}
					} else {
						worker::<board_size>(Some(&next_linked_queen), next_level, sender.clone());
					}
				}
				
			})	;
	}
	
	let (sender, receiver) = mpsc::sync_channel(1 << ((board_size as usize) - 1));
	
	let consumer_thread = thread::spawn(move || {
		let mut results = Vec::new();
		loop {
			match receiver.try_recv() {
				Ok(res) => {results.push(res)},
				Err(TryRecvError::Empty) => {()},
				Err(TryRecvError::Disconnected) => {break;},
			}
		}
		results
	});

	worker::<board_size>(None, 0, sender);
	consumer_thread
		.join()
		.expect("eita")
}

	
pub fn main() -> (){
	const board_size : u8 = 14;
	
	println!("starting serial...");
	{
		let mut board = QueensBoard::<board_size>::new();
		let timer = SystemTime::now();
		let (node_count, res) = try_solve_low_mem_serial_proc(&mut board.clone());
		let time_it_took = timer.elapsed().unwrap();
		memory_stats().map(|mem_stats| {println!("physical memory used: {} bytes; virtual mem used: {}", mem_stats.physical_mem, mem_stats.virtual_mem);});


	 	println!("Found {} results using low mem.\nSearched through {} nodes.\nTook {}ms.", res.len(), node_count, time_it_took.as_millis());
	 	for sol_drawing in res.into_iter().map(|sol| sol.draw()).take(6) {
	 		println!("{}", sol_drawing);
	 	}
	}
	println!("starting parallel...");
	{
		let timer = SystemTime::now();
		rayon::ThreadPoolBuilder::new().num_threads(15).build_global().unwrap();
		let res = try_solve_high_mem_par_proc::<board_size>();
		let time_it_took = timer.elapsed().unwrap();
		memory_stats().map(|mem_stats| {println!("physical memory used: {} bytes; virtual mem used: {}", mem_stats.physical_mem, mem_stats.virtual_mem);});
	
		println!("Found {} results using parallel. Took {}ms", res.len(), time_it_took.as_millis());
		for sol_drawing in res.into_iter().map(|sol| sol.draw()).take(6) {
			println!("{}", sol_drawing);
		}
	}
}