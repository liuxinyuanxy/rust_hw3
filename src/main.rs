use std::collections::HashMap;
macro_rules! hash_map {
    ($($key:expr => $val:expr), *) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

pub mod myrc;
pub mod mystack;

fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", map);
}
