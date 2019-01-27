extern crate rand;

use rand::Rng;

fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let month = rand::thread_rng().gen_range(1, months.len());
    println!("The random month is: {}", months[month]);
}
