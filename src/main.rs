
//adding link stuff here?
extern {
    fn call();
}

fn main() {
    
    print!("try to invoce libfoo.call()");
    unsafe { call() };
    
}