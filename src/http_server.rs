use std::path::Path;

use iron::Iron;
use iron::Listening;
use staticfile::Static;
use mount::Mount;


pub fn start_http(ip_port: &str) -> Listening {
    println!("Serving HTTP on: {}", ip_port);
    Iron::new(get_mount())
        .http(ip_port)
        .expect("starting HTTP server FAILED")
}

fn get_mount() -> Mount {
    // let views_handler = views::get_handler();
    //
    // let mut rest_chain = Chain::new(rest_api::rest_router());
    // rest_chain.link_before(rest_api::AuthToken);

    let mut mount = Mount::new();
    mount
        .mount(
            "/static",
            Static::new(Path::new("./static"))
        );
        // .mount("/gui-api/", gui_api::get_router())
        // .mount("/api", rest_chain)
        // .mount("/", views_handler);
    mount
}
