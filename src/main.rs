use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

// TODO:
// Keep track of the time of each new query attempt,
// if it has been more than 15 min since last request,
// then get a new session id.

fn main() {
    test_loop();
}

fn test_loop() {
    let mut session = json::make_session().unwrap();
    let mut builder = QueryBuilder::new(session.session_id);
    loop {
        let time_dif = json::get_formatted_time().parse::<usize>().unwrap()
            - session.timestamp.parse::<usize>().unwrap();
        println!("Seconds from last query: {}", time_dif);
        if time_dif > 900 {
            session = json::make_session().unwrap();
            builder = QueryBuilder::new(session.session_id);
            println!("A new session was constructed");
        }

        println!("Type something to make the query");
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let json_data = builder.get_player("SwiggedySwoody").unwrap();

        println!("{}", json_data);
    }
}
