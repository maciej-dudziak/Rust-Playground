#[allow(unused_variables)]
//Global variable has "static" keyword in front of it
static HELLO_WORLD: &str = "Hello, world!";
//mutable static variable
static mut COUNTER: u32 = 0;
//any code that reads or wrties from COUNTER must be within an unsafe block!
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");
    println!("name is: {}", HELLO_WORLD);
    //Dereferencing a raw pointer
    //Raw pointer type: <*const T>, <*mut T>, where "*" is part of the type name!
    //we can create raw pointers in safe code - we just cant dereference raw pointers outside
    //an unsafe block
    //we use "as" casting to create raw pointers form references
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32; 
    //create raw pointer to an arbitrary location in the memory
    let address = 0x012345usize;
    let _r = address as *const i32;
    //we can create raw pointer in the safe code - we just cant dereference it and read the data being pointed to
    //to do so we need to use UNSAFE block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

use std::slice;
//example of safe function that need to implement unsafe operations - there is real method split_as_mut, here we do it as function for simplicity
fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //use len method on slice to get its length
    let len = slice.len();
    //use as_mut_ptr() method on slice to get raw pointer of a slice
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    //function "slice::from_raw_parts_mut" is unsafe because it takes raw pointer and must trust that this pointer is valid
    //the slice::from_raw_parts_mut function takes a raw pointer and a length, and it creates a slice. 
    //We use this function to create a slice that starts from ptr and is mid items long
    //. Then we call the offset method on ptr with mid as an argument to get a raw pointer that starts at mid, 
    //and we create a slice using that pointer and the remaining number of items after mid as the length.
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
    //we created safe API around unsafe behaviour (created pointers are valid for sure and we are asserting mid value)
    //therefore we can use this function is safe code and we ensured that all unsafe operations are valid!
}
//using extern functions to call external code - in this case from C
extern "C" {
    fn abs(input: i32) -> i32;
}

//unsafe traits
//trait is unsafe when at least one of its methods has some invariant that the compiler can't varify
unsafe trait Foo {
    //methods go here
}
//by using unsafe impl, we're promising that we'll uphold the invariants that the compiles can't verify
unsafe impl Foo for i32 {
    //method implementations go here
}
//As an example, recall the Sync and Send marker traits we discussed in the “Extensible Concurrency 
//with the Sync and Send Traits” section in Chapter 16: the compiler implements these traits automatically 
//if our types are composed entirely of Send and Sync types. If we implement a type that contains a type 
//that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe. 
//Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; 
//therefore, we need to do those checks manually and indicate as such with unsafe.