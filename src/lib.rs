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
pub use fmts::void_bin::VoidBIN;
pub use fmts::void_flv::VoidFLV;
pub use fmts::void_m3u::VoidM3U;
pub use fmts::void_mp3::VoidMP3;
pub use fmts::void_mts::VoidMTS;
pub use fmts::void_pdf::VoidPDF;
pub use fmts::void_svg::VoidSVG;
pub use fmts::void_wav::VoidWAV;
#[path = "fmts"]
mod fmts {
    pub mod void_bin;
    pub mod void_flv;
    pub mod void_m3u;
    pub mod void_mp3;
    pub mod void_mts;
    pub mod void_pdf;
    pub mod void_svg;
    pub mod void_wav;
}
// starts with `void_` because some of ext like `7z` is not a legal identifier
