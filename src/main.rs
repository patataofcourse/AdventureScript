use adventure_script;

fn main() {
    let mut game = adventure_script::create_game(String::from("test_game"), None);
    game.run();
}
