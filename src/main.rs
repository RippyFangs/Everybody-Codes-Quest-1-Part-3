use std::fs::read_to_string;

fn main() {
    //ingest data in a raw form

    let filename = "assets/everybody_codes_e2025_q01_p1.txt";

    let raw_data = match read_to_string(filename) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening and reading file: {error:?}"),
    };

    //structure data, takes a raw text file assuming the first line is a list of comma separated names, and the third line is a list of comma separated commands

    let structured_data: Vec<String> = raw_data.lines().map(String::from).collect();

    //gives us two vectors, a list of names and a list of commands
    let names: Vec<&str> = structured_data[0].split(",").collect();
    let commands: Vec<&str> = structured_data[2].split(",").collect();

    //follow rules

    //as per the rules we start our journey at 0, this will keep track of what the
    let mut chosen_name: u32 = 0;

    //0 is our minumum, which we'll embody by working in unsigned integers. We can't go equal to or over the maximum number (as equal to is actuall over by 1 due to starting at 0)
    let max_names: u32 = names.len() as u32;

    //loop through the commands
    for steps in commands {
        //extract the number of steps
        let num_steps: u32 = steps
            .chars()
            .nth(1)
            .expect("Unable to get number of steps")
            .to_digit(10)
            .expect("Unable to convert to digit");

        //match left or right commands, ensure we don't go over or under
        match steps.chars().nth(0).expect("Unable to get instruction") {
            'R' => {
                if chosen_name + num_steps >= max_names {
                    chosen_name = max_names - 1
                } else {
                    chosen_name += num_steps
                }
            }
            'L' => {
                if num_steps > chosen_name {
                    chosen_name = 0
                } else {
                    chosen_name -= num_steps
                }
            }
            _ => println!("Scuffed command - check input"),
        };
    }
    //print result
    println!("The name is {}", names[chosen_name as usize]);
}
