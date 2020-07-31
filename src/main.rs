use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

fn main() {
    // test_loop();
    let session = json::make_session().unwrap();
    let builder = QueryBuilder::new(session.session_id);
    let data = builder.get_gods();
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

        let json_data = builder.get_player(&answer).unwrap();

        println!("{}", json_data);
    }
}
