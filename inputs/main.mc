export fn check(z) {
    let a = z.get_player("@a");
    let entity = new Entity(a.selector);
    entity.tp(a.entity);
}
export let x = new Scoreboard("b", "dummy");
