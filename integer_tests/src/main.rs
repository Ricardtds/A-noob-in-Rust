fn main() {
    compound();
}

fn integer_test(){
    let n1: i8 = 127;
    println!("{}", n1);
}

fn boolean(){
    println!("Teste");
}

fn character(){
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("{}\n{}\n{}", c, z, heart_eyed_cat);
}

fn compound(){
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}\n{}\n{}", five_hundred, six_point_four, one);
}

fn floating(){
    let x = 2.0; //f64
    let y:f32 = 3.0; //f32
    println!("O f64 vale: {}\nO f32 vale: {}", x, y);
}

fn operations(){
        //adiction
        let sum = 5 + 10;
        println!("Sum: {}", sum);
    
        //subtraction
        let difference = 95.5 - 4.3;
        println!("Difference: {}", difference);
    
        //multiplication
        let product = 4 * 30;
        println!("Product: {}",product);
    
        //divison
        let quotient = 56.7 / 32.2;
        let floored = 2/3; // Results in 0
        println!("Quotient: {}\nFloored: {}", quotient, floored);
    
        //remainder
        let remainder = 43 % 5;
        println!("Remainder: {}", remainder);
}