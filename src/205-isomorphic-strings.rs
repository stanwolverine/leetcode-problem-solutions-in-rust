use std::collections::HashMap;

fn get_set_char_value(char_mapping: &mut HashMap<char, u32>, char_in_concern: &char, tracker: &mut u32) -> u32 {
  let char_value: u32;
  
  match char_mapping.get(char_in_concern) {
    None => {
      char_mapping.insert(*char_in_concern, *tracker);
      char_value = *tracker;
      *tracker += 1;
    },
    Some(T) => {
      char_value = *T;
    },
  }
  
  return char_value;
}

fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_char_mapping: HashMap<char, u32> = HashMap::new();
        let mut t_char_mapping: HashMap<char, u32> = HashMap::new();
      
        let mut s_tracker: u32 = 0;
        let mut t_tracker: u32 = 0;
      
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
      
        let len = s_chars.len();
      
        for i in 0..len {
          let s_char = s_chars[i];
          
          let s_char_value: u32 = get_set_char_value(&mut s_char_mapping, &s_char, &mut s_tracker);

          let t_char = t_chars[i];
          
          let t_char_value: u32 = get_set_char_value(&mut t_char_mapping, &t_char, &mut t_tracker);
          
          if (s_char_value != t_char_value) {
            return false;
          }
        }
      
        return true;
  }
