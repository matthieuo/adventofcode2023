use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Clone, Copy,  Debug)]
struct Elt {
    c: char,
    pn: bool,
    idx_s: Option<u32>,
}

fn get_num_stars(v :&Vec<Elt>, map :&mut HashMap<u32, Vec<u32>> ) {
    let mut cur_str: String = String::new();
    let mut cur_star = None;

   
    
    for e in v {
	if e.c.is_digit(10) {
	    cur_str.push(e.c);
	    if let Some(i) = e.idx_s {
		cur_star = Some(i);
	    }
	} else {
	    
	    if let Some(i) = cur_star {
		if map.contains_key(&i) {
		    map.get_mut(&i).unwrap().push(cur_str.parse::<u32>().unwrap());
		}
		else {
		    map.insert(i, vec![cur_str.parse::<u32>().unwrap()]);
		}
	    }
	    cur_star = None;
	    cur_str.clear();
	}
    }

    if let Some(i) = cur_star {
	if map.contains_key(&i) {
	    map.get_mut(&i).unwrap().push(cur_str.parse::<u32>().unwrap());
	}
	else {
	    map.insert(i, vec![cur_str.parse::<u32>().unwrap()]);		   
	}
    }



}



fn get_num_line(v :&Vec<Elt>) -> u32 {

    let mut ret_val: u32 = 0;
    let mut cur_str: String = String::new();
    let mut cur_pn = false;
    
    for e in v {
	if e.c.is_digit(10) {
	    cur_str.push(e.c);
	    if e.pn {
		cur_pn = true;
	    }
	} else {
	    if cur_pn {
		ret_val += cur_str.parse::<u32>().unwrap();
	    }
	    cur_pn = false;
	    cur_str.clear();
	}

    }

    if cur_pn {
	ret_val + cur_str.parse::<u32>().unwrap()
    }
    else {
	ret_val
    }
}

pub fn day3(file_name: &str) -> (u32,u32) {

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    
    let mut input_grid: Vec<Vec<Elt>> = read_to_string(file_name)
	.unwrap()
	.lines()
	.map(|l| l.chars().map(|c| Elt {c: c,pn: false, idx_s:None}).collect::<Vec<Elt>>())
	.collect();

    let mut num_s = 0;

    for (l_idx, line) in input_grid.clone().iter().enumerate() {
	for (c_idx, el) in line.iter().enumerate() {
	    if (!el.c.is_digit(10)) & (el.c != '.') {
		if el.c == '*' {
		    num_s += 1;
		}
		for m_i in [-1,0,1] {
		    for m_j in [-1,0,1] {
			if (l_idx as i32+ m_i >= 0) & (c_idx as i32 + m_j >= 0) { 
			
			    if let Some(l_tom) = input_grid.get_mut((l_idx as i32 + m_i) as usize) {
				if let Some(el_tom) = l_tom.get_mut((c_idx as i32 + m_j) as usize) {
				    el_tom.pn = true;

				    if el.c == '*' {
					el_tom.idx_s = Some(num_s);
				    }
				}
			    }
			}
		    }
		
		}
	    }
	}
    }

    let sum_1:u32 = input_grid.iter()
        .map(|l| get_num_line(l))
        .sum();

    for l in input_grid.iter() {
	get_num_stars(l,&mut map);
    }

    let sum_2:u32 = map.iter().filter(|(_,v)| v.len() == 2).map(|(_,v)| v[0]*v[1]).sum();

    
    (sum_1,sum_2)
}
