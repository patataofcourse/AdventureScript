use adventure_script;

fn main() {
    let game = adventure_script::create_game(String::from("test"), None);
    game.run();
}
