mod day1;
mod day2;

fn main() {
    let (d1_p1, d1_p2) = day1::day1("/home/matthieu/dev/adventofcode2023/adventofcode2023/input_files/input_day1");
    let (d2_p1, d2_p2) = day2::day2("/home/matthieu/dev/adventofcode2023/adventofcode2023/input_files/input_day2");
    
    println!("d11 {} d12 {}",d1_p1, d1_p2);
    println!("d21 {} d22 {}",d2_p1, d2_p2);
}
