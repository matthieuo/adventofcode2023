mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let (d1_p1, d1_p2) = day1::day1("input_files/input_day1");
    let (d2_p1, d2_p2) = day2::day2("input_files/input_day2");
    let (d3_p1, d3_p2) = day3::day3("input_files/input_day3");
    let (d4_p1, d4_p2) = day4::day4("input_files/input_day4");
    let (d5_p1, d5_p2) = day5::day5("input_files/input_day5");
    
    println!("d11 {} d12 {}",d1_p1, d1_p2);
    println!("d21 {} d22 {}",d2_p1, d2_p2);
    println!("d31 {} d32 {}",d3_p1, d3_p2);
    println!("d41 {} d42 {}",d4_p1, d4_p2);
    println!("d51 {} d52 {}",d5_p1, d5_p2);
}
