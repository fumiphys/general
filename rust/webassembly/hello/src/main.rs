#[macro_use]
extern crate stdweb;

use stdweb::web::*;
use stdweb::web::event::ClickEvent;

fn main() {
    stdweb::initialize();

    let message = "Hello World!";
    js! {
        alert( @{message} );
        console.log( @{message} );
    }

    if let Ok(body) = document().query_selector("body") {
        let body = body.unwrap();
        body.add_event_listener(move |_: ClickEvent| {
            js! {
                console.log("clicked");
            }
        });
        let message = document().create_text_node(message);
        body.append_child(&message);
    } else {
        println!("cannot find body");
        return;
    }

    stdweb::event_loop();
}
