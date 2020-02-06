mod sum;

fn main() {
    let x = sum::sum(1,2);
    let a = String::from("a");
    {
        let b = a;
    }

    println!("sum {}", b);
}
