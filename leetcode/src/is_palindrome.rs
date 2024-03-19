impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
      let x_string: String = x.to_string();
      let mut char_vec: Vec<char> = x_string.chars().collect();
      char_vec.reverse();
      let reversed_string: String = char_vec.iter().collect();
      if reversed_string == x_string{
          return true;
      }
      else {
          return false;
      }
  }
}