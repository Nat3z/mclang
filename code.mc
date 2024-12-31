const x = 5;
let a = new Scoreboard("fortnite", "dummy");
let relative = a.get_player("@p");

relative.add(x);

if relative == 5 {
    relative.add(5);
}
