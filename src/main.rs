use smite_api_library::json;

fn main() {
    let session = json::make_session().unwrap();
    let req = json::create_link(
        "getplayer",
        &session.session_id,
        &json::get_formatted_time(),
    );
    println!("{}", req);
    println!("Timestamp: {}", session.timestamp);
}
