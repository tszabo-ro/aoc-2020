use helpers::read_lines;

#[allow(dead_code)]
pub fn problem_1() {
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String

        let mut current_pos = 0;
        let mut tree_count = 0;

        let tree_marker = "#";

        for line in lines {
            if let Ok(data) = line {
                if &data[current_pos..current_pos+1] == tree_marker {
                    tree_count += 1;
                }
                current_pos = (current_pos + 3) % data.len();
            }
        }

        println!("Num trees: {}", tree_count);
    }
}

#[allow(dead_code)]
pub fn problem_2() {

    let steps = vec![
    (1,1),
    (3,1),
    (5,1),
    (7,1),
    (1,2),
    ];

    let mut result : u64 = 1;
    for step in steps {
        if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
            let tree_marker = "#";

            let right_step = step.0 as usize;
            let down_step = step.1 as usize;

            let mut right_pos = 0;
            let mut down_pos = 0;

            let mut tree_count = 0;

            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(data) = line {
                    if down_pos % down_step == 0 {
                        if &data[right_pos..right_pos + 1] == tree_marker {
                            tree_count += 1;
                        }
                        right_pos = (right_pos + right_step) % data.len();
                    }
                    down_pos += 1;
                }
            }
            println!("Step {} / {} -> {} / {}", right_step, down_step, tree_count, result);
            result *= tree_count;
        }
    }
    println!("Result: {}", result);
}


fn main() {
    problem_2();
}