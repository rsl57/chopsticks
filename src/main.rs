mod game_objects;
mod io_handling;

fn main() {
    let state = game_objects::new_game();
    io_handling::display_game_state(state.as_tuple());
    let choice = io_handling::get_turn_choice();
}
