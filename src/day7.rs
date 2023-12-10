use std::fs::read_to_string;
use std::collections::HashMap;


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    t: HandType,
    cards_digit: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn from_line(line: &str, bid: u32, is_part_2: bool) -> Hand {
	let mut map: HashMap<char,u32> = HashMap::new();
	
	for c in line.chars() {
	    if let Some(x) = map.get_mut(&c) {
		*x += 1;
	    } else {
		map.insert(c,1);
	    }
	   
	}

	let cards_digit:Vec<u32> = line
	    .chars()
	    .map(|c| match c {
		'A' => 14,
		'K' => 13,
		'Q' => 12,
		'J' => if is_part_2 {1} else {11},
		'T' => 10,
		x => x.to_digit(10).unwrap(),
	    })
	    .collect();



	if is_part_2 {
	    let max_kv_try = map
		.iter()
		.filter(|(k,_)| **k != 'J')
		.max_by(|a, b| a.1.cmp(&b.1));

	    if let Some((&max_k,_)) = max_kv_try {
		let mut j_cards = 0;
		if let Some(j_found) = map.get(&'J') {
		    j_cards = *j_found;
		}
		*map.get_mut(&max_k).unwrap() += j_cards;
	    }
	    else {
		//only J
		map.insert('2', 5);
	    }
	}

	
	let max_val = map.values().max().unwrap();

	let mut ht;
	match max_val {
	    5 => ht = HandType::Five,
	    4 => ht = HandType::Four,
	    3 => ht = HandType::Three,
	    1 => ht = HandType::High,
	    _=>  ht = HandType::OnePair,
	}

	if *max_val == 3 {
	    let mut is_pair = false;
	    for (_,&v) in map.iter().filter(|(k,_)| **k != 'J') {
		if v == 2 {
		    is_pair = true;
		}
	    }
	    if is_pair {
		ht = HandType::Full;
	    }
	    
	}
	
	if *max_val == 2 {
	    //maybe 2 pairs...
	    let mut num_pair = 0;
	    for (_,&v) in map.iter().filter(|(k,_)| **k != 'J') {
		if v == 2 {
		    num_pair +=1;
		}
	    }
	    if num_pair == 1 {
		ht = HandType::OnePair;
	    } else {
		ht = HandType::TwoPair;
	    }
	}

	
	Hand {t: ht, cards_digit:cards_digit, bid:bid}
    }
}

pub fn day7(file_name: &str) -> (u32,u32) {
    let mut input_parsed: Vec<Hand> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
	    let mut elts = l.split(" ");
	    (elts.nth(0).unwrap(), elts.nth(0).unwrap())
	})
        .map(|(t1,t2)| Hand::from_line(t1,t2.parse::<u32>().unwrap(), false))
        .collect();


    let mut input_parsed_part2: Vec<Hand> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
	    let mut elts = l.split(" ");
	    (elts.nth(0).unwrap(), elts.nth(0).unwrap())
	})
        .map(|(t1,t2)| Hand::from_line(t1,t2.parse::<u32>().unwrap(), true))
        .collect();
    
    input_parsed.sort();
    let out_sum1: u32 = input_parsed.iter().enumerate().map(|(idx,h)| (1 + idx as u32)*h.bid).sum();
 
    input_parsed_part2.sort();
    let out_sum2: u32 = input_parsed_part2.iter().enumerate().map(|(idx,h)| (1 + idx as u32)*h.bid).sum();
 
    
    (out_sum1,out_sum2)
}
