#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate image;

mod buffer;
mod hasher;
mod ops;
pub use ops::transform::Rotation;
mod opbasics;
mod pipeline;
pub use self::pipeline::*;
pub use self::ops::*;
pub mod color_conversions;
mod scaling;
pub use self::ops::curves::SplineFunc;

use std::path::Path;

pub fn simple_decode_8bit<P: AsRef<Path>>(img: P, maxwidth: usize, maxheight: usize) -> Result<SRGBImage, String> {
  let mut pipeline = Pipeline::new_from_file(&img)?;
  pipeline.globals.settings.maxwidth = maxwidth;
  pipeline.globals.settings.maxheight = maxheight;
  pipeline.output_8bit(None)
}
