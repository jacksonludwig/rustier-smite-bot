use smite_api_library::json;

// TODO:
// Keep track of the time of each new query attempt,
// if it has been more than 15 min since last request,
// then get a new session id.

fn main() {
    let session = json::make_session().unwrap_or_else(|err| {
        panic!("There was an error creating the session: {:?}", err);
    });
    let req = json::create_link(
        "getplayer",
        &session.session_id,
        &json::get_formatted_time(),
    );
    println!("{}", req);
    println!("Timestamp: {}", session.timestamp);
}
