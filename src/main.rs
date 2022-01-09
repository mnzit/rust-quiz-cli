use crate::readable::readable::{FileReader, Readable};
use crate::prompt::simple_user_input::get_input;
use rand::thread_rng;
use rand::seq::SliceRandom;
mod readable;
mod prompt;

fn main() {
    let data_list: Vec<String> = FileReader {}.read(String::from("data.txt"));
    begin_quiz(data_list);
}

fn begin_quiz(mut data_list: Vec<String>) {
    println!("{}", "Welcome to Quiz");
    let mut exit = false;
    let mut score = 0;
    loop {
        let slice = data_list.as_mut_slice();
        let has_begin = get_input("Press (B) begin to (Q) quit the game.");
        slice.shuffle(&mut thread_rng());
        if has_begin.eq("B") {
            for (_pos, data) in data_list.iter().enumerate() {
                println!("{}", "Press [1][2][3][4] to answer or press [Q] to quit.");
                let data: Vec<&str> = data.split(":").collect();

                let question = data.get(0).unwrap();
                let option1 = data.get(1).unwrap();
                let option2 = data.get(2).unwrap();
                let option3 = data.get(3).unwrap();
                let option4 = data.get(4).unwrap();
                let actual_answer = String::from(data.get(5).unwrap().to_string());
                println!("{}","------------------------------------------------------------------------------------------------");
                println!("{}", question);
                println!("[1] {}", option1);
                println!("[2] {}", option2);
                println!("[3] {}", option3);
                println!("[4] {}", option4);
                println!("{}","------------------------------------------------------------------------------------------------");
                loop {
                    let answer = get_input("Choose [1][2][3][4]: ");
                    if answer.eq("1") || answer.eq("2") || answer.eq("3") || answer.eq("4") {
                        if actual_answer.eq(answer.as_str()) {
                            score = score + 1;
                        }
                        break;
                    } else if answer.eq("Q") {
                        exit = true;
                        break;
                    } else {
                        println!("{}", "Incorrect input. Please press [1][2][3][4] to answer or press [Q] to quit.");
                    }
                }

                if exit {
                    break;
                }
            }
            println!("{}","------------------------------------------------------------------------------------------------");
            println!("Your Score is {}/{}", score, data_list.len());
            println!("{}","------------------------------------------------------------------------------------------------");
        }  else if has_begin.eq("Q") {
            println!("{}", "Thank you for entering in our game!!");
            break;
        } else {
            println!("{}", "Incorrect input. Press (B) begin to (Q) quit the game.");
        }
    }
}

