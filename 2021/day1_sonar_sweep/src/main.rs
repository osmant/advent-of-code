fn main() {
    let lines = include_str!("input.txt");

    println!("first_part : {}", first_part(lines));
    println!("second_part : {}", second_part(lines));
    println!("second_part_general : {}", second_part_general(lines, &3));
}

fn first_part(input: &str) -> usize {
    let mut measure_inc_count = 0;
    let mut prev_measure: Option<usize> = None;

    for line in input.lines() {
        let sweep_reading = line.trim().parse::<usize>().expect("Unable to parse the line to integer");

        if let Some(prev) = prev_measure  {
            if prev < sweep_reading {
                measure_inc_count += 1;
            }
        }
        prev_measure = Some(sweep_reading);
    }

    measure_inc_count
}

fn second_part(input: &str) -> usize {
    let mut measure_inc_count = 0;
    let mut prev_1: Option<usize> = None;
    let mut prev_2: Option<usize> = None;
    let mut prev_3: Option<usize> = None;


    for line in input.lines() {
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

    measure_inc_count
}

fn second_part_general(input: &str, window_size: &usize) -> usize {
    let mut measure_inc_count = 0;
    let mut prev_measurements = std::collections::VecDeque::with_capacity(*window_size);

    let mut prev_moving_sum = 0;
    for line in input.lines() {
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

    measure_inc_count
}
