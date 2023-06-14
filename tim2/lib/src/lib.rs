//! # tim2
//! 
//! An image loader for TIM2 (.tm2) image files

///
/// ```
/// fn main() {
///     let image = tim2::load("../assets/test.tm2").unwrap();
/// 
///     /* print the header info for each frame found */
///     for (i, frame) in image.frames().iter().enumerate() {
///         println!("frame[{}]: <{}  {}>", i, frame.header().width(), frame.header().height());
///     }
/// }
/// ```

mod common;
mod error;
mod frame;
mod image;
mod pixel;

pub use error::*;
pub use frame::*;
pub use image::*;
pub use pixel::*;
