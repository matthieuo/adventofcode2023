use std::fs::read_to_string;
use std::collections::HashSet;



/*fn compute_sum(file_name: &str, convert_str: bool) -> u32 {
    let sum_cal: u32 = read_to_string(file_name)
	.unwrap()
	.lines()
	.map(|l| if convert_str  {prepare_string(l)} else {String::from(l)})*/


#[derive(Hash, Eq, PartialEq, Debug)]
struct Grab {
    green: u32,
    red: u32,
    blue: u32,
}

struct Game {
    id: u32,
    grabs: HashSet<Grab>,
}
impl Grab {
    fn from_txt(line: &str) -> Grab{
	let mut gre=0;
	let mut red=0;
	let mut blu=0;
	
	
	for s in line.split(",") {
	    if s.contains("green") {
		gre = s.split(" green").nth(0).unwrap()[1..].parse::<u32>().unwrap();
	    }
	    if s.contains("blue") {
		blu = s.split(" blue").nth(0).unwrap()[1..].parse::<u32>().unwrap();
	    }
	    if s.contains("red") {
		red = s.split(" red").nth(0).unwrap()[1..].parse::<u32>().unwrap();
	    }
	    //println!("{:?}", s.split(" green").nth(0))s.split(" green").nth(0));
	}
	//println!("{:?}", line);
	//println!("{} {} {}",gre,red,blu);
	Grab {green:gre,red:red,blue:blu}
    }
}
impl Game {
    fn from_line(line: &str) -> Game{
	let v = line.split("Game").nth(1).unwrap();
	let id_game = v.split(":").nth(0).unwrap()[1..].parse::<u32>().unwrap();
	let grabs: HashSet<Grab> = v.split(":").nth(1).unwrap().split(";").map(|s| Grab::from_txt(s)).collect();

	//println!("{:?}", grabs);
	Game {id:id_game, grabs: grabs}
	
    }

    fn valid(&self, gr:u32, bl:u32,re:u32) -> bool {
	for g in &self.grabs {
	    if (g.blue > bl) | (g.red > re) | (g.green > gr) {
		return false
	    }
	}
	true
    }

    fn max_vals(&self) -> u32 {
	let max_r = self.grabs.iter().map(|g| g.red).max().unwrap();
	let max_b = self.grabs.iter().map(|g| g.blue).max().unwrap();
	let max_g = self.grabs.iter().map(|g| g.green).max().unwrap();

	max_r*max_b*max_g
    }
}

pub fn day2(file_name: &str) -> (u32,u32) {

    let sum_id:u32 = read_to_string(file_name)
	.unwrap()
	.lines()
	.map(|l| Game::from_line(l))
	.filter(|g| g.valid(13,14,12))
	.map(|g| g.id)
	.sum();

        let sum_max:u32 = read_to_string(file_name)
	.unwrap()
	.lines()
	.map(|l| Game::from_line(l))
	.map(|g| g.max_vals())
	.sum();
    
    (sum_id,sum_max)
    
}
