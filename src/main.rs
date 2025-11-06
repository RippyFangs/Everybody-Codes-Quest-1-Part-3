use std::fs::read_to_string;

fn main() {
    //ingest data in a raw form

    let filename = "assets/everybody_codes_e2025_q01_p2.txt";

    let raw_data = match read_to_string(filename) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening and reading file: {error:?}"),
    };

    //structure data, takes a raw text file assuming the first line is a list of comma separated names, and the third line is a list of comma separated commands

    let structured_data: Vec<String> = raw_data.lines().map(String::from).collect();

    //gives us two vectors, a list of names and a list of commands
    let names: Vec<&str> = structured_data[0].split(',').collect();
    let commands: Vec<&str> = structured_data[2].split(',').collect();

    //follow rules

    //as per the rules we start our journey at 0, this will keep track of what the
    let mut counter: u32 = 0;

    //0 is our minumum, which we'll embody by working in unsigned integers. We can't go equal to or over the maximum number (as equal to is actuall over by 1 due to starting at 0)
    let num_names: u32 = names.len() as u32;
    println!("There are {} names.", num_names);
    //loop through the commands
    for steps in commands {
        //extract the number of steps
        /*         let num_steps: u32 = steps
        .chars()
        .nth(1)
        .expect("Unable to get number of steps")
        .to_digit(10)
        .expect("Unable to convert to digit"); */

        let num_steps: u32 = steps[1..].parse().expect("Unable to convert to digit");

        let reduced_steps: u32 = num_steps % num_names;

        //slight optimisation over last part is that we can now just add all the steps up and then mod number of steps and then we have what we need to do
        //match left or right commands, ensure we don't go over or under
        match steps.chars().next().expect("Unable to get instruction") {
            'R' => counter += reduced_steps,
            'L' => counter += num_names - reduced_steps,
            _ => println!("Scuffed command - check input"),
        };
        println!(
            "Number of steps {}, reduced to result {}, new total {}",
            num_steps, reduced_steps, counter
        );
    }
    counter += num_names;
    counter %= num_names;

    //print result
    println!("The name is {}", names[counter as usize]);
}
