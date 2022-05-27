use clap::Parser;

/// Generate ASCII Sierpiński triangles
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// base character to use for the triangle
    #[clap(short, long, default_value = "*")]
    base: String,

    /// Triangle iterations
    #[clap(default_value_t = 4)]
    amount: usize,
}

/// Generate a sierpenski triangle of a certain magnitude.
/// Reccomended for order > 0
pub fn sierpinski(order: usize, character: String, space: String) -> Vec<String> {
    let mut triangle: Vec<String> = vec![character];
    for i in 0..order {
        let upsized_space = space.as_str().repeat(2_usize.pow(i as u32));

        // save original state
        let mut triangle_original = triangle.clone();

        // extend existing lines
        triangle_original.iter_mut().for_each(|r| {
            let new_row = format!("{}{}{}", upsized_space, r, upsized_space);
            *r = new_row;
        });

        // add new lines
        triangle.iter().for_each(|r| {
            let new_row = format!("{}{}{}", r, " ", r);
            triangle_original.push(new_row);
        });

        triangle = triangle_original;
    }

    triangle
}

fn main() {
    let args = Args::parse();

    let amount = args.amount;

    if amount < 1 {
        eprintln!("Amount must be greater than 0!");
        return;
    };

    let triangle: String = sierpinski(amount, args.base, " ".to_string()).join("\n");

    println!("{}", triangle);
}
