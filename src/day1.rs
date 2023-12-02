use std::fs::read_to_string;

fn prepare_string(str_in: &str) -> String {
    let mut str_out = String::from(str_in);
    
    for (s_rep, s_num) in [("one","1"),("two","2"), ("three","3"), ("four","4"),("five","5"),("six","6"),("seven","7"),("eight","8"), ("nine","9")] {
	let mut st_id = str_out.find(s_rep);
	while let Some(i) = st_id {
	    let len_str= s_rep.len();
	    
	    str_out.replace_range(i+1..i+len_str-2, s_num);
	    st_id = str_out.find(s_rep);
	};
    }
    str_out
}

fn compute_sum(file_name: &str, convert_str: bool) -> u32 {
    let sum_cal: u32 = read_to_string(file_name)
	.unwrap()
	.lines()
	.map(|l| if convert_str  {prepare_string(l)} else {String::from(l)})
	.map(|l| {
	    let first_digit = l.chars().find(|c| c.is_digit(10));
	    let sec_digit = l.chars().rev().find(|c| c.is_digit(10));

	    match (first_digit,sec_digit) {
		(Some(f_d),Some(s_d)) => f_d.to_digit(10).unwrap() *10+s_d.to_digit(10).unwrap(),
		_=>0,	    
	    }
	})
	.sum();

    sum_cal
}


pub fn day1(file_name: &str) -> (u32,u32) {

    (compute_sum(file_name, false), compute_sum(file_name,true))
	    
}
