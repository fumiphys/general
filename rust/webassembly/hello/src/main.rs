#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello World!";
    js! {
        alert( @{message} );
        console.log( @{message} );
    }
    stdweb::event_loop();
}
