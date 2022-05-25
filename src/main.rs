use std::iter::repeat;

fn sierpinski(order: usize) -> Vec<String> {
  let mut triangle = vec!["*".to_string()];
  for i in 0..order {
    let space = repeat(' ').take(2_usize.pow(i as u32)).collect::<String>();

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
  let order = std::env::args()
    .nth(1)
    .unwrap_or_else(|| "4".to_string())
    .parse::<usize>()
    .unwrap();

  if order < 1 {
    println!("Amount must be greater than 0!");
    return;
  };

  sierpinski(order).iter().for_each(|it| println!("{}", it));
}
