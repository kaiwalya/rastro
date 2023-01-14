use std::io::{Read};
use csv_async::AsyncReader;

use tokio_util::compat::{Compat};
use url;

pub struct ESA {
    url: url::Url
}

pub struct Gaia {
    url: url::Url
}

pub struct DR3 {
    url: url::Url
}
pub struct GaiaSource {
    url: url::Url
}

pub struct GaiaSourceFile {
    pub hash: String,
    pub url: url::Url
}

pub struct GaiaSourceFileRow {
    pub source_id: String,
    pub phot_g_mean_mag: f32
}

pub struct Photometry {
    url: url::Url
}

pub struct EpochPhotometry {
    url: url::Url
}

pub struct EpochPhotometryFile {
    pub hash: String,
    pub url: url::Url
}

pub struct EpochPhotometryFileRow {
    pub source_id: String,
    pub g_transit_mag: Vec<f32>
}

pub trait DataDirectory {
    type FileType;
    fn listing_file_url(&self) -> url::Url;
    fn list_from_listing_file_contents(&self, contents: Vec<u8>) -> Vec<Self::FileType>;
}

impl ESA {
    pub fn new() -> ESA {
        return ESA {
            url: url::Url::parse("https://cdn.gea.esac.esa.int").unwrap()
        };
    }

    pub fn gaia(&self) -> Gaia {
        return Gaia::new(self);
    }
}

impl Gaia {
    fn new(esa: &ESA) -> Gaia {
        return Gaia {
            url: esa.url.join("Gaia/").unwrap()
        };
    }

    pub fn dr3(&self) -> DR3 {
        return DR3::new(self);
    }
}

impl DR3 {
    fn new(gaia: &Gaia) -> DR3 {
        return DR3 {
            url: gaia.url.join("gdr3/").unwrap()
        };
    }

    pub fn gaia_source(&self) -> GaiaSource {
        return GaiaSource::new(self);
    }

    pub fn photometry(&self) -> Photometry {
        return Photometry::new(self);
    }
}

impl GaiaSource {
    fn new(dr3: &DR3) -> GaiaSource {
        return GaiaSource {
            url: dr3.url.join("gaia_source/").unwrap()
        }
    }
}
impl DataDirectory for GaiaSource {
    type FileType = GaiaSourceFile;

    fn listing_file_url(&self) -> url:: Url {
        return self.url.join("_MD5SUM.txt").unwrap();
    }

    fn list_from_listing_file_contents(&self, contents: Vec<u8>) -> Vec<Self::FileType> {
        return String::from_utf8(contents).unwrap().lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .map(|cols| GaiaSourceFile::new(
                self.url.join(cols[1]).unwrap(),
                cols[0].to_string()
            ))
            .collect();
    }
}

pub struct GaiaSourceFileIterator<'a> {
    source_id_idx: usize,
    phot_g_mean_mag_idx: usize,
    csv_reader: csv::Reader<flate2::read::GzDecoder<&'a [u8]>>
}

pub struct GaiaSourceFileIterator2<'a, R> {
    source_id_idx: usize,
    phot_g_mean_mag_idx: usize,
    csv_reader: AsyncReader<Box<Compat<async_compression::tokio::bufread::GzipDecoder<tokio::io::BufReader<&'a mut R>>>>>
}

impl GaiaSourceFileIterator<'_> {
    fn new(contents: &[u8]) -> GaiaSourceFileIterator {
        let decoder = flate2::read::GzDecoder::new(contents);

        log::trace!("Starting csv parsing for {} bytes", contents.len());
        let mut csv_reader = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .has_headers(true)
            .from_reader(decoder);


        let headers: Vec<_> = csv_reader.headers().unwrap().iter().collect();
        let source_id_idx = headers.iter().position(|key| (**key).eq("source_id")).unwrap();
        let phot_g_mean_mag_idx = headers.iter().position(|key| (**key).eq("phot_g_mean_mag")).unwrap();

        return GaiaSourceFileIterator {
            csv_reader,
            source_id_idx,
            phot_g_mean_mag_idx
        }
    }
}


impl Iterator for GaiaSourceFileIterator<'_> {
    type Item = GaiaSourceFileRow;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.csv_reader.records().next();
        return match next {
            Option::None => Option::None,
            Option::Some(next) => {
                let record = next.unwrap();
                let cols: Vec<_> = record.into_iter().collect();

                let source_id = cols[self.source_id_idx].to_string();
                let phot_g_mean_mag =
                    if cols[self.phot_g_mean_mag_idx] == "null" {
                        f32::NAN
                    } else if let Ok(phot_g_mean_mag) = cols[self.phot_g_mean_mag_idx].parse::<f32>() {
                        phot_g_mean_mag
                    } else {
                        f32::NAN
                    };


                Option::Some(GaiaSourceFileRow {
                    source_id,
                    phot_g_mean_mag
                })
            }
        }
    }
}

impl GaiaSourceFile {
    fn new(url: url::Url, hash: String) -> Self {
        return GaiaSourceFile {
            hash,
            url
        }
    }

    pub fn iter<'a>(&self, contents: &'a [u8]) -> GaiaSourceFileIterator<'a> {
        return GaiaSourceFileIterator::new(contents);
    }

    pub fn parse_all(&self, contents: Vec<u8>) -> Vec<GaiaSourceFileRow> {
        let decoder = flate2::read::GzDecoder::new(&contents[..]);
        // let mut uncompressed = String::new();
        //
        // decoder.read_to_string(&mut uncompressed).unwrap();

        log::trace!("Starting csv parsing for {} bytes", contents.len());
        let mut csv_reader = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .has_headers(true)
            .from_reader(decoder);


        let headers: Vec<_> = csv_reader.headers().unwrap().iter().collect();
        let source_id_idx = headers.iter().position(|key| (**key).eq("source_id")).unwrap();
        let phot_g_mean_mag_idx = headers.iter().position(|key| (**key).eq("phot_g_mean_mag")).unwrap();
        let mut output = Vec::<GaiaSourceFileRow>::new();

        for record in csv_reader.records() {
            if let Ok(record) = record {
                let cols: Vec<_> = record.into_iter().collect();

                let source_id = cols[source_id_idx].to_string();
                let phot_g_mean_mag =
                    if cols[phot_g_mean_mag_idx] == "null" {
                        f32::NAN
                    }
                    else if let Ok(phot_g_mean_mag) = cols[phot_g_mean_mag_idx].parse::<f32>() {
                        phot_g_mean_mag
                    }
                    else {
                        f32::NAN
                    };


                output.push(GaiaSourceFileRow {
                    source_id,
                    phot_g_mean_mag
                });
            }
        }
        log::trace!("Done csv parsing for {} bytes with {} records", contents.len(), output.len());
        return output;
    }
}

impl Photometry {
    fn new(dr3: &DR3) -> Photometry {
        return Photometry {
            url: dr3.url.join("Photometry/").unwrap()
        };
    }

    pub fn epoch_photometry(&self) -> EpochPhotometry {
        return EpochPhotometry::new(self);
    }
}

impl EpochPhotometry {
    fn new(photometry: &Photometry) -> EpochPhotometry {
        return EpochPhotometry {
            url: photometry.url.join("epoch_photometry/").unwrap()
        };
    }
}

impl DataDirectory for EpochPhotometry {
    type FileType = EpochPhotometryFile;

    fn listing_file_url(&self) -> url:: Url {
        return self.url.join("_MD5SUM.txt").unwrap();
    }

    fn list_from_listing_file_contents(&self, contents: Vec<u8>) -> Vec<Self::FileType> {

        return String::from_utf8(contents).unwrap().lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .map(|cols| EpochPhotometryFile::new(
                self.url.join(cols[1]).unwrap(),
                cols[0].to_string()
            ))
            .collect();
    }
}

impl EpochPhotometryFile {
    fn new(url: url::Url, hash: String) -> Self {
        return EpochPhotometryFile {
            hash,
            url
        }
    }

    pub fn parse_all(&self, contents: Vec<u8>) -> Vec<EpochPhotometryFileRow> {
        let mut decoder = flate2::read::GzDecoder::new(&contents[..]);
        let mut uncompressed = String::new();
        decoder.read_to_string(&mut uncompressed).unwrap();

        let mut csv_reader = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .has_headers(true)
            .from_reader(uncompressed.as_bytes());


        let headers: Vec<_> = csv_reader.headers().unwrap().iter().collect();
        let source_id_idx = headers.iter().position(|key| (**key).eq("source_id")).unwrap();
        let g_transit_mag_idx = headers.iter().position(|key| (**key).eq("g_transit_mag")).unwrap();
        let mut output = Vec::<EpochPhotometryFileRow>::new();

        for record in csv_reader.records() {
            let record = record.unwrap();
            let cols: Vec<_> = record.into_iter().collect();

            let source_id = cols[source_id_idx];
            let g_transit_mag = cols[g_transit_mag_idx];
            let g_transit_mag_floats: Vec<_> = (g_transit_mag[1..g_transit_mag.len() - 1])
                .split(",")
                .map(|s| s.trim())
                .map(|s| s.parse::<f32>().unwrap())
                .filter(|f| !f.is_nan())
                .collect();

            output.push(EpochPhotometryFileRow {
                source_id: source_id.to_string(),
                g_transit_mag: g_transit_mag_floats
            })
        }
        return output;
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;

    #[test]
    fn photometry() {
        let esa = ESA::new();
        let epoch_photometry = esa.gaia().dr3().photometry().epoch_photometry();
        assert_eq!(epoch_photometry.listing_file_url().to_string(), "https://cdn.gea.esac.esa.int/Gaia/gdr3/Photometry/epoch_photometry/_MD5SUM.txt");
        let listing_file_contents = Vec::<u8>::from("hash filename.csv.gz");
        let files = epoch_photometry.list_from_listing_file_contents(listing_file_contents);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].url.path_segments().unwrap().last().unwrap(), "filename.csv.gz");

        let file_contents = [
            "source_id,g_transit_mag",
            "sid,\"[1, 2]\"",
        ].join("\n");

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(file_contents.as_bytes()).unwrap();
        let rows = files[0].parse_all(encoder.finish().unwrap());
        assert_eq!(rows.len(), 1, "exact number of rows not parsed");
        assert_eq!(rows[0].source_id, "sid", "source id not correctly extracted");
        assert_eq!(rows[0].g_transit_mag.len(), 2, "array not correctly parsed");
        assert_eq!(rows[0].g_transit_mag[1], 2.0, "array elements not correctly parsed");
    }

    #[test]
    fn gaia_source() {
        let esa = self::ESA::new();
        let source = esa.gaia().dr3().gaia_source();
        assert_eq!(source.listing_file_url().to_string(), "https://cdn.gea.esac.esa.int/Gaia/gdr3/gaia_source/_MD5SUM.txt");
        let listing_file_contents = Vec::<u8>::from("hash filename.csv.gz");
        let files = source.list_from_listing_file_contents(listing_file_contents);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].url.path_segments().unwrap().last().unwrap(), "filename.csv.gz");

        let file_contents = [
            "source_id,phot_g_mean_mag",
            "sid,1.1",
        ].join("\n");

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(file_contents.as_bytes()).unwrap();
        let rows = files[0].parse_all(encoder.finish().unwrap());
        assert_eq!(rows.len(), 1, "exact number of rows not parsed");
        assert_eq!(rows[0].source_id, "sid", "source id not correctly extracted");
        assert_eq!(rows[0].phot_g_mean_mag, 1.1, "floating point not correctly parsed");
    }


}
