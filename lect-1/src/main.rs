fn main() {
    // printing something
    println!("Hello, world!");

    // data types 
    let x: i32 = 20;  
    let y: u8 = 10; 

    // functions 
    fn function(n: i32) -> bool {
        return n % 2 == 0  
    }

    println!("{}, {}, {}",x,y, function(x));
}
