#[macro_use]
extern crate stdweb;

use stdweb::web::*;

fn main() {
    stdweb::initialize();

    let message = "Hello World!";
    js! {
        alert( @{message} );
        console.log( @{message} );
    }

    if let Ok(body) = document().query_selector("body") {
        let body = body.unwrap();
        let message = document().create_text_node(message);
        body.append_child(&message);
    } else {
        println!("cannot find body");
        return;
    }

    stdweb::event_loop();
}
