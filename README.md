# **RUST**

## 1.  Install: 

* [RUST LINK](https://www.rust-lang.org/tools/install)

***

## 2. Complie a Rust Program:

```
rustc <file-name>.rs
```
* Creates a binary file
```
./<file-name>
```
* Runs the program
***

## 3. Streamlined Way:
```
cargo  init
>>    Created binary (application) package
cargo run
```
* cargo init --> initializes the folder  
  * Creates a src folder and a Cargo.toml

  ```
  ----------Cargo.toml--------------
    [package]
    name = "Winter2022-23"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
  ```
  
* Code goes inside the `src ` folder
* cargo run --> runs and complies the program without creating a binary file
    * Creates the target folder
* cargo build --> To just build it out not run it
* cargo build --release --> to build for production
    * creates the realse folder where the executable is.

***

## 4. Println! Syntax:
```
----print.rs----
pub fn print(){
  println!("Hello Human");

  println!("{}", 1);

  println!("{} is from {}", "Josh", "Earth");

  println!("{0} is fun but {1} is painfuly so {2} is better than {1} ", "Programming","Java", "Python");

  println!("{name} likes to listen to {genre}", name = "Josh", genre = "Rock");

  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

  println!("{:?}", (12, true, "hello"));
  
  println!("10 + 10 = {}", (10+10));



}

``` 

``` 
------main.rs------
mod print;

fn main() {
    print::run();
}

``` 
* pub --> like public in java
* fn --> like def in python
* println!("Hello Human") --> like print in python but only for string
* println!("{}", 1); --> for anything other than string
* println!("{} is from{}", "Mayur", "Earth")--> {} are place holders and can be any type 
* println!("{0} is fun but {1} is painfuly so {2} is better than {1} ", "Programming","Java", "Python"); --> Positional arguments
* println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10); --> placeholders
* println!("{:?}", (12, true, "hello")); --> Placeholder for debug trait or for tuples
* println!("10 + 10 = {}", (10+10)); --> Basic Math
***



## 5. Variables:
```
-----vars.rs-----
pub fn run(){
    let name = "Joash";
    let mut age = 12;
    age = 90;
    // name = "Kory"
   
    println!("They're name is {name} and is {age} years old");
    const ID: i32 = 001;
    println!("{}", ID)

    let (my_name, my_age) = ("Tob", 123);
    println!("{} is {}!", my_name, my_age );

}
```

* Variables are immutable so it is not possible to assign a already defined variable another thing
* Can be mutable if `mut` keyword is specified like  `let mut age = 12;`

## 6.	Data Types:
*	Primitive Types :-
    * Integers: u8, i8, u16, u32, i32, u64, i64, u128, i128 (number of bits)
    * Floats: f32, f64
    * Boolean: bool
    * Characters: char
    * Tuples
    * Arrays

```
------types.rs---------
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


```

## 7.	Strings:
* Primitive Str = Immutable fixed-length string somewhere in memory 
* String = Growable, heap-allocated data structure - Use when you need to modify or won string data
```
pub fn run(){
    let hello = String ::from ("Hello");

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
    
    assert_eq!(2,str.len());
    assert_eq!(10,str.capacity());

    println!("{}", hello);

} 
```
* In order to use the build in string fucntions there needs to be `let hello = String ::from ("Hello");` format
* <variable_name>.len() --> Length of the string variable
* <variable_name>.push('A') --> Appends the char to the string
* <variable_name>.push_str("Java") --> Appends the string to the string
* <variable_name>.capacity() --> Finds the capacity in bytes
* <variable_name>.empty() --> Return type bool if empty or not
* <variable_name>.contains("Java") --> Finds the substring specified
* <variable_name>.replace("Java", "Python") --> Replaces the specifed substring
* asser_eq!(2, str.len()); --> Checks if both parameters are equal 
* [More Built in String fucntions](https://doc.rust-lang.org/std/string/struct.String.html)

## 8.	Tuples:
* Tuples is a list that can contain data that are multiple different data types
* Max 12 elememts
```
pub fn run() {

    let person: (&str, &str, i8) = ("Broom", "Sweep", 43);
    println!("{} has been {}ing for {} years now", person.0, person.1,person.2);
}
```  

## 9.	Arrays:
* Data type should always be the same
* Must have specifed lenght and cannot exced it or less than the length
```
pub fn run(){

    let nums: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",nums);
    println!("Lenght: {}", nums.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&nums));

    let slice: &[i32] = &nums[0..2];
    println!("Slice: {:?}", slice);

}
``` 
* [i31;5] --> the data type followed by the length
* in order to print the return type should be ambigious
* [0..2] --> will slice or take from the left index to right index

## 8.	Tuples:
* Tuples is a list that can contain data that are multiple different data types
* Max 12 elememts
```
<link rel="stylesheet" href="{% static '<app_name>/styles.css' %}">
```  

## 8.	Tuples:
* Tuples is a list that can contain data that are multiple different data types
* Max 12 elememts
```
<link rel="stylesheet" href="{% static '<app_name>/styles.css' %}">
```  





