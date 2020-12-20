use helpers::read_lines;
use std::collections::{HashMap};

fn get_bag_collection() -> HashMap<String, Vec<(i32,String)>> {
    let mut bag_collections: HashMap<String, Vec<(i32,String)>> = HashMap::new();
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        for line in lines {
            if let Ok(data) = line {
                let (bag, contains) = decode_contains(&data);
                bag_collections.insert(bag, contains);
            }
        }
    }

    bag_collections
}

fn decode_contains(data: &str) -> (String, Vec<(i32,String)>) {
    let keys : Vec<&str> = data.split(" bags contain ").collect();
    let outer_bag = keys[0].trim();

    let contained_bags: Vec<&str> = keys[1].split(",").collect();

    let mut inner_bags: Vec<(i32, String)> = vec![];
    for bag_id in contained_bags {
        let bag_keys: Vec<&str> = bag_id.trim().split(" ").collect();
        let opt_bag_count = bag_keys[0].parse::<i32>();

        match opt_bag_count {
            Err(_e) => { continue; }
            Ok(num_bags) => {
                let bag = bag_keys[1..bag_keys.len()-1].join(" ");
                inner_bags.push((num_bags, bag));
            }
        }
    }

    return (outer_bag.to_string(), inner_bags);
}

fn bag_contains(bag_definitions: &HashMap<String, Vec<(i32, String)>>, start_at: &str, key: &str) -> bool {
    if start_at == key {
        return true;
    }

    for (_bag_count, bag) in &bag_definitions[start_at] {
        if bag_contains(bag_definitions, &bag, key) {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
fn problem_1() {
    let bag_collections = get_bag_collection();

    let mut num_bags_found = 0;
    let bag_to_look_for = "shiny gold";
    for bag in bag_collections.keys() {
        if bag == bag_to_look_for {
            continue;
        }

        if bag_contains(&bag_collections, bag, bag_to_look_for) {
            println!("Bag {} contains {}", bag, bag_to_look_for);
            num_bags_found += 1;
        }
    }

    println!("{} bags contain {}", num_bags_found, bag_to_look_for);
}

fn num_bags_inside(bag_definitions: &HashMap<String, Vec<(i32, String)>>, bag: &str) -> i32 {
    let mut num_inner_bags = 0;
    for (num_bags, inner_bag) in &bag_definitions[bag] {
        let total = num_bags * (num_bags_inside(bag_definitions, inner_bag) + 1);
        println!("{} contains {} {} bags - total {}", bag, num_bags, inner_bag, total);
        num_inner_bags += total;
    }

    num_inner_bags
}

#[allow(dead_code)]
fn problem_2() {
    let bag_collections = get_bag_collection();
    let bag_to_look_for = "shiny gold";

    println!("{} bags contains {} others", bag_to_look_for,
             num_bags_inside(&bag_collections, bag_to_look_for));
}

fn main() {
    problem_2();
}