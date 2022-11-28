extern "C" {
    pub fn hello(a: *mut u8, b: *mut u8);
}

mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        let mut a = "hello";
        let mut b = "world";

        unsafe { hello(&mut a as *mut _ as *mut u8, &mut b as *mut _ as *mut u8) }
    }
}
