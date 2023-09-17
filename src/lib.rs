use std::error::Error;
use std::ffi::{c_char, c_int, CStr};
use std::fmt::{Debug, Formatter};
use std::str::Utf8Error;
use hstring::{HSTRING, hstring_clear, hstring_free, hstring_new, hstring_push_string_raw};

pub struct Rhstring {
    inner: HSTRING
}

impl Rhstring{

    //Add the contents of a str to the underlying hstring
    pub fn push_str(&mut self, value: &str) {
        // Get the inner hstring reference and cast it to a pointer
        let mut hs: *mut HSTRING = &mut self.inner;

        // Convert the given str to its pointer and length
        let len = value.len();
        let ptr = value.as_ptr();

        // Give the pointers and length to the external function
        unsafe {
            hstring_push_string_raw(hs, ptr as *mut c_char, len as c_int);
        }
    }

    //Copy the contents to a String, free,  and return the string.
    //May fail if the contents aren't valid utf8
    pub fn to_rust_string(mut self)-> Result<String, Box<dyn Error>>{
        //Grab the inner hstring
        let mut hs = self.inner;

        //Make a slice from contents
        let slice = core::ptr::slice_from_raw_parts(hs.contents, hs.length);
        let slice = slice as *const [u8];

        //Convert the contents from `i8/c_char`s to `u8`s
        let slice = unsafe{
            &*slice
        };

        //Turn that slice into a str slice. It is still just a reference to the same memory, but now rust knows its valid
        let str = core::str::from_utf8(slice)?;

        //Copy the contents out
        let string = String::from(str);

        //Release the memory for the inner hstring. This calls hstring_free
        core::mem::drop(self);

        Ok(string)
    }

    ///Create a hstring.
    ///It immediately allocates 1 bytes on the heap to maintain backwards compatability with C apis that always expect a null terminator in a string.
    pub fn new() -> Rhstring{
        Rhstring{
            inner: unsafe { hstring_new() }
        }
    }

    //Sets length to 0.
    //Does not shrink for performance reasons
    pub fn clear(&mut self){
        unsafe {
            hstring_clear(&mut self.inner);
        }
    }

    //Get a reference to inner string and check if it's valid utf8, else error.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        let hs = & self.inner;

        //Make a slice from contents
        let slice = core::ptr::slice_from_raw_parts(hs.contents, hs.length);
        let slice = slice as *const [u8];

        //Convert the contents from `i8/c_char`s to `u8`s
        let slice = unsafe{
            &*slice
        };

        //
        let st = core::str::from_utf8(slice)?;
        Ok(st)
    }

    // Copy the contents of a &str into the Rhstirng
    pub fn from_str(st: &str) -> Rhstring{
        let mut hstring = unsafe{ hstring_new() };
        let ptr = st.as_ptr();
        unsafe {
            hstring_push_string_raw(&mut hstring, ptr as *mut c_char, st.len() as c_int)
        }

        Rhstring{
            inner: hstring
        }
    }

    // View the contents as a cstr.
    pub fn as_cstr(&self) -> &CStr{
        let hs = &self.inner;
        unsafe{ std::ffi::CStr::from_ptr(hs.contents) }
    }
}


impl Drop for Rhstring{
    /// The inner hstring must be manually deallocated because it is written in C. This drop impl takes care of that
    fn drop(&mut self) {
        unsafe {
            hstring_free(&mut self.inner);
        }
    }
}

impl Debug for Rhstring{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rhstring")
            .field("inner.length",&self.inner.length)
            .field("inner.capacity", &self.inner.capacity)
            .field("inner.contents(ptr)",&self.inner.contents)
            .field("innter.contents.as_str()", &self.as_str())
            .finish()
    }
}