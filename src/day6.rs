use std::fs::read_to_string;

fn get_dist_ms(ms: u64, race_ms: u64) -> u64 {
    if ms == 0 || ms >= race_ms{
	return 0;
    }

    (race_ms - ms) * ms
}

pub fn day6(file_name: &str) -> (u32,u32) {

    let input_parsed: Vec<Vec<u64>> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.split(" "))
        .map(|s| s.filter(|ex| ex != &"" && ex.chars().nth(0).unwrap().is_digit(10))
	     .map(|spl| spl.parse::<u64>().unwrap())
	     .collect::<Vec<u64>>())
        .collect();

    let prod_part1 = input_parsed[0].iter().zip(input_parsed[1].iter())
        .map(|(&rms,&d)| (0..d).map(|ms| get_dist_ms(ms,rms)).filter(|&x| x > d).count())
        .fold(1, |acc,x| acc*x);



    let input_parsed2: Vec<u64> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.split(" "))
        .map(|s| s.filter(|ex| ex != &"" && ex.chars().nth(0).unwrap().is_digit(10))
             .map(|s| String::from(s))
	     .fold(String::from(""), |acc,x| format!("{}{}", acc, x)).parse::<u64>().unwrap())
        .collect();

    let rms = input_parsed2[0];
    let d = input_parsed2[1];

    let ratio = d/rms;

    let prod_part2 = (ratio..rms).map(|ms| get_dist_ms(ms,rms)).filter(|&x| x > d).count();

        

    
    (prod_part1 as u32,prod_part2 as u32)
}
