export fn check(z) {
    let a = z.get_player("@a");
    let entity = new Entity(a.selector);
    entity.tp(new BlockPos(0, 0, 0));
}
export let x = new Scoreboard("b", "dummy");
