use std::iter::repeat;
use clap::Parser;

/// Generate ASCII Sierpiński triangles 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Character to use
  #[clap(short, long, default_value_t = '*')]
  base_character: char,

  /// Triangle iterations
  #[clap(default_value_t = 4)]
  amount: usize
}

/// Generate a sierpenski triangle of a certain magnitude.
/// Reccomended for order > 0
pub fn sierpinski(order: usize, character: char, space_char: char) -> Vec<String> {
  let mut triangle = vec![character.to_string()];
  for i in 0..order {
    let space = repeat(space_char).take(2_usize.pow(i as u32)).collect::<String>();

    // save original state
    let mut triangle_original = triangle.clone();

    // extend existing lines
    triangle_original.iter_mut().for_each(|r| {
        let new_row = format!("{}{}{}", space, r, space);
        *r = new_row;
    });

    // add new lines
    triangle.iter().for_each(|r| {
      let new_row = format!("{}{}{}", r, " ", r);
      triangle_original.push(new_row);
    });

    triangle = triangle_original;
  }

  return triangle;
}

fn main() {
  let args = Args::parse();

  let amount = args.amount;

  if amount < 1 {
    eprintln!("Amount must be greater than 0!");
    return;
  };

  sierpinski(amount, args.base_character, ' ').iter().for_each(|it| println!("{}", it));
}
