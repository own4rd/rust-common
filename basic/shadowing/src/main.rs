fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of X in inner scope: {x}");
    }

    println!("Value of X is: {x}");
}