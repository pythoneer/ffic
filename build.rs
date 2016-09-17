extern crate gcc;

fn main() {
    gcc::compile_library("libfoo.a", &["csrc/foo.c"]);
}