let x = 5;
let a = new Scoreboard("a", "dummy");
let relative = a.get_player("@p");

if x == relative {
    let entity = new Entity("@a");
    entity.kill();
}

