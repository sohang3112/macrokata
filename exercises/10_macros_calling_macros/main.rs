////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;
use std::fmt::Debug;

fn print_pair<K: Debug, V: Debug>(pair: (K, V)) {
    println!("{pair:#?}");
}
fn print_hashmap<K: Debug, V: Debug>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! pair {
    ($fst:expr => $snd:expr) => {
        ($fst, $snd)
    };
}

macro_rules! hashmap {
    ($($pair:tt),+) => {
        HashMap::from([
            $( pair!($pair) ),+
        ])
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
