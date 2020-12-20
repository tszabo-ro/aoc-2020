use helpers::read_lines;

fn load_instructions() -> Vec<(String, i32)> {
    let mut instructions: Vec<(String, i32)> = vec![];
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        for line in lines {
            if let Ok(data) = line {
                let inst_val_str: Vec<&str> = data.split(" ").collect();
                let inst = inst_val_str[0];
                let val = inst_val_str[1].parse::<i32>().unwrap();

                instructions.push((inst.to_string(), val));
            }
        }
    }
    return instructions;
}


fn run_program(instructions_in: &Vec<(String, i32)>) -> (i32, bool) {
    let mut instructions = instructions_in.to_vec();

    let mut accumulator = 0;
    let mut ptr: i32 = 0;

    loop {
        let old_ptr = ptr;
        if ptr < 0 || ptr >= instructions.len() as i32 {
            return (accumulator, true);
        }

        let (inst, val) = &instructions[ptr as usize];
        match &inst[..] {
            "acc" => { accumulator += val; ptr += 1; }
            "jmp" => { ptr += val; }
            "nop" => { ptr += 1; }
            "moot" => {
                return (accumulator, false);
            }
            _ => {panic!("Unknown instruction!"); }
        }

        // println!("Instruction {} - value {} / ptr {} -> acc: {}", inst, val, ptr, accumulator);
        instructions[old_ptr as usize] = ("moot".to_string(), 0);
    }
}

#[allow(dead_code)]
fn problem_1() {
    let instructions = load_instructions();
    let (accumulator, finished) = run_program(&instructions);
    println!("The program finished: {}. The accumulator is {}.", finished, accumulator);
}

fn find_next_jmp_or_nop(instructions_in: &Vec<(String, i32)>, current_ptr: i32) -> i32 {
    let mut ptr = current_ptr + 1;
    loop {
        let (inst, _val) = &instructions_in[ptr as usize];
        match &inst[..] {
            "nop" => { return ptr; }
            "jmp" => { return ptr; }
            _ => { ptr += 1; }
        }
    }
}

fn swap_instruction(current_inst: &(String, i32)) -> (String, i32) {
    let (instr, val) = current_inst;

    match &instr[..] {
        "nop" => { return ("jmp".to_string(), *val); }
        "jmp" => { return ("nop".to_string(), *val); }
        _ => { panic!("invalid instruction: {}", instr); }
    }
}

#[allow(dead_code)]
fn problem_2() {
    let orig_instructions = load_instructions();

    let mut error_ptr: i32 = find_next_jmp_or_nop(&orig_instructions, 0);
    loop {
        let mut instructions = orig_instructions.to_vec();
        println!("Swapping instruction at {}", error_ptr);
        instructions[error_ptr as usize] = swap_instruction(&instructions[error_ptr as usize]);

        let (acc, finished) = run_program(&instructions);
        match finished {
            true => {
                println!("The program finished. Swapped position {}. Accumulator: {}", error_ptr, acc);
                return;
            }
            false => {
                error_ptr = find_next_jmp_or_nop(&orig_instructions, error_ptr);
            }
        }

    }
}

fn main() {
    problem_2();
}