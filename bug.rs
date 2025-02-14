fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; // This is okay
    }
    // v is still valid, so next line should also be okay
    println!("v[0] = {}", v[0]);
}