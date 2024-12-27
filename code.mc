while let x = [ new Entity("@a"), new Entity("@p"), new Entity("@s"), new Entity("@e") ] {
  x.kill();
  x.tp(new BlockPos(0, 0, 0));
}
