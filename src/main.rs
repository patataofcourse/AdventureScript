use adventure_script;

fn main() {
    adventure_script::save::test_save();
    let mut game =
        adventure_script::AdventureScriptGame::new(String::from("test_game"), None, true);
    game.run();
}
