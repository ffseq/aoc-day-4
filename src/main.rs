use std::{fs::File, io::{BufReader, prelude::*}};

const INPUT_FILE: &str = "input.txt";

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new(pass_slice: &[String]) -> Self {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
        for field in pass_slice {
            let kv: Vec<_> = field.split(':').collect();
            match kv[0] {
                "byr" => passport.byr = Some(kv[1].to_string()),
                "iyr" => passport.iyr = Some(kv[1].to_string()),
                "eyr" => passport.eyr = Some(kv[1].to_string()),
                "hgt" => passport.hgt = Some(kv[1].to_string()),
                "hcl" => passport.hcl = Some(kv[1].to_string()),
                "ecl" => passport.ecl = Some(kv[1].to_string()),
                "pid" => passport.pid = Some(kv[1].to_string()),
                "cid" => passport.cid = Some(kv[1].to_string()),
                _ => panic!("Bad input!"),
            }
        }
        passport
    }
    
    fn is_valid(&self) -> bool {
        matches!(self, Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
        })
    }
}

fn get_passport(buf: &mut BufReader<File>) -> Option<Vec<String>> {
    let mut result = Vec::new();
    for line in buf.lines() {
        match line.unwrap() {
            l if l.is_empty() => {
                if result.is_empty() {
                    return None;
                }
                else {
                    return Some(result);
                }
            },
            l => {
                l.trim_end()
                    .split(' ').for_each(|field| {
                        result.push(field.to_string());
                    });
            },
        }
        
    }
    
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}


fn main() {
    let mut valid_passports = 0;
    let f = File::open(INPUT_FILE).unwrap();
    let mut reader = BufReader::new(f);
    while let Some(pass_slice) = get_passport(&mut reader) {
        let passport = Passport::new(&pass_slice);
        if passport.is_valid() {
            valid_passports += 1;
        }
    }
    println!("Valid passports: {}", valid_passports);
}
