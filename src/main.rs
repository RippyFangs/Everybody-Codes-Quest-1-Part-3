use core::num;
use std::{fs::read_to_string, mem::swap};

fn main() {
    //ingest data in a raw form

    let filename = "assets/everybody_codes_e2025_q01_p3.txt";

    let raw_data = match read_to_string(filename) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening and reading file: {error:?}"),
    };

    //structure data, takes a raw text file assuming the first line is a list of comma separated names, and the third line is a list of comma separated commands

    let structured_data: Vec<String> = raw_data.lines().map(String::from).collect();

    //gives us two vectors, a list of names and a list of commands
    let mut names: Vec<&str> = structured_data[0].split(',').collect();
    let commands: Vec<&str> = structured_data[2].split(',').collect();

    //follow rules

    //0 is our minumum, which we'll embody by working in unsigned integers. We can't go equal to or over the maximum number (as equal to is actuall over by 1 due to starting at 0)
    let num_names: u32 = names.len() as u32;
    println!("There are {} names.", num_names);
    //loop through the commands
    for steps in commands {
        let num_steps: u32 = steps[1..]
            .parse()
            .expect("Unable to convert instruction to digit");

        let mut swap_target: u32 = num_steps % num_names;

        //match left or right commands, ensure we don't go under
        match steps.chars().next().expect("Unable to get instruction") {
            'R' => (),
            'L' => swap_target = num_names - swap_target,
            _ => println!("Scuffed command - check input"),
        };

        swap_target += num_names - 1;
        swap_target %= num_names;
        println!("The swap target is {},", swap_target);
        names.swap(0, swap_target as usize);
    }

    //print result
    println!("The name is {}", names[0]);
}
