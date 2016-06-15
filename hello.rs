fn main() {
    println!("Hello World");
    println!("I'm a Rustacean!");

    println!("{} days",30);

    println!("{0}, {1}, {0}, {1}", "Alice", "Bob");
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    println!("Pi is roughly {pi:.2}", pi=22.0/7.0);
}
