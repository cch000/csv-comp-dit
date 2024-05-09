use std::env;
use std::{error::Error, process};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    //sent_seconds: u64,
    //sent_microseconds: u64,
    //received_seconds: u64,
    //received_microseconds: u64,
    //sample_time_stamp_seconds: u64,
    sample_time_stamp_microseconds: u64,
    ground_steering: f64,
}

fn get_values(answer_path: &str) -> Result<Vec<(u64, f64)>, Box<dyn Error>> {
    let mut values: Vec<(u64, f64)> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(answer_path)
        .expect("Where file");

    for result in rdr.deserialize() {
        let record: Record = result?;

        values.push((
            record.sample_time_stamp_microseconds,
            record.ground_steering,
        ));
    }
    Ok(values)
}

fn compare_values(example: &Vec<(u64, f64)>, ours: &Vec<(u64, f64)>, smallest_index: usize) {
    println!("---------MATCHED FRAMES +-25% error---------");

    let mut counter = 0;
    let mut non_zero_counter = 0;

    for i in 0..smallest_index {
        let (example_time, example_angle) = example[i];
        let (our_time, our_angle) = ours[i];

        if example_angle != 0.0 || example_angle != -0.0 {
            non_zero_counter += 1;

            let error_margin = 25.0 * example_angle.abs() / 100.0;
            let lower_bound = example_angle - error_margin;
            let upper_bound = example_angle + error_margin;

            if our_angle > lower_bound && our_angle < upper_bound {
                println!(
                    "index: {}; example timestamp(ms) {}; our timestamp(ms): {}",
                    i, example_time, our_time
                );
                counter += 1;
            }
        }
    }

    let percentage = counter as f64 / non_zero_counter as f64 * 100.0;

    println!("------------------SUMMARY-------------------");
    println!("Number of frames on the example csv: {}", example.len());
    println!("Number of frames on our csv: {}", ours.len());
    println!(
        "Total frames taken into consideration (non zero): {}",
        non_zero_counter
    );
    println!("Total frames with the same steering angle: {}", counter);
    println!(
        "Percentage of frames with the steering angle within the error margin (25%): {}%",
        percentage
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments");
        process::exit(1);
    }

    let example_path = &args[1];
    let our_path = &args[2];

    if let Err(err) = get_values(&example_path) {
        println!("error answer_path: {}", err);
        process::exit(1);
    }
    if let Err(err) = get_values(&our_path) {
        println!("error our_path: {}", err);
        process::exit(1);
    }

    let example_values = get_values(&example_path).unwrap();
    let our_values = get_values(&our_path).unwrap();

    let example_len = example_values.len();
    let our_len = our_values.len();

    if example_len <= our_len {
        compare_values(&example_values, &our_values, example_len);
        println!("We have more frames");
    } else {
        compare_values(&example_values, &our_values, our_len);
        println!("We have less frames");
    }
}
