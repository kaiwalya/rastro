use std::io::{Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;
use futures::{TryFutureExt, TryStreamExt};
use tokio::io::{AsyncRead, AsyncWriteExt, ReadBuf};

pub struct ReadThroughReader {
    cache_file: Option<std::fs::File>,
    source: Box<dyn AsyncRead + Unpin + Send>
}

impl ReadThroughReader {
    fn read_from_source_async(self: &mut Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let before = buf.filled().len();
        let pinned_source = unsafe {Pin::new_unchecked(&mut self.source)};
        let poll_result = pinned_source.poll_read(cx, buf);
        match poll_result {
            Poll::Pending => {},
            Poll::Ready(ref result) => {
                match result {
                    Ok(_) => log::trace!("ReadThroughReader::read_from_source_async got {} bytes", buf.filled().len() - before),
                    Err(err) => log::trace!("ReadThroughReader source errored {:?}", err)
                }
            }
        }
        return poll_result;
    }
}

impl AsyncRead for ReadThroughReader {

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let old_filled = buf.filled().len();
        let poll_result = self.read_from_source_async(cx, buf);
        return match poll_result {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                if result.is_ok() {
                    if let Some(cache_file) = &mut self.cache_file {
                        let new_filled = buf.filled().len();
                        let _new_bytes = new_filled - old_filled;
                        cache_file.write(&buf.filled()[old_filled..]).unwrap();
                        cache_file.flush().unwrap();
                    }
                }
                Poll::Ready(result)
            }
        };
    }
}

#[async_trait]
pub trait CacheableResource {
    async fn download_async(&self) -> ReadThroughReader {
        let cache_path = self.absolute_cache_path();
        let exists = cache_path.exists();
        if exists {
            log::trace!("Cache hit {:?}", &cache_path);
            return ReadThroughReader {
                cache_file: None,
                source: Box::new(tokio::fs::File::open(&cache_path).await.unwrap())
            };
        }

        let async_reader_box = self.manifest_async().await;
        std::fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
        //std::fs::write(&cache_path, &async_reader_box).unwrap();
        //log::trace!("Cache ({:?}) stored {} bytes", cache_path, async_reader_box.len());

        return ReadThroughReader {
            cache_file: Some(std::fs::File::create(&cache_path).unwrap()),
            source: async_reader_box
        };
    }
    //
    // async fn download(&self) -> Vec<u8> {
    //     let cache_path = self.absolute_cache_path();
    //     let exists = cache_path.exists();
    //     if exists {
    //         log::trace!("Cache hit {:?}", &cache_path);
    //         let contents = std::fs::read(&cache_path).unwrap();
    //         log::trace!("Cache read {} bytes from {:?}", contents.len(), &cache_path);
    //         return contents;
    //     }
    //
    //     let mut reader = self.manifest_async().await;
    //     let mut contents = Vec::new();
    //     reader.read_to_end(&mut contents).await.unwrap();
    //     std::fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
    //     std::fs::write(&cache_path, &contents).unwrap();
    //     log::trace!("Cache ({:?}) stored {} bytes", cache_path, contents.len());
    //     return contents;
    // }

    fn relative_cache_path(&self) -> std::path::PathBuf;
    async fn manifest_async(&self) -> Box<dyn AsyncRead + Unpin + Send>;

    fn absolute_cache_path(&self) -> std::path::PathBuf {
        let base_path = dirs::cache_dir().unwrap().join("rastro").join("web");
        let final_path = base_path.join(self.relative_cache_path());
        return final_path;
    }
}

#[async_trait]
impl CacheableResource for url::Url {

    fn relative_cache_path(&self) -> std::path::PathBuf {
        let domain = self.domain().unwrap();
        let cache_path = std::path::Path::new("cache");
        let domain_path = cache_path.join(domain);
        let relative_path = self.path_segments().unwrap()
            .fold(domain_path, |a, b| a.join(b));
        return relative_path;
    }

    async fn manifest_async(&self) -> Box<dyn AsyncRead + Unpin + Send> {
        eprintln!("Downloading {}", self.as_str());
        let res = reqwest::get(self.clone()).await.unwrap();

        if res.status() != 200 {
            panic!("Cannot download {}", self.as_str());
        }

        let stream = res.bytes_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));

        return Box::new(tokio_util::io::StreamReader::new(stream));
    }
}


#[cfg(test)]
mod tests {
    
    use tokio::io::AsyncReadExt;
    use crate::resource::{CacheableResource, ReadThroughReader};

    async fn consume(reader: &mut ReadThroughReader) -> (usize, std::time::Duration) {
        let mut sz_completed: usize = 0;
        //let mut buff = vec![0; 1024*1024];
        let mut buff = vec![0; 1024];
        let start_time = std::time::Instant::now();
        while let result = reader.read(&mut buff[..]).await {
            if let Ok(sz) = result {
                if sz == 0 {
                    break;
                }
                sz_completed = sz_completed + sz;
            }
            else {
                sz_completed = sz_completed;
                break;
            }
        }
        let end_time = std::time::Instant::now();
        let time_to_complete = end_time - start_time;
        return (sz_completed, time_to_complete)
    }

    #[tokio::test]
    async fn it_can_work_with_example_com() {
        env_logger::init();
        let _url_str = "http://cdn.gea.esac.esa.int/Gaia/gdr3/gaia_source/GaiaSource_000000-003111.csv.gz";
        let url_str = "https://example.com/index.html";
        let url = url::Url::parse(url_str).unwrap();
        let path = url.absolute_cache_path();
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }
        let mut reader = url.download_async().await;

        let first_run = consume(&mut reader).await;

        assert!(first_run.0 > 0, "Could not read from example.com");
        assert!(path.exists(), "Cache file should be created");

        /*
        Expecting a cached version here
        */
        let mut reader = url.download_async().await;
        let second_run = consume(&mut reader).await;
        assert!(second_run.0 > 0, "Could not read from example.com");


        assert_eq!(first_run.0, second_run.0, "Both runs should have the same size");
        //cache should be faster for slightly larger file,
        //but we are testing with a small file, we just make sure the cache takes around
        //the same time
        assert!(first_run.1 > second_run.1.mul_f32(0.5), "{:?} >= {:?}", first_run.1, second_run.1);
    }
}