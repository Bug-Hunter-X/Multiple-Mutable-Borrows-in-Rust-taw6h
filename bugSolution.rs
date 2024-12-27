fn main() {
    let mut x = 5;
    { //creating a new scope
        let y = &mut x; 
        *y = 10;
        println!("x = {}", x);
    }
    { //creating a new scope
        let z = &mut x; 
        *z = 15;
        println!("x = {}", x);
    }
    
}