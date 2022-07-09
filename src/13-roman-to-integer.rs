use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
		let roman_to_int_mappings: HashMap<char, i32> = HashMap::from([
				('I', 1),
				('V', 5),
				('X', 10),
				('L', 50),
				('C', 100),
				('D', 500),
				('M', 1000),
		]);

		let mut sum: i32 = 0;
		let roman_nums: Vec<char> = s.chars().collect();
		let total_roman_nums = roman_nums.len();


		for i in 0..total_roman_nums {
				let roman_num_value = roman_to_int_mappings.get(&roman_nums[i]).unwrap();

				if i == total_roman_nums - 1 || roman_num_value >= roman_to_int_mappings.get(&roman_nums[i + 1]).unwrap() {
						sum += roman_num_value;
				} else {
						sum -= roman_num_value;
				}
		}

		sum
}
