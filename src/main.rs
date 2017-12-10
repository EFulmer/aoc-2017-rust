use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

fn main() {
    println!("Hello, world!");
    day_1();
    day_2();
    day_4();
}

fn day_1() {
    // let input_number = args.next().unwrap();
    let input_number = open_and_read("../input/day_1.txt");
    // let mut buf = String::new();
    // io::stdin().read_line(&mut buf).expect("Failed to read line");
    let mut numbers: Vec<u32> = Vec::with_capacity(input_number.len());
    for (i, c) in input_number.trim().char_indices() {
        let char_digit = c.to_digit(10).unwrap();
        numbers.push(char_digit);
        // using buf.len() - 2 because the last character is \n
        if i == input_number.len() - 1 {
            numbers.insert(0, char_digit);
        }
    }
    let rest = numbers.get(1..).unwrap();
    let zipped: Vec<(&u32, &u32)> = numbers.iter().zip(rest.iter()).collect();
    let mut total = 0;
    for (a, b) in zipped {
        if a == b {
            total += *a;
        }
    }
    println!("Answer = {}", total);

    // part two
    let mut total_2 = 0;
    let (first, last) = numbers.get(1..).unwrap().split_at(numbers.get(1..).unwrap().len() / 2);
    let zipped_2: Vec<(&u32, &u32)> = first.iter().zip(last).collect();
    for (a, b) in zipped_2 {
        if a == b {
            total_2 += *a * 2;
        }
    }
    println!("Answer for part 2 = {}", total_2);
}


fn day_2() {
    let mut f = File::open("../input/day_2.txt").expect("File not found");
    let mut text = String::new();

    f.read_to_string(&mut text)
     .expect("Could not read from file");

    let mut numbers: Vec<Vec<i32>> = text.lines().map(|line| {
        let nums: Vec<i32> = line.split_whitespace()
                                 .map(|y| y.parse::<i32>().expect("Error: non-integer found!"))
                                 .collect();
        nums
    }).collect();

    let mut checksum = 0;
    for row in numbers.iter() {
        let max = row.iter().max_by(|x, y| x.cmp(y))
                     .expect("Error: empty row found!");

        let min = row.iter().min_by(|x, y| x.cmp(y))
                     .expect("Error: empty row found!");

        checksum += max - min;
    }
    println!("Day 2 answer = {:?}", checksum);

    // part 2
    // numbers.iter().for_each(|&mut row| row.sort());
    for row in numbers.iter_mut() {
        row.sort();
    }
    let mut checksum_2 = 0;
    for row in numbers.iter() {
        for pair in all_pairs(&row).iter().filter(|t| t.0 != t.1 && t.0 % t.1 == 0) {
            checksum_2 += pair.0 / pair.1;
        }
    }
    println!("Day 2 part 2 answer = {:?}", checksum_2);
}

fn all_pairs<T: Copy>(v: &Vec<T>) -> Vec<(T, T)> {
    let mut result = Vec::with_capacity(v.len());
    for (i, val) in v.iter().enumerate() {
        for val_2 in v.iter().skip(i) {
            result.push((*val, *val_2));
        }
    }
    result
}

/// Helper function to read the input and return the entire text as a String.
/// Shouldn't be used outside of this project where the files might not exist.
fn open_and_read(f_name: &str) -> String {
    let mut f = File::open(f_name).expect("File not found");
    let mut text = String::new();
    f.read_to_string(&mut text)
     .expect("Could not read from file");
    text
}

fn day_4() {
    let text = open_and_read("../input/day_4.txt");
    let passphrases = text.lines();
    let mut unique_count = 0;
    for passphrase in passphrases {
        let words = passphrase.split_whitespace();
        let mut word_set = HashSet::new();
        let mut unique = true;
        for word in words {
            unique = unique && !word_set.contains(word);
            if !unique {
                break;
            }
            word_set.insert(word);
        }
        if unique {
            unique_count += 1;
        }
    }
    println!("Day 4 answer = {:?}", unique_count);
}
