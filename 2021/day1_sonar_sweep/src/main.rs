use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = Path::new("sweep_report.txt");

    first_part(file_path)?;
    second_part(file_path)?;
    second_part_general(file_path, &3)?;

    Ok(())
}

fn first_part(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    let mut measure_inc_count = 0;
    let mut prev_measure: Option<usize> = None;

    for line in lines {
        let line = line.expect("Unable to read the line");
        let sweep_reading = line.parse::<usize>().expect("Unable to parse the line to integer");

        if let Some(prev) = prev_measure  {
            if prev < sweep_reading {
                measure_inc_count += 1;
            }
        }
        prev_measure = Some(sweep_reading);
    }

    println!("{}", measure_inc_count);
    Ok(())
}

fn second_part(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    let mut measure_inc_count = 0;
    let mut prev_1: Option<usize> = None;
    let mut prev_2: Option<usize> = None;
    let mut prev_3: Option<usize> = None;

    for line in lines {
        let line = line.expect("Unable to read the line");
        let sweep_reading = line.parse::<usize>().expect("Unable to parse the line to integer");

        match (prev_1, prev_2, prev_3) {
            (Some(p1), Some(p2), Some(p3))
            if (p1 + p2 + p3 ) < sweep_reading + p1 + p2 => {
                measure_inc_count += 1;
                prev_3 = Some(p2);
                prev_2 = Some(p1);
                prev_1 = Some(sweep_reading);
            },
            (Some(p1), Some(p2), Some(_)) => {
                prev_3 = Some(p2);
                prev_2 = Some(p1);
                prev_1 = Some(sweep_reading);
            },
            (Some(p1), Some(p2), None) => {
                prev_3 = Some(p2);
                prev_2 = Some(p1);
                prev_1 = Some(sweep_reading);
            },
            (Some(p1), None, None) => {
                prev_2 = Some(p1);
                prev_1 = Some(sweep_reading);
            },
            (None, None, None) => {
                prev_1 = Some(sweep_reading);
            },
            (_, _, _) => {

            }
        }
    }

    println!("{}", measure_inc_count);
    Ok(())
}

fn second_part_general(path: &Path, window_size: &usize) -> io::Result<()> {
    let file = File::open(path)?;
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    let mut measure_inc_count = 0;
    let mut prev_measurements = std::collections::VecDeque::with_capacity(*window_size);

    let mut prev_moving_sum = 0;
    for line in lines {
        let line = line.expect("Unable to read the line");
        let sweep_reading = line.parse::<usize>().expect("Unable to parse the line to integer");
        if prev_measurements.len() == *window_size {
            let sum: usize = prev_measurements.iter().sum();
            if sum > prev_moving_sum {
                measure_inc_count += 1;
            }
            prev_moving_sum = sum;
            prev_measurements.pop_back();
        }
        prev_measurements.push_front(sweep_reading);
    }

    println!("{}", measure_inc_count);
    Ok(())
}