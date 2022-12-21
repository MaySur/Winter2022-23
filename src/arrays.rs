pub fn run(){

    let mut nums: [i32; 5] = [1,2,3,4,5];
    nums [2] = 23;
    println!("{:?}",nums );
    println!("{}",nums[0]);

    println!("Lenght: {}", nums.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&nums));

    let slice: &[i32] = &nums[0..2];
    println!("Slice: {:?}", slice);

}