#[no_mangle]
#[export_name="_test"]
pub extern "system" fn test(){
    print!("hello world!");
}

#[export_name="_triple"]
pub extern "system" fn triple(x: i32) -> i32 {
    x * x * x
}

//#[export_name="\x01square"]

//pub extern fn square(x: i32) -> i32 {
//    x *  x
//}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//
//    }
//}
