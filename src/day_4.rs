use helpers::read_lines;
use std::fmt;

struct UserData {
    byr : Option<String>,
    iyr : Option<String>,
    eyr : Option<String>,
    hgt : Option<String>,
    hcl : Option<String>,
    ecl : Option<String>,
    pid : Option<String>,
    cid : Option<String>,
}

impl UserData {
    fn new() -> UserData {
        return UserData{byr : None,
            iyr : None,
            eyr :  None,
            hgt :  None,
            hcl :  None,
            ecl :  None,
            pid :  None,
            cid :  None
        };
    }

    fn is_valid_simple(&self) -> bool {
        if self.byr.is_none() || self.iyr.is_none() || self.eyr.is_none() || self.hgt.is_none() ||
            self.hcl.is_none() || self.ecl.is_none() || self.pid.is_none() {

            return false;
        }

        return true;
    }

    fn is_byr_valid(&self) -> bool {
        if self.byr.is_none() { return false; }
        let val = &self.byr.as_ref().unwrap();

        if val.len() != 4 { return false; }

        let byr = val.parse::<i32>().unwrap();
        if (byr < 1920) || (byr > 2002) { return false; }

        return true;
    }

    fn is_iyr_valid(&self) -> bool {
        if self.iyr.is_none() { return false; }
        let val = &self.iyr.as_ref().unwrap();

        if val.len() != 4 { return false }
        let iyr = val.parse::<i32>().unwrap();
        if (iyr < 2010) || (iyr > 2020) { return false; }

        return true;
    }

    fn is_eyr_valid(&self) -> bool {
        if self.eyr.is_none() { return false; }
        let val = &self.eyr.as_ref().unwrap();

        if val.len() != 4 { return false }
        let eyr = val.parse::<i32>().unwrap();
        if (eyr < 2020) || (eyr > 2030) { return false; }

        return true;
    }

    fn is_hgt_valid(&self) -> bool {
        if self.hgt.is_none() { return false; }
        let val = &self.hgt.as_ref().unwrap();

        if val.len() < 3 { return false; }
        let hgt_val = val[0..val.len()-2].parse::<i32>().unwrap();

        match &val[(val.len()-2)..(val.len())] {
            "cm" => {
                if (hgt_val < 150) || (hgt_val > 193) { return false; }
            }
            "in" => {
                if (hgt_val < 59) || (hgt_val > 76) { return false; }
            }
            _ => { return false; }
        }

        return true;
    }

    fn is_hcl_valid(&self) -> bool {
        if self.hcl.is_none() { return false; }
        let val = self.hcl.as_ref().unwrap();

        if val.len()  != 7 || val.chars().nth(0).unwrap() != '#' { return false; }
        if !val[1..6].chars().all(char::is_alphanumeric) { return false; }

        return true;
    }

    fn is_ecl_valid(&self) -> bool {
        if self.ecl.is_none() { return false; }
        let val = self.ecl.as_ref().unwrap();

        return match val.as_str() {
            "amb" => { true },
            "blu" => { true },
            "brn" => { true },
            "gry" => { true },
            "grn" => { true },
            "hzl" => { true },
            "oth" => { true },
            _ => { false }
        }
    }

    fn is_pid_valid(&self) -> bool {
        if self.pid.is_none() { return false; }
        let val = self.pid.as_ref().unwrap();

        if val.len()  != 9 { return false; }
        return val.chars().all(char::is_numeric);
    }

    fn is_cid_valid(&self) -> bool {
        // There are no checks for this!
        return true;
    }

    fn is_valid_complex(&self) -> bool {
        return
            self.is_byr_valid() && self.is_iyr_valid() && self.is_pid_valid() && self.is_ecl_valid() &&
            self.is_hcl_valid() && self.is_cid_valid() && self.is_eyr_valid() && self.is_hgt_valid();
    }
}

impl fmt::Display for UserData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        let str = "UserData{".to_string() +
            " byr: " + match &self.byr {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " iyr: " + match &self.iyr {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " eyr: " + match &self.eyr {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " hgt: " + match &self.hgt {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " ecl: " + match &self.ecl {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " pid: " + match &self.pid {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " cid: " + match &self.cid {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " hcl: " + match &self.hcl {
                None => "--NONE--",
                Some(x) => x.as_str()
            } + " }";

        formatter.pad(str.as_str())
    }
}


fn parse_passports() -> Vec<UserData> {
    let mut passports: Vec<UserData> = vec![];
    if let Ok(lines) = read_lines("/Users/tszabo/projects/sandbox/advent-of-code/input") {
        // Consumes the iterator, returns an (Optional) String
        passports.push(UserData::new());
        for line in lines {
            if let Ok(data) = line {
                if data.is_empty() {
                    passports.push(UserData::new());
                }

                let tokens: Vec<&str> = data.split(" ").collect();

                for kvpair_str in tokens {
                    let kvpair: Vec<&str> = kvpair_str.split(":").collect();

                    match kvpair[0] {
                        "byr" => passports.last_mut().unwrap().byr = Some(kvpair[1].to_string()),
                        "iyr" => passports.last_mut().unwrap().iyr = Some(kvpair[1].to_string()),
                        "eyr" => passports.last_mut().unwrap().eyr = Some(kvpair[1].to_string()),
                        "hgt" => passports.last_mut().unwrap().hgt = Some(kvpair[1].to_string()),
                        "hcl" => passports.last_mut().unwrap().hcl = Some(kvpair[1].to_string()),
                        "ecl" => passports.last_mut().unwrap().ecl = Some(kvpair[1].to_string()),
                        "pid" => passports.last_mut().unwrap().pid = Some(kvpair[1].to_string()),
                        "cid" => passports.last_mut().unwrap().cid = Some(kvpair[1].to_string()),
                        _ => ()
                    }
                }
            }
        }
    }
    return passports;
}

#[allow(dead_code)]
pub fn problem_1() {
    let passports = parse_passports();
    let mut valid_passport_count = 0;
    for entry in &passports {
        if entry.is_valid_simple() {
            valid_passport_count += 1;
        }
    }
    println!("NumUsers: {} - {} valid", &passports.len(), valid_passport_count);
}

#[allow(dead_code)]
pub fn problem_2() {
    let passports = parse_passports();
    let mut valid_passport_count = 0;
    for entry in &passports {
        if entry.is_valid_complex() {
            valid_passport_count += 1;
        }
    }
    println!("NumUsers: {} - {} valid", &passports.len(), valid_passport_count);
}

fn main() {
    problem_2();
}