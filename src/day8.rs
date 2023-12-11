use std::fs::read_to_string;
use std::collections::HashMap;


#[derive(Debug)]
struct Choice {
    l: String,
    r: String,
}

#[derive(Debug)]
struct Tree {
    map: HashMap<String, Choice>,
    starts: Vec<String>,
}


impl Tree {
    fn from_file(file_name: &str, is_step_2: bool) -> (Tree, String) {
	let mut map: HashMap<String, Choice> = HashMap::new();
	let mut directions = String::new();
	let mut starts = Vec::new();

	if !is_step_2 {
	    starts.push("AAA".to_string());
	}
	
	for (idx,l) in read_to_string(file_name).unwrap().lines().enumerate() {
	    if idx == 0 {
		directions = l.to_string();
		continue;
	    }
	    if idx == 1 {continue;}

	    if is_step_2 {
		if l.chars().nth(2).unwrap() == 'A' {
		    starts.push((&l[..3]).to_string());
		}
	    }
	    map.insert((&l[..3]).to_string(), Choice {l:(&l[7..10]).to_string(), r:(&l[12..15]).to_string()});
	}
	(Tree {starts:starts, map:map}, directions)
    }

    fn is_end_reached(nodes: &Vec<String>, is_step_2: bool, step:u32, map_step2: &mut HashMap<usize, u32>) -> bool {
	if !is_step_2 {
	    return  nodes[0] == "ZZZ".to_string() ;
	} else {
	    for (idx,nf) in nodes.iter().enumerate() {
		if nf.contains('Z') {
		    map_step2.insert(idx, step);
		}
	    }
	    if  map_step2.len() == nodes.len() {
		println!("Compute LCM with these values\n{:?}", map_step2);
	    }
	    return map_step2.len() == nodes.len()
	}
    }

    fn find_num_step(&self, insts: &String, is_step_2: bool) -> u32 {
	let mut curr_nodes = self.starts.clone();
	let mut count = 0;

	let mut map_step2: HashMap<usize, u32> = HashMap::new();
	
	while !Tree::is_end_reached(&curr_nodes, is_step_2, count, &mut map_step2) {
	    for inst in insts.chars() {

		count += 1;
		let mut tmp_nodes = Vec::new();
		for curr_node in curr_nodes.iter() {
		    if inst == 'L' {
			tmp_nodes.push(self.map.get(curr_node).unwrap().l.clone());		
		    } else {
			tmp_nodes.push(self.map.get(curr_node).unwrap().r.clone());		
		    }		    
		}
		curr_nodes = tmp_nodes.clone();
		if Tree::is_end_reached(&curr_nodes, is_step_2, count, &mut map_step2) {break;}
	    }
	}
	count	
    }
   
}


pub fn day8(file_name: &str) -> (u32,u32) {
    
    let (tree, insts) = Tree::from_file(file_name, false);
    let steps_1 = tree.find_num_step(&insts, false);
    let steps_1 = 3;
    println!("------");
    let (tree2, insts2) = Tree::from_file(file_name, true);
    let steps_2 = tree2.find_num_step(&insts2, true);
 
    (steps_1,steps_2)
}
