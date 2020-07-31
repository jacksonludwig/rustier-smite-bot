use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

mod transformer;

fn main() {
    let session = json::make_session().unwrap();
    let builder = QueryBuilder::new(session.session_id);
    let god_list = transformer::make_god_list();
    for god in god_list {
        println!("Name: {}, id: {}", god.name, god.id);
    }
}
