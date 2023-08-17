use adventure_script::AdventureScriptGame;

use adventure_script::modules::inventory;

fn main() {
    let mut game = AdventureScriptGame::new(String::from("test_game"), None, true, true);
    game.add_module(inventory::get_module(Some("inv"))).unwrap();
    game.run();
}
