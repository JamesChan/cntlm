#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Config {}

#[warn(dead_code)]
fn foo_bar() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_def() {
        assert!(1 == 1);
    }
}
