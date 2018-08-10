extern crate yew;
extern crate yew_sample;

use yew::prelude::*;
use yew_sample::Model;

fn main() {
    println!("Hello, world!");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
