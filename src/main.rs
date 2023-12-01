// Learning Rust. Lesson 1
// My tuple for (index), value should probably be a struct
// Would be semantically clearer

fn first_wordnum_in_str(input: &str, reversed: bool) -> Option<(i32, i32)> {
    let all_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    const EMPTY_STRING: String = String::new();
    let mut words: [String; 9] = [EMPTY_STRING; 9]; 
    if reversed {
        for (i, word) in all_words.iter().enumerate() {
            words[i] = word.chars().rev().collect::<String>();
        }
    } else {
        // I really shouldn't have to do this
        for (i, word) in all_words.iter().enumerate() {
            words[i] = word.to_string();
        }
    }

    let mut index: i32 = -1;
    let mut value: i32 = 0;

    for (i, word) in words.iter().enumerate() {
        if let Some(pos) = input.find(word) {
            if index == -1 || (pos as i32) < index {
                value = (i as i32) + 1; // gets the word value
                index = pos as i32;
            }
        }
    }

    if index == -1 {
        None
    } else {
        Some((index, value))
    }
}

fn first_num_in_str(input: &str) -> Option<(i32, i32)> {
    for (i, c) in input.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            return Some((i as i32, digit as i32));
        }
    }
    None 
}

fn first_num_or_word_in_str(input: &str, reversed: bool) -> i32 {
    let word = first_wordnum_in_str(input, reversed);
    let num = first_num_in_str(input);
    match word {
        Some(word) => 
            match num {
                Some(num) => if word.0 < num.0 { word.1 } else { num.1 },
                None => word.1,
            },
        None => 
            match num {
                Some(num) => num.1,
                None => 0,
            },
        
    }
}

fn num_from_pair(a: i32, b: i32) -> i32 {
    a * 10 + b
}

fn main() {
    let path = std::env::current_dir().unwrap().join("puzzle-inputs/1.txt"); 

    let file_result = std::fs::read_to_string(path);
    let puzzle_input: String;
    match file_result {
        Ok(contents) => puzzle_input = contents,
        Err(err) => panic!("Failed reading puzzle input {:?}", err),
    }

    let mut sum: i32 = 0;
    for line in puzzle_input.lines() {
        let reversed = String::from(line).chars().rev().collect::<String>();
        let value: i32 = num_from_pair(
            first_num_or_word_in_str(line, false),
            first_num_or_word_in_str(reversed.as_str(), true),
        );
        sum += value;
        println!("Value {}", value);
    }

    println!("Total {}", sum);
}
