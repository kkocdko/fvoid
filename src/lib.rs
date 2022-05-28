//! fvoid, a void file generator, supports many formats.
//!
//! # Example
//!
//! Generate a blank PDF file with 3 pages and default size (100 x 200):
//!
//! ```rust
//! use fvoid::VoidPDF;
//! let pdf_cfg = VoidPDF {
//!     page_count: 3,
//!     ..Default::default()
//! };
//! let pdf_data = pdf_cfg.data();
//! // std::fs::write("void.pdf", pdf_data).unwrap();
//! ```
mod fmts;
pub use fmts::bin::VoidBIN;
pub use fmts::flv::VoidFLV;
pub use fmts::m3u::VoidM3U;
pub use fmts::mp3::VoidMP3;
pub use fmts::mts::VoidMTS;
pub use fmts::pdf::VoidPDF;
pub use fmts::svg::VoidSVG;
pub use fmts::wav::VoidWAV;
