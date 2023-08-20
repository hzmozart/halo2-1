#[link(name = "hello", kind = "static")]
extern "C" {
    pub fn helloMain(a: *mut u8, b: *mut u8) -> i32;
}

mod tests {
    use super::helloMain;

    #[test]
    fn test_hello() {
        let mut a = "hello";
        let mut b = "world";

        unsafe { let _ = helloMain(&mut a as *mut _ as *mut u8, &mut b as *mut _ as *mut u8); }
    }
}
