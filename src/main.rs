use adventure_script;

fn main() {
    let mut game = adventure_script::AdventureScriptGame::new(
        String::from("../../PATATFAT/Projects/Tales of Tayron - A Burst of Hope/ToT"),
        None,
        true,
        true,
    );
    game.run();
}
