pub fn run(){

    let x = 1;
    let y = 2.3;
    let Z: i64 = 43434343434343434;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    
    let is_greater = 10 > 5;

    let a1 = 'a';
    println!("{:?}", (x,y,Z, is_active, is_greater, a1));
}