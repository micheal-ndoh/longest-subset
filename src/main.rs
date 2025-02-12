use std::io::{self, Write};

use set::Subset;

fn main() -> io::Result<()> {
    loop {
        println!("Enter numbers separated a spaces (or 'q' to quit):");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "q" {
            break;
        }

        let nums: Vec<i32> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        if nums.is_empty() {
            println!("Enter again and seperate numbers by spaces. 'q' to quit:)");
            continue;
        }

        let mut subset = Subset::new(nums.clone());
        let result = subset.subsets_dup();

        println!("Input: {:?}", nums);
        println!("Subsets: {:?}", result);
    }

    Ok(())
}
mod set;

