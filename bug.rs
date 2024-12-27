fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; //this will cause an error because of multiple mutable borrows
    *y = 10;
    *z = 15;
    println!("x = {}", x);
}