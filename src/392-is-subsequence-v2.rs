fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let total_s_chars = s_chars.len();

    if (total_s_chars == 0) {
      return true;
    }

    let t_chars: Vec<char> = t.chars().collect();
    let total_t_chars = t_chars.len();

    if (total_s_chars > total_t_chars) {
      return false;
    }

    let mut match_count = 0;

    for t_char in &t_chars {
      if (*t_char == s_chars[match_count]) {
        match_count += 1;

        if (match_count == total_s_chars) {
          return true;
        }
      }
    }

    return false;
}
