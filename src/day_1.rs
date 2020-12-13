use helpers::read_lines;

#[allow(dead_code)]
pub fn problem_1() {
    let mut expenses : Vec<i32> = vec![];
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(expense_str) = line {
                expenses.push(expense_str.parse::<i32>().unwrap());
            }
        }
    }

    for lo_index in 0..expenses.len() {
        let lo_element = &expenses[lo_index];

        for hi_index in (lo_index+1)..expenses.len() {
            let hi_element = &expenses[hi_index];

            if (lo_element + hi_element) == 2020 {
                println!("Elements: {} + {} = 2020 / {}", lo_element, hi_element, lo_element*hi_element);
                return;
            }
        }
    }
}

#[allow(dead_code)]
pub fn problem_2() {
    let mut expenses : Vec<i32> = vec![];
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(expense_str) = line {
                expenses.push(expense_str.parse::<i32>().unwrap());
            }
        }
    }

    for lo_index in 0..expenses.len() {
        let lo_element = &expenses[lo_index];

        for mid_index in (lo_index+1)..expenses.len() {
            let mid_element = &expenses[mid_index];

            for hi_index in (mid_index+1)..expenses.len() {
                let hi_element = &expenses[hi_index];

                if (lo_element + mid_element + hi_element) == 2020 {
                    println!("Elements: {} + {} + {} = 2020 / {}",
                             lo_element, mid_element, hi_element,
                             lo_element * mid_element * hi_element);
                    return;
                }
            }
        }
    }
}


fn main() {
    problem_2();
}