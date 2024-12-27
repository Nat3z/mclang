let a = new Entity("@a");
a.tp(new BlockPos(50, 50, 100));

let scoreboard = 0;
if scoreboard == 5 {
  while let x = [ new Entity("@a"), new Entity("@p"), new Entity("@s"), new Entity("@e") ] {
    x.kill();
    x.tp(new BlockPos(0, 0, 0));
    a.kill();
  }
}

if scoreboard == 6 {
  a.kill();
}
