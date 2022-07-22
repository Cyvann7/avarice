use std::{fs::File, io::Read, fmt, collections::{VecDeque, HashSet}};
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
//? \ ============= CONFIGURATION ============ //

//?  Enables Verbose logging and output. This is defualt value - can be changed by --v or --verbose
static VERBOSE: bool = false;

//?  File path to interpret.
static PATH: &'static str = "./source.avrc";

//?  Maximum number of operations.
static MAX: u128 = 65536;

//? \ ======================================= //



#[derive(Debug)]
enum State {
	Normal,
	Number,
	Arithmetic,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Operation {
	op: char,
	pos: (i32,i32)
}

impl fmt::Debug for Operation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "OP('{}' at {:?})", self.op, self.pos)
	}
}



fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut verbose = VERBOSE;
	for a in args {
		if a == "--v" || a == "--verbose" { verbose = true }
	}

	let program = source_path_to_grid(PATH);
	
	println!("{}{}|Program|{}", "\n".repeat(2), "=".repeat(15),"=".repeat(15));
	for line in program.iter() {
		println!("{}", line.iter().collect::<String>());
	}
	println!("{}{}|Running|{}", "\n".repeat(2), "=".repeat(15),"=".repeat(15));
	
	let mut output_buffer = String::new();
	let res = interpret_program(program, &mut output_buffer, verbose);
	
	println!("{}{}|Outputs|{}", "\n".repeat(2), "=".repeat(15),"=".repeat(15));
	println!("{}", output_buffer);
	let mut colour = String::from("\x1b[1;33m");
	if res == 1 { colour = String::from("\x1b[1;32m"); }
	if res == 2 { colour = String::from("\x1b[1;31m"); }
	println!("{}{}{}|Program Exited With Code {:02}|{}{}", 
					"\n".repeat(2), 
					"=".repeat(5), 
					colour,
					res,
					"\x1b[0m",
					"=".repeat(5)
	);
	

	
}

fn source_path_to_grid(src_path: &str) -> Vec<Vec<char>>{
	let mut f = if let Ok(f) = File::open(src_path) {f
	} else { panic!("\x1b[0;91m SOURCE PATH [{}] NOT FOUND \x1b[0m", src_path); }; //prints in red
	let mut contents = String::new();
	f.read_to_string(&mut contents).ok();
	let linebychars = contents.lines().map(|c| c.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
	return linebychars;
} 

fn interpret_program(source_code: Vec<Vec<char>>, output_buffer: &mut String, verbose: bool) -> isize{

	let first_op = Operation {
		op: source_code[0][0],
		pos: (0,0),
	};

	let mut opstack: VecDeque<Operation> = VecDeque::new();
	opstack.push_back(first_op);

	let mut numstack: Vec<char> = Vec::new();
	let mut current_values: Vec<u32> = Vec::new();

	let mut program_state = State::Normal;
	let mut c = 0;

	let mut visited: HashSet<Operation> = HashSet::new();

	// macro to push all the neighbour characters and their positions into the stack
	macro_rules ! push_neighbours {
		($y_pos: expr, $x_pos: expr) => {
			for i in  -1..=1_i32 {
				for j in  -1..=1_i32 {

					//stop when diagonal or on point
					if i == -1  || i == 1 {
						if j == -1 || j == 1 { continue;}
					}
					if j == -1  || j == 1 {
						if i == -1 || i == 1 { continue;}
					}
					if i == j { continue;}

					let possible_char = source_code
					.get(($y_pos + i ) as usize)
					.and_then(|a| { a.get(($x_pos + j ) as usize) } );
					if (verbose) {
						println!("({},{}) -> {:?}",$y_pos + i,$x_pos + j, possible_char);
					}
					if let Some(c) = possible_char {
						let o = Operation {
							op: *c,
							pos: ($y_pos + i, $x_pos + j)
						};
						opstack.push_back(o);
					}
				}
			}
		};
	}
	macro_rules ! push_neighbours_horizontal {
		($y_pos: expr, $x_pos: expr) => {
			for i in 0..1 {
				for j in  -1..=1_i32 {

					//stop when diagonal or on point
					if i == -1  || i == 1 {
						if j == -1 || j == 1 { continue;}
					}
					if j == -1  || j == 1 {
						if i == -1 || i == 1 { continue;}
					}
					if i == j { continue;}

					let possible_char = source_code
					.get(($y_pos + i ) as usize)
					.and_then(|a| { a.get(($x_pos + j ) as usize) } );
					if verbose {
						println!("({},{}) -> {:?}",$y_pos + i,$x_pos + j, possible_char);
					}
						if let Some(c) = possible_char {
						let o = Operation {
							op: *c,
							pos: ($y_pos + i, $x_pos + j)
						};
						opstack.push_back(o);
					}
				}
			}
		};
	}
	macro_rules ! push_neighbours_vertical {
		($y_pos: expr, $x_pos: expr) => {
			for i in  -1..=1_i32{
				for j in  0..1 {

					//stop when diagonal or on point
					if i == -1  || i == 1 {
						if j == -1 || j == 1 { continue;}
					}
					if j == -1  || j == 1 {
						if i == -1 || i == 1 { continue;}
					}
					if i == j { continue;}

					let possible_char = source_code
					.get(($y_pos + i ) as usize)
					.and_then(|a| { a.get(($x_pos + j ) as usize) } );
					if verbose {
						println!("({},{}) -> {:?}",$y_pos + i,$x_pos + j, possible_char);
					}
					if let Some(c) = possible_char {
						let o = Operation {
							op: *c,
							pos: ($y_pos + i, $x_pos + j)
						};
						opstack.push_back(o);
					}
				}
			}
		};
	}
	loop {
		if verbose {
			println!("STATE: {:?}\nOPSTACK: {:#?}\nVISITED: {:#?}\nFORMING-NUMBER: {:#?}\nNUMSTACK: {:#?}\n", program_state, opstack, visited,numstack, current_values);
		}
		c+=1; 
		if c == MAX { return 2 }
		match program_state {
			State::Normal => {
				let opera = opstack.pop_front();
				let opera = if let Some(x) = opera {
					if visited.contains(&x) { continue;} else {x}
				} else { return 0 };
				let op = opera;
				visited.insert(op);
				match op.op {
					'S' | '+' => { 
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'Y' => {
						let num1 = current_values.pop();
						push_neighbours_horizontal!(op.pos.0, op.pos.1);
						if num1 == Some(0) || num1 == None {
							{
								let num2 = op.pos.1;
								let num1 = op.pos.0 + 1;
		
								let possible_char = source_code
								.get(num1 as usize)
								.and_then(|a| { a.get(num2 as usize) } );
		
								if verbose {
									println!("ARROW CALL: ({},{}) -> {:?}",num1,num2, possible_char);
								}
								
								if let Some(c) = possible_char {
									let o = Operation {
										op: *c,
										pos: (num1 as i32, num2 as i32)
									};
									opstack.push_back(o);
								}
							}
						}
					}
					'#' => { 
						push_neighbours!(op.pos.0, op.pos.1);
						program_state = State::Number;
					},
					'i' => { 
						print!("INPUT >>> ");
						stdout().lock().flush().ok();
						current_values.push(get_user_input_as_u32());
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'P' => { 
						output_buffer.push(current_values.pop().expect("P: Number Stack was empty when popped") as u8 as char);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'p' => { 
						output_buffer.push_str(current_values.pop().expect("p: Number Stack was empty when popped").to_string().as_str());
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'|' => { 
						push_neighbours_vertical!(op.pos.0, op.pos.1)
					},

					'-' => { 
						push_neighbours_horizontal!(op.pos.0, op.pos.1)
					},

					'T' => { 
						return 1; 
					},
					'&'=> {
						let num1 = current_values.pop().expect("&1: Number Stack was empty when popped");
						let num2 = current_values.pop().expect("&2: Number Stack was empty when popped");

						let possible_char = source_code
						.get(num1 as usize)
						.and_then(|a| { a.get(num2 as usize) } );

						if verbose {
							println!("SUBROUTINE CALL: ({},{}) -> {:?}",num1,num2, possible_char);
						}
						
						if let Some(c) = possible_char {
							let o = Operation {
								op: *c,
								pos: (num1 as i32, num2 as i32)
							};
							opstack.push_front(o);
						}
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'R' => {
						visited = HashSet::new();
						push_neighbours!(op.pos.0, op.pos.1)
					},
					'>' => {
						let num2 = op.pos.1 + 1;
						let num1 = op.pos.0;

						let possible_char = source_code
						.get(num1 as usize)
						.and_then(|a| { a.get(num2 as usize) } );

						if verbose {
							println!("ARROW CALL: ({},{}) -> {:?}",num1,num2, possible_char);
						}
						
						if let Some(c) = possible_char {
							let o = Operation {
								op: *c,
								pos: (num1 as i32, num2 as i32)
							};
							opstack.push_back(o);
						}
					},
					'^' => {
						let num2 = op.pos.1;
						let num1 = op.pos.0 - 1;

						let possible_char = source_code
						.get(num1 as usize)
						.and_then(|a| { a.get(num2 as usize) } );

						if verbose {
							println!("ARROW CALL: ({},{}) -> {:?}",num1,num2, possible_char);
						}
						
						if let Some(c) = possible_char {
							let o = Operation {
								op: *c,
								pos: (num1 as i32, num2 as i32)
							};
							opstack.push_back(o);
						}
					},
					'v' => {
						let num2 = op.pos.1;
						let num1 = op.pos.0 + 1;

						let possible_char = source_code
						.get(num1 as usize)
						.and_then(|a| { a.get(num2 as usize) } );

						if verbose {
							println!("ARROW CALL: ({},{}) -> {:?}",num1,num2, possible_char);
						}
						
						if let Some(c) = possible_char {
							let o = Operation {
								op: *c,
								pos: (num1 as i32, num2 as i32)
							};
							opstack.push_back(o);
						}
					},
					'<' => {
						let num2 = op.pos.1 - 1;
						let num1 = op.pos.0;

						let possible_char = source_code
						.get(num1 as usize)
						.and_then(|a| { a.get(num2 as usize) } );

						if verbose {
							println!("ARROW CALL: ({},{}) -> {:?}",num1,num2, possible_char);
						}
						
						if let Some(c) = possible_char {
							let o = Operation {
								op: *c,
								pos: (num1 as i32, num2 as i32)
							};
							opstack.push_back(o);
						}
					},
					'M' => {
						program_state = State::Arithmetic;
						push_neighbours!(op.pos.0, op.pos.1);
					}
					_ => {}
				}
			} 
			State::Number => {
				let opera = opstack.pop_front();
				let opera = if let Some(x) = opera {
					if visited.contains(&x) { continue;} else {x}
				} else { return 0 };
				let op = opera;
				visited.insert(op);
				match op.op {
					'D' => {
						let num1 = current_values.pop().expect("D: Number Stack was empty when popped");
						current_values.push(num1);
						current_values.push(num1);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'-' => { 
						push_neighbours_horizontal!(op.pos.0, op.pos.1)
					},
					'|' => { 
						push_neighbours_vertical!(op.pos.0, op.pos.1)
					},

					'+' => { 
						push_neighbours!(op.pos.0, op.pos.1);
					},

					'0'..='9'=> {
						numstack.push(op.op);
						push_neighbours!(op.pos.0, op.pos.1);
					},

					'#' => {
						program_state = State::Normal;
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'&' => {
						if let Ok(x) =  numstack.iter().collect::<String>().parse::<u32>() {
							numstack = Vec::new();
							push_neighbours!(op.pos.0, op.pos.1);
							current_values.push(x);
						} else {
							println!("#[n]# requires n to be a number, recieved [{}]",
											numstack.iter().collect::<String>());
							panic!("See above.")
						};
					},

					'C' => { 
						numstack = Vec::new();
						current_values = Vec::new();
						push_neighbours!(op.pos.0, op.pos.1);
						program_state = State::Normal;
					},
					'w' => {},
					_ => {}
				}
			}
			State::Arithmetic => {
				let opera = opstack.pop_front();
				let opera = if let Some(x) = opera {
					if visited.contains(&x) { continue;} else {x}
				} else { return 0 };
				let op = opera;
				visited.insert(op);
				match op.op {
					'*' => {
						let num1 = current_values.pop().expect("*: Number Stack was empty when popped");
						let num2 = current_values.pop().expect("*: Number Stack was empty when popped");
						current_values.push(num1*num2);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'+' => {
						let num1 = current_values.pop().expect("+: Number Stack was empty when popped");
						let num2 = current_values.pop().expect("+: Number Stack was empty when popped");
						current_values.push(num1+num2);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'-' => {
						let num1 = current_values.pop().expect("-: Number Stack was empty when popped");
						let num2 = current_values.pop().expect("-:Number Stack was empty when popped");
						current_values.push(num2-num1);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'/' => {
						let num1 = current_values.pop().expect("/: Number Stack was empty when popped");
						let num2 = current_values.pop().expect("/: Number Stack was empty when popped");
						current_values.push(num1/num2);
						push_neighbours!(op.pos.0, op.pos.1);
					},
					'M' => {
						program_state = State::Normal;
						push_neighbours!(op.pos.0, op.pos.1);
					},
					_ => {}
				}
			}
		}
	};
}



fn get_user_input_as_u32() -> u32 {
    let mut number:u32 = 0;
    let mut entered = false;
    while !entered {
        let mut input = String::new();
        stdin().read_line(&mut input).ok().expect("Failed to read line");
        let answer = input.split_whitespace().collect::<String>().parse::<u32>();
        match answer {
            Ok(int) => {
                entered = true;
                number = int;
            },
            Err(er) => {
                println!("Error: {}", er); 
                continue;
            }
        }
    }
    return number;
}