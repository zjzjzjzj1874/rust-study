mod base {
    const X: i32 = 10;

    pub fn print_test() {
        println!("print_test => x :{}", X);
    }
}

mod aaa {
    pub fn print_aaa(){
        printlbn!("print_aaa=> x: {}",X);
    }
}
