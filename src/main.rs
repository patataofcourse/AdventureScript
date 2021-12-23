use adventure_script;

fn main() {
    let mut game =
        adventure_script::AdventureScriptGame::new(String::from("test_game"), None, true, true);
    game.run();
}
