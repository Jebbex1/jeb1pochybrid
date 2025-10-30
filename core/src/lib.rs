pub(crate) mod modone;

use modone::add_one;

pub fn add(mut a: i32, b: i32) -> i32 {
    for _ in 0..b {
        a = add_one(a);
    }
    a
}