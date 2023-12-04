use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Card {
    id :u32,
    l1 :HashSet<u32>,
    l2: HashSet<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
	let v = line.split("Card").nth(1).unwrap();
	let id_card = v.split(":").nth(0).unwrap().replace(" ","").parse::<u32>().unwrap();
	let mut v2 = v.split(":").nth(1).unwrap().split('|');

	let l1:HashSet<u32> = v2.nth(0).unwrap().split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect();
	let l2:HashSet<u32> = v2.nth(0).unwrap().split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap()).collect();
		    
	return Card {id:id_card, l1:l1, l2:l2}
    }
}


pub fn day4(file_name: &str) -> (u32,u32) {

    let cards:Vec<Card> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| Card::from_line(l))
        .collect();

    let r1:usize = cards.iter()
	.map(|c| match  c.l1.intersection(&c.l2).count() {
	    1 => 1,
	    0 => 0,
	    x =>  (2 as usize).pow((x -1)as u32),
	})
	.sum();

    let mut occ = vec![1 as u32;cards.len()];
    
    for c in cards {
	for _ in 0..occ[c.id as usize - 1] {
	    let num_match = c.l1.intersection(&c.l2).count();
	    for id_c in (c.id as usize)..(c.id as usize + num_match) {
		if let Some(sel_oc) = occ.get_mut(id_c) {
		    *sel_oc +=1;
		}
	    }
	}	
    }

    let r2:u32 = occ.iter().sum();

    (r1 as u32,r2)
}
