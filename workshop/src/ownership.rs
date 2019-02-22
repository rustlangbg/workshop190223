pub fn take(v: Vec<i32>) {
    println!("v[10] + v[100] = {}", v[10] + v[100])
}

pub fn copy(a: i32, b: i32) {
    println!("{}", a + b);
}

pub fn re(v: Vec<i32>) -> Vec<i32> {
    println!("v[10] + v[100] = {}", v[10] + v[100]);
    v
}

// Mutable references are copied, immutable references are copied. When a reference is dropped, the borrow ends
fn borrow(v: &Vec<i32>) {
    println!("v[10] + v[100] = {}", v[10] + v[100]);
    println!("v[10] + v[100] = {}", (*v)[10] + (*v)[100]);
}

pub fn main() {
    // Each variable has a value and the variable itself is called the owner
    let x = 1; // x owns 1. In this case it's a literal, which is storted on the stack.

    // Each piece of data can have only one owner at a time. When the owner gets out of scope, the value will be dropped
    // {
    //     let a = 10;
    //     println!("a: {:?}", a);
    // }
    // println!("a: {:?}", a); // This will go wrong, because a is out of scope

    // let y = x;
    // println!("{:?}", y); // This works
    
    // let s = String::from("Hello there!");
    // let z = &s;
    // println!("s: {:?}, z: {:?}", s, z); // This doesn't work, because s is moved, to make this work we need to use a reference
    
    // let mut v = Vec::new();
    // for i in 1..1000 {
    //     v.push(i);
    // }

    //take(v); // Moving ownership to the take function
    //println!("{}", v[0]); // Won't work, because we moved v to the take function and never got ownership back
    
    // copy(x, y); // Primitive types are being copied by default
    
    // v = re(v); // Moving and returning ownership
    // println!("We still own v: {}", v[5]);
    
    // borrow(&v);
    // println!("We still own v: {}", v[5]);
}