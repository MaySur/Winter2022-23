pub fn run(){
    let mut hello = String ::from("Hello ");

    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str("ombats");

    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());

    println!("Contains 'Wombats' {}", hello.contains("Wombats"));
    println!("Replace: {}", hello.replace("Wombats", "Human"));

    let mut count = 0;
    for i in hello.split_whitespace(){        
        count = count+1;
        println!("Count: {} Word: {}", count,i);
    }

    let mut str = String::with_capacity(10);
    str.push('a');
    str.push('b');
    str.push('c');
    
    assert_eq!(3,str.len());
    assert_eq!(10,str.capacity());





    println!("{str}");

    //println!("{}", hello);
}