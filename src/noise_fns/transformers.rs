pub use self::{displace::*, rotate_point::*, scale_point::*, translate_point::*, turbulence::*};

mod displace;
mod rotate_point;
mod scale_point;
mod translate_point;
mod turbulence;

pub trait TransformerArgs {
  fn expand(self, default: f64) -> [f64; 4];
}

impl TransformerArgs for f64 {
  /// Produces an args list where all args are this value.
  #[inline]
  fn expand(self, _default: f64) -> [f64; 4] {
    [self; 4]
  }
}

impl TransformerArgs for [f64; 1] {
  /// Produces an args list where the first value (x) is filled
  /// from the array, the rest are default.
  #[inline]
  fn expand(self, default: f64) -> [f64; 4] {
    [self[0], default, default, default]
  }
}

impl TransformerArgs for [f64; 2] {
  /// Produces an args list where the first two values (x, y) are
  /// filled from the array, the rest are default.
  #[inline]
  fn expand(self, default: f64) -> [f64; 4] {
    [self[0], self[1], default, default]
  }
}

impl TransformerArgs for [f64; 3] {
  /// Produces an args list where the first three values (x, y, z)
  /// are filled from the array, the rest are default.
  #[inline]
  fn expand(self, default: f64) -> [f64; 4] {
    [self[0], self[1], self[2], default]
  }
}

impl TransformerArgs for [f64; 4] {
  #[inline]
  fn expand(self, _default: f64) -> [f64; 4] {
    self
  }
}
