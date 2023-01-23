// create main function that calls function from lib.rs file
mod lib;
use lib::data;

fn main() {
    let (x, z) = data();
    let mut fg = Figure::new();
    fg.axes2d().lines(&x, &y, &[Caption("Data")]);
    fg.show();
}
