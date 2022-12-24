extern crate core;
mod table_streamer;
mod esa;
mod eso;
mod resource;


use crate::esa::DataDirectory;

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;
    use crate::resource::CacheableResource;
    use super::*;


    //#[test]
    #[tokio::test]
    async fn download() {
        // env_logger::init();
        //
        // let esa = esa::ESA::new();
        // let epoch_photometry = esa.gaia().dr3().gaia_source();
        // let files = epoch_photometry.list_from_listing_file_contents(epoch_photometry.listing_file_url().download().await);
        // let mut i_file = 0;
        // let n_files = files.len();
        // for f in files {
        //
        //     let mut drain= vec![0; 1024*1024];
        //     let mut download = resource::CacheableResource::_download(&f.url).await;
        //     let mut loop_count = 0;
        //     loop {
        //         let bytes = download.read_buf(&mut drain).await.unwrap();
        //         loop_count = loop_count + 1;
        //         if bytes == 0 {
        //             break;
        //         }
        //     }
        //     i_file = i_file + 1;
        //     log::debug!("{}/{} downloaded", i_file, n_files);
        //
        // }
    }

    //#[tokio::test]
    // async fn process() {
    //     env_logger::init();
    //     let esa = esa::ESA::new();
    //     let epoch_photometry = esa.gaia().dr3().gaia_source();
    //     let files = epoch_photometry.list_from_listing_file_contents(epoch_photometry.listing_file_url().download().await);
    //     let mut min = f32::INFINITY;
    //     for f in files {
    //
    //         let contents = f.url.download().await;
    //         for record in f.iter(&contents) {
    //             if record.phot_g_mean_mag < min {
    //                 min = record.phot_g_mean_mag;
    //                 log::trace!("{}: {} {}", f.url, record.source_id, record.phot_g_mean_mag);
    //             }
    //         }
    //     }
    // }
}
