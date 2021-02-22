pub trait Clocked {
  fn rise_edge(&mut self);
  fn fall_edge(&mut self);
}
