use std::ffi::{c_char, c_int};
use hstring;
use hstring::{hstring_new, hstring_print, hstring_push_string, hstring_push_string_raw};
use crate::lib::Rhstring;

mod lib;

fn main() {

    // Try to cause a memory error by frequent allocations and frees. Don't actually use the hstirng like this! They are perfect for re-use
    let mut hstring = Rhstring::new();
    for i in 0..100 {
        hstring.push_str("h");
        println!("{:#?}", hstring);
    }
    hstring.clear();
    for i in 0..100 {
        hstring.push_str("h");

        let mut rhs2 = hstring.clone();
        println!("1: {:#?}", hstring);
        println!("2: {:#?}", rhs2);

        assert_eq!(hstring, rhs2);

        rhs2.push_str("aaa");
        assert_ne!(hstring, rhs2);

    }

}
