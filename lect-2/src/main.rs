fn main() {
    let line: &str = "aditya is a noice boi😀"; // this takes 6 bytes 

    let _: usize = line.len(); // no. of bytes used

    // for indexing a str since normal [0] is not possible 
    let _: &str = &line[0..2];
    // let slice: &str = &line[0..3]; // this throws error  

    // iterating over a string 
    for index in  line.as_bytes().iter() { 
        dbg!(index);
    }    

    for index in  line.chars() { 
        dbg!(index);
    }

    let mut state: String = String::new();

    state += "time package";
    println!("{}", state);
}
