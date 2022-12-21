pub fn run(){
    //Print to console 
    println!("Checking in --> print.rs");
    println!("{}", 1.01);
    println!("{} is from {}", "Josh", "Earth");
    println!("{0} is fun but {1} is painfuly so {2} is better than {1} ", "Programming","Java", "Python");
    println!("{name} likes to listen to {genre}", name = "Josh", genre = "Rock");
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", (10+10));
}