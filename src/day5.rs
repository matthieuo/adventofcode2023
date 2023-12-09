use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct Mapping {
    dest: u64,
    source: u64,
    len: u64,
}


impl Mapping {
    fn from_line(line: &str) -> Mapping {
	let v_tmp: Vec<u64> = line.split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
	Mapping {dest: v_tmp[0], source:v_tmp[1], len: v_tmp[2]}
    }

    fn map(&self, num :u64) -> Option<u64> {
	if num >= self.source && num < self.source + self.len {
	    let off = num - self.source;
	    return Some(self.dest + off);
	}	
	
	None
    }
}
#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn map(&self, num: u64) -> u64 {
	for m in &self.mappings {
	    if let Some(v) = m.map(num) {
		return v
	    }
	}
	num
    }
}

fn parse_input(file_name: &str) -> (Vec<u64>, HashMap<&str, Map>){

    let mut out_maps = HashMap::new();

    for k in ["sts","stf","ftw", "wtl", "ltt", "tth", "htl"] {
	out_maps.insert(k, Map {mappings : Vec::new()});
    }
    
    let mut seeds:Vec<u64> = Vec::new();
    let mut cur_step = "";
    for  l in read_to_string(file_name).unwrap().lines() {
	if l.contains("seeds:") {
	    seeds = l.split(" ")
		.filter(|e| !e.contains("seeds:"))
		.map(|s| s.parse::<u64>().unwrap())
		.collect();
	}
	
	if cur_step != "" && l != "" {
	    out_maps.get_mut(cur_step).unwrap().mappings.push(Mapping::from_line(l));
	}
	
	match l {
	    "seed-to-soil map:" => cur_step = "sts",
	    "soil-to-fertilizer map:" => cur_step = "stf",
	    "fertilizer-to-water map:" => cur_step = "ftw",
	    "water-to-light map:" => cur_step = "wtl",
	    "light-to-temperature map:" => cur_step = "ltt",
	    "temperature-to-humidity map:" => cur_step = "tth",
	    "humidity-to-location map:" => cur_step = "htl",
	    "" => cur_step = "",
	    _ => (),
	}
    
    }
    (seeds, out_maps)
}


fn find_min(seeds: &Vec<u64>, maps: &HashMap<&str, Map>) -> u32 {
    seeds.iter().map(|&s| {
	let mut val = s;
	for k in ["sts","stf","ftw", "wtl", "ltt", "tth", "htl"] {
	    val = maps.get(k).unwrap().map(val);
	}
	val
    }).min().unwrap() as u32
}

pub fn day5(file_name: &str) -> (u32,u32) {

    let (seeds, maps) = parse_input(file_name);
    let mut min_val = u64::MAX;
    for (&s,&v) in seeds.iter().step_by(2).zip(seeds.iter().skip(1).step_by(2)) {
	for i in 0..v {
	    let mut val = s + i;
	    for k in ["sts","stf","ftw", "wtl", "ltt", "tth", "htl"] {
		val = maps.get(k).unwrap().map(val);
	    }
	    if val < min_val {
		min_val = val
	    }
	}
    }

    (find_min(&seeds, &maps),min_val as u32)
}
