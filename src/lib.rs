#[no_mangle]
#[export_name="_square"]
pub extern "system" fn square(x: i32) -> i32 {
    x * x
}


//#[export_name="\x01square"]

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
