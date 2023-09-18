# hstring_weapper_rs

 A wrapper around the raw rust bindings for my first attempt at a [C string library](https://github.com/largenumberhere/hstring). 
It's not particulary safe but at least it implements some rust design patterns, 
common traits and automatically frees on drop. 
 It's better than nothing but it's still not really memory safe because it's only about as memory safe
as my c library happens to be. 
### Safety: 
#### This naive library assumes...
- HSTRING will never be a non-null terminated string
- HSTRING.contents\[0  to HSTRING.length\] contains bytes that can be read, written to and will always be a valid Cstr.
- a HSTRING will never be modified from another context, thread, etc.
- HSTRING can be allocated and deallocated using `malloc` in the `HSITRNG` source code without conflicting with rust's allocator.
- HSTRING will never run out of memory to expand
- HSTRING will only be dropped once

... and so on.