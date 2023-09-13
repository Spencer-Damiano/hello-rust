fn main(){
    let mut line = String::new();
    println!("Enter your name :");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
 }
