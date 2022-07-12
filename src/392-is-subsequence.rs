use std::collections::VecDeque;


pub fn is_subsequence(s: String, t: String) -> bool {
		let mut s_chars: VecDeque<char> = s.chars().collect();

		let t_chars: Vec<char> = t.chars().collect();


		// handling edge cases
		{
			let s_len = s_chars.len();
			let t_len = t_chars.len();

			// case 1: both have 0 length
			if (s_len == 0 && t_len == 0) {
				return true;
			}

			// case 2: t.length is smaller than s.length
			if (s_len > t_len) {
				return false;
			}
		}

		for t_char in &t_chars {
			match s_chars.get(0) {
				None => {
					return true;
				},
				Some(T) => {
					if (*T == *t_char) {
						s_chars.pop_front();

						if (s_chars.len() == 0) {
							return true;
						}
					}
				}
			}
		}

		return false;
}
