use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

// TODO:
// Keep track of the time of each new query attempt,
// if it has been more than 15 min since last request,
// then get a new session id.

fn main() {
    let session = json::make_session().unwrap_or_else(|err| {
        panic!("There was an error creating the session: {:?}", err);
    });

    let builder = QueryBuilder::new(session.session_id, session.timestamp);
    let json_data = builder.get_player("SwiggedySwoody").unwrap();
    println!("{}", json_data);
}
