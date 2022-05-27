//! fvoid, a void file generator, supports many formats.
//!
//! # Example
//!
//! Generate a PDF file with 3 pages and default size (100 x 200):
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
pub use fmts::flv::VoidFLV;
pub use fmts::mp3::VoidMP3;
pub use fmts::pdf::VoidPDF;
pub use fmts::svg::VoidSVG;
pub use fmts::txt::VoidTXT;
