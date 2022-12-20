fn main() {
    println!("Hello, world!");
    // fork
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // child
        println!("child");
        unsafe { libc::exit(0) };
    } else {
        // parent
        println!("parent");
        unsafe { libc::exit(0) };
    }
}
