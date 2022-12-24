use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use rustronomy_fits::{Extension, Fits, HeaderDataUnit};
use ndarray::{Array, Ix3, IxDyn};

pub struct ParsedFitsFile {
    f: Fits
}

pub struct ParsedFitsFileHDU<'a> {
    hdu: &'a HeaderDataUnit
}

impl<'a> ParsedFitsFileHDU<'a> {
    pub fn gain(&self) -> f32 {self.hdu.get_header().get_value_as::<f32>("GAIN").unwrap()}
    pub fn offset(&self) -> f32 {self.hdu.get_header().get_value_as::<f32>("OFFSET").unwrap()}
    pub fn ra(&self) -> f32 {self.hdu.get_header().get_value_as::<f32>("RA").unwrap()}
    pub fn dec(&self) -> f32 {self.hdu.get_header().get_value_as::<f32>("DEC").unwrap()}
    pub fn epoch(&self) -> i32 {self.hdu.get_header().get_value_as::<i32>("EQUINOX").unwrap()}
    pub fn b_zero(&self) -> i32 {self.hdu.get_header().get_value_as::<i32>("BZERO").unwrap()}
    pub fn b_scale(&self) -> i32 {self.hdu.get_header().get_value_as::<i32>("BSCALE").unwrap()}
    pub fn bayer_pattern(&self) -> Option<&str> {self.hdu.get_header().get_value("BAYERPAT").map(|s| s.trim())}
    pub fn data_raw_i16(&self) -> Option<&Array<i16, IxDyn>> {
        if let Extension::Image(img) = self.hdu.get_data()? {
            return img.as_i16_array().ok();
        }
        return None;
    }

    pub fn data_copy_f32(&self, ) -> Option<Array<f32, IxDyn>> {
        self.data_raw_i16().map(|a| {
            a.map(|v| self.b_zero() as f32 + *v as f32)
        })
    }
}

impl<'a> Debug for ParsedFitsFileHDU<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.hdu)
            .and_then(|_|{writeln!(f, "{}", self.hdu.get_header())})
    }
}

impl ParsedFitsFile {
    pub fn parse(path: &Path) -> Result<ParsedFitsFile, Box<dyn Error>> {
        Fits::open(&path)
            .map(|f| ParsedFitsFile {f})
    }

    pub fn hdu(&self, idx: usize) -> Option<ParsedFitsFileHDU> {
        self.f.get_hdu(idx).map(|hdu| ParsedFitsFileHDU {hdu})
    }
}

impl Debug for ParsedFitsFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.f)
    }
}

