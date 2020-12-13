use helpers::read_lines;

#[allow(dead_code)]
pub fn problem_1() {
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String

        let mut accepted_password = 0;
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split(" ").collect();

                let limits: Vec<&str> = tokens[0].split("-").collect();
                let criteria = &tokens[1][0..tokens[1].len() - 1];
                let password = tokens[2];

                let limit_lo: i32 = limits[0].parse::<i32>().unwrap();
                let limit_hi: i32 = limits[1].parse::<i32>().unwrap();
                let char_count = password.matches(criteria).count();

                if (char_count >= limit_lo as usize) && (char_count <= limit_hi as usize) {
                    accepted_password += 1;
                }
            }
        }

        println!("Accepted passwords: {}", accepted_password);
    }
}

pub fn problem_2() {
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String

        let mut accepted_password = 0;
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split(" ").collect();

                let limits: Vec<&str> = tokens[0].split("-").collect();
                let criteria = &tokens[1][0..tokens[1].len() - 1];
                let password = tokens[2];

                // The provided indices start at 1 instead of 0!
                let limit_lo: usize = limits[0].parse::<usize>().unwrap() - 1;
                let limit_hi: usize = limits[1].parse::<usize>().unwrap() - 1;

                let lo_match = &password[limit_lo..limit_lo + 1] == criteria;
                let hi_match = &password[limit_hi..limit_hi + 1] == criteria;

                if lo_match ^ hi_match {
                    accepted_password += 1;
                }
            }
        }

        println!("Accepted passwords: {}", accepted_password);
    }
}

fn main() {
    problem_2();
}