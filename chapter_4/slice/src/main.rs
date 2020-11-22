fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  /*
  iter returns each element in a collection
  enumerate wraps results of iter in a tuple
  Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
  */
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }
  s.len()
}

/*
  Refactor includes returning a string slice
*/
fn first_word_refactor(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i]
    }
  }
  &s[..]
}

fn main() {
  let mut word = String::from("Hello Peter Bjorn");
  let index = first_word(&word);
  println!("First word ends at index: {}", index);
  word.clear();
  // Index is now out of sync
  println!("First word ends at index: {}", index);

  
  // Slice is reference to part of the object
  let word = String::from("This is a test sentence.");
  let first = &word[0..4];
  let second = &word[5..8];
  let also_first = &word[..4];
  println!("first, second, {}, {}", first, second);
  println!("first, also_first, {}, {}", first, also_first);

  // Slices are immutable references
  let str_slice_example = first_word_refactor(&word);
  println!("str_slice_example: {}", str_slice_example);

  // Slices can be used on parts of an array too
  let scores: [i32; 10] = [39, 59, 78, 92, 29, 40, 59, 39, 39, 19];
  let _array_slice = &scores[..3];
}
