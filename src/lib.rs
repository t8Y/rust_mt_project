#[no_mangle]
pub extern "stdcall" fn square(x: i32) -> i32 {
    x * x
}

//pub extern fn square(x: i32) -> i32 {
//    x * x
//}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//
//    }
//}
