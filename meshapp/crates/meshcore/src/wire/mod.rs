//! On-air formats (docs/DESIGN.md §2): the 512-byte cell, the inner fragment
//! header, per-chain recognition tags, sealed first flights, the per-hop link
//! frame and the Noise NN link layer.

pub mod cell;
pub mod frame;
pub mod inner;
pub mod noise;
pub mod recog;
pub mod seal;

pub use cell::{BODY_LEN, CELL_LEN, Cell, CellReject};
pub use frame::Class;
