// This function is public and can be called using othe modules
pub fn hello(name: &str) {
    println!("I am in my_module, Hello {}!", name);
    hello_private();
}

// This function is private and can not be called from outer module
fn hello_private() {
    println!("This printing is from a private function in the my_module")
}