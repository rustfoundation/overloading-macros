#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features, unused_braces)]

use splat_overload::overload;

struct Reader;

overload! {
    unsafe fn read(ptr: *const i32) -> i32 { unsafe { *ptr } }
    unsafe fn read(ptr: *const i32, offset: usize) -> i32 { unsafe { *ptr.add(offset) } }
}

overload! {
    impl Reader {
        unsafe fn get(&self, ptr: *const i32) -> i32 { unsafe { *ptr } }
        unsafe fn get(&self, ptr: *const i32, offset: usize) -> i32 { unsafe { *ptr.add(offset) } }
    }
}

fn main() {
    let x = 42;
    let ptr = &raw const x;
    // Missing unsafe block
    read(ptr);
    read(ptr, 0);
    Reader.get(ptr);
    Reader.get(ptr, 0);
}
