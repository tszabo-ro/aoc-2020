use helpers::read_lines;
use std::cmp::max;

fn parse_input() -> Vec<String> {
    let mut lines_read: Vec<String> = vec![];
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data) = line {
                lines_read.push(data);
            }
        }
    }
    return lines_read;
}

fn decode_row(row_code : &str) -> u8 {
    assert_eq!(row_code.len(), 7);

    let mut lo : u8= 0;
    let mut hi : u8 = 127;

    for code in row_code.chars() {
        match code {
            'F' => { hi = lo + (((hi - lo + 1) as f32) / 2.0  - 1.0).floor() as u8; }
            'B' => { lo = lo + (((hi - lo + 1) as f32) / 2.0).ceil() as u8; }
            _ => {}
        }
    }

    return lo;
}

fn decode_seat(seat_code : &str) -> u8 {
    assert_eq!(seat_code.len(), 3);

    let mut lo : u8= 0;
    let mut hi : u8 = 7;

    for code in seat_code.chars() {
        match code {
            'L' => { hi = lo + (((hi - lo + 1) as f32) / 2.0  - 1.0).floor() as u8; }
            'R' => { lo = lo + (((hi - lo + 1) as f32) / 2.0).ceil() as u8; }
            _ => {}
        }
    }

    return lo;
}

fn seat_id(card : &str) -> u32{
    assert_eq!(card.len(), 10);
    let row = decode_row(&card[0..7]) as u32;
    let seat = decode_seat(&card[7..10]) as u32;

    return row * 8 + seat;
}

#[allow(dead_code)]
fn problem_1() {
    let cards = parse_input();

    let mut max_id: u32 = 0;
    for card in cards {
        let id = seat_id(&card);
        max_id = max(max_id, id);
    }
    println!("Max ID: {}", max_id);
}

#[allow(dead_code)]
fn problem_2() {
    let mut seat_ids : Vec<u32> = vec![];
    for card in parse_input() {
        seat_ids.push(seat_id(&card));
    }

    seat_ids.sort();
    for i in 1..seat_ids.len() {

        if seat_ids[i] != (seat_ids[i-1] + 1) {
            println!("Seat ID missing between {} and {}",seat_ids[i-1], seat_ids[i]);
        }
    }
}

fn main() {
    problem_2();
}