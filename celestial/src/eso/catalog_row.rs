use std::fmt;
use std::fmt::Formatter;

pub struct CatalogRow {
    row: Vec<String>
}

impl CatalogRow {

    pub fn new(row: Vec<String>) -> CatalogRow {
        CatalogRow {
            row
        }
    }

    pub fn tyc_id(&self) -> String {
        return self.row[0]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .join("-").to_string();
    }

    pub fn ra_deg(&self) -> f32 {
        return self.row[2].trim().parse::<f32>().unwrap_or(f32::NAN)
    }
    pub fn dec_deg(&self) -> f32 {
        return self.row[3].trim().parse::<f32>().unwrap_or(f32::NAN)
    }
    pub fn bt_mag(&self) -> f32 {
        self.row[17].parse::<f32>().unwrap_or(f32::NAN)
    }

    pub fn vt_mag(&self) -> f32 {
        self.row[19].parse::<f32>().unwrap_or(f32::NAN)
    }
}

impl fmt::Debug for CatalogRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
            "TYC:{:8} ra:{:10} dev:{:10}, bt:{:10}, vt:{:10}",
            self.tyc_id(),
            self.ra_deg(),
            self.dec_deg(),
            self.bt_mag(),
            self.vt_mag()
        )
    }
}
