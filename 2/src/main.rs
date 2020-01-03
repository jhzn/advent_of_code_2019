fn main() {
	//TODO make program accept from stdin intead of hardcoding
	let inital_memory_as_string = "1,0,0,0,99";
	let split: Vec<&str> = inital_memory_as_string.split(",").collect();
	let initial_memory: Vec<i64> = split
		.iter()
		.map(|opt_code| {
			opt_code
				.parse::<i64>()
				.expect("invalid inital memory received")
		})
		.collect();

	let output = run_program(initial_memory.clone());
	println!("Output = {:?}", output);

	part1();
	part2();
}

fn part1() {
	let initial_memory = vec![
		1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27,
		1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51,
		9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5,
		79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103,
		1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
	];
	let output = run_program(initial_memory.clone());
	println!("Part 1 output = {:?}", output[0]);
}

pub fn part2() {
	let initial_memory = vec![
		1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27,
		1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51,
		9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5,
		79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103,
		1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
	];
	//brute force answer, noun and verb are words used in the challenge
	for noun in 0..=99 {
		for verb in 0..=99 {
			let mut program_memory = initial_memory.clone();
			program_memory[1] = noun;
			program_memory[2] = verb;
			let memory_after = run_program(program_memory);
			if memory_after[0] == 19690720 {
				println!(
					"Part 2 output! Noun = {:?} Verb = {:?} and final answer = {:?}",
					noun,
					verb,
					100 * noun + verb
				);
			}
		}
	}
}

pub fn run_program(initial_memory: Vec<i64>) -> Vec<i64> {
	let mut program_memory = initial_memory.clone();
	let mut instruction_pointer = 0;
	loop {
		//match for op code
		match program_memory[instruction_pointer] {
			//addition, 3 memory addresses are needed, A and B as input
			//C to store the output of the computation
			1 => {
				let instruction_param_a_address = program_memory[instruction_pointer + 1];
				let instruction_param_b_address = program_memory[instruction_pointer + 2];

				let instruction_param_a = program_memory[instruction_param_a_address as usize];
				let instruction_param_b = program_memory[instruction_param_b_address as usize];

				let output_address = program_memory[instruction_pointer + 3];
				program_memory[output_address as usize] = instruction_param_a + instruction_param_b;
				instruction_pointer += 4;
			}
			//multiplication, 3 memory addresses are needed, A and B as input
			//C to store the output of the computation
			2 => {
				let instruction_param_a_address = program_memory[instruction_pointer + 1];
				let instruction_param_b_address = program_memory[instruction_pointer + 2];

				let instruction_param_a = program_memory[instruction_param_a_address as usize];
				let instruction_param_b = program_memory[instruction_param_b_address as usize];

				let output_address = program_memory[instruction_pointer + 3];
				program_memory[output_address as usize] = instruction_param_a * instruction_param_b;
				instruction_pointer += 4;
			}
			//program termination
			99 => {
				break; //end main loop
			}
			_ => panic!("invalid op code {:?}", program_memory[instruction_pointer]),
		}
	}
	program_memory
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	//test examples given in challenge and validate the correctness of the function
	fn it_works() {
		assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
		assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
		assert_eq!(
			run_program(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
			vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		assert_eq!(
			run_program(vec![2, 4, 4, 5, 99, 0]),
			vec![2, 4, 4, 5, 99, 9801]
		);
		assert_eq!(
			run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
			vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
		);
		assert_eq!(
			run_program(vec![
				1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13,
				23, 27, 1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1,
				47, 6, 51, 2, 51, 9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71,
				2, 6, 71, 75, 1, 75, 5, 79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91,
				95, 2, 13, 95, 99, 1, 99, 10, 103, 1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0
			]),
			vec![
				2842648, 12, 2, 2, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 48, 1, 19, 6, 50,
				2, 13, 23, 250, 1, 27, 13, 255, 1, 9, 31, 258, 1, 35, 9, 261, 1, 39, 5, 262, 2, 6,
				43, 524, 1, 47, 6, 526, 2, 51, 9, 1578, 2, 55, 13, 7890, 1, 59, 6, 7892, 1, 10, 63,
				7896, 2, 67, 9, 23688, 2, 6, 71, 47376, 1, 75, 5, 47377, 2, 79, 10, 189508, 1, 5,
				83, 189509, 2, 9, 87, 568527, 1, 5, 91, 568528, 2, 13, 95, 2842640, 1, 99, 10,
				2842644, 1, 103, 2, 2842646, 1, 107, 6, 0, 99, 2, 14, 0, 0
			]
		);
	}
}
