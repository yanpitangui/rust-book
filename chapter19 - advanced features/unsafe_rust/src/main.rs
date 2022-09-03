use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe fn dangerous() { }

    unsafe  {
        dangerous();
    }


    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    let (a, b) = split_at_mut_custom(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe  {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is {}", HELLO_WORLD);

    add_to_count(3);
    unsafe  {
        println!("COUNTER: {}", COUNTER);
    }

}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn split_at_mut_custom(slice: &mut[i32], mid: usize) -> (&mut [i32], &mut[i32]){
    let len = slice.len();

    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(
                ptr.add(mid), len - mid
            ),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C!");
}