fn addition_table(n: u64) {
    println!("Addition Table (mod {}):", n);
    // Print the header row
    print!("    ");
    for i in 0..n {
        print!("{:4}", i);
    }
    println!();

    // Print the table
    for i in 0..n {
        print!("{:2} ", i);
        for j in 0..n {
            print!("{:4}", (i + j) % n);
        }
        println!();
    }
}

fn main() {
    let n = 15; // Example value
    addition_table(n);
}

