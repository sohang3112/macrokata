////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_pair<K: std::fmt::Debug, V: std::fmt::Debug>(pair: (K, V)) {
    println!("{pair:#?}");
}
fn print_hashmap<K: std::fmt::Debug, V: std::fmt::Debug>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! pair {
    ($i:expr => $e: expr) => {
        ($i, $e)
    };
}

macro_rules! hashmap {
    ( $($k:expr => $v:expr,)* ) => {
        {
            HashMap::from([
                $(pair!($k => $v)),*
            ])
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair: (char, u8) = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        String::from("Hash") => "map",
        String::from("Key") => value,
    );

    print_hashmap(&my_hashmap);
}
