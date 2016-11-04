#[crate_type="rlib"]
extern {
    fn square(x: i32) -> i32;
}

#[no_mangle]
#[export_name="square"]
pub extern fn _square(x: i32){
    x * x
}

//#[export_name="square"]
//pub extern fn _square(x: i32){
//    x * x
//}

//
//#[export_name="_triple"]
//pub extern "system" fn triple(x: i32) -> i32 {
//    x * x * x
//}

//#[export_name="\x01square"]

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//
//    }
//}
