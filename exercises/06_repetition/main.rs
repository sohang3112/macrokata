////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! if_any {
    ($($condition:expr),+; $body: block) => {
        if $($condition)||+ $body
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}
