fn main() {
    println!("Hello, world!");
    fibonacci(10);
}
fn fibonacci(x: u32){
    let mut c1: u32 = 1;
    let mut c2: u32 = 1;
    let mut aux: u32;
    print!("1 -> 1 -> ");
    for k in 1..x-1 {
        aux = c1 + c2;
        if k < x-2{
            print!("{} -> ", aux);
        } else {
            print!("{}", aux);
        }
        
        c1 = c2;
        c2 = aux; 
    }
    println!("");
}