use helpers::read_lines;
use std::collections::{HashSet};

#[allow(dead_code)]
fn problem_1() {
    let mut num_answers = 0;

    let mut answers: HashSet<char> = HashSet::new();
    print!("Group: ");
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        for line in lines {
            if let Ok(data) = line {
                if data.is_empty() {
                    println!(" - Num answers in this group: {}", answers.len());
                    print!("Group: ");
                    num_answers += answers.len();
                    answers.clear();
                    continue;
                }

                for id in data.chars() {
                    print!("{}", id);
                    answers.insert(id);
                }
            }
        }
    }
    println!(" - Num answers in this group: {}", answers.len());
    num_answers += answers.len();

    println!("Number of yes answers: {}", num_answers);
}

#[allow(dead_code)]
fn problem_2() {
    let mut num_all_answered = 0;

    let mut num_people_in_group = 0;
    let mut num_people_for_question: Vec<i32> = vec![0; 26];

    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        for line in lines {
            if let Ok(data) = line {
                if data.is_empty() {

                    for num in num_people_for_question {
                        if num == num_people_in_group {
                            num_all_answered += 1;
                        }
                    }
                    num_people_in_group = 0;
                    num_people_for_question = vec![0; 26];
                    continue;
                }

                num_people_in_group += 1;
                for code_id in data.chars() {
                    let question_id = (code_id as u8) - ('a' as u8);
                    num_people_for_question[question_id as usize] += 1;
                }
            }
        }
    }

    for num in num_people_for_question {
        if num == num_people_in_group {
            num_all_answered += 1;
        }
    }

    println!("Number of question answered by everyone in a group: {}", num_all_answered);

}

fn main() {
    problem_2();
}