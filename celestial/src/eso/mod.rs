mod catalog_row;







use crate::eso::catalog_row::CatalogRow;

pub struct Tycho2 {
    pub catalog_url: url::Url,
    pub suppl_1: url::Url,
    pub suppl_2: url::Url,
    pub index: url::Url
}


impl Tycho2 {
    pub fn new() -> Tycho2 {
        let base = url::Url::parse("http://archive.eso.org/ASTROM/TYC-2/data/").unwrap();
        return Tycho2 {
            catalog_url: base.join("catalog.dat").unwrap(),
            suppl_1: base.join("suppl1.dat").unwrap(),
            suppl_2: base.join("suppl2.dat").unwrap(),
            index: base.join("index.dat").unwrap()
        }
    }

    pub fn parse_row_columns(&self, row: Vec<String>) -> CatalogRow {
        return CatalogRow::new(row);
    }
}
//
// struct CatalogParser<'a, R: AsyncBufRead + ?Sized + Unpin> {
//     inner: &'a mut R
// }
//
// impl<R: AsyncBufRead + ?Sized + Unpin> CatalogParser<'_, R> {
//     fn new(reader: &mut R) -> CatalogParser<R> {
//         return CatalogParser {
//             inner: reader
//         }
//     }
// }
//
// #[async_trait]
// impl<R: AsyncBufRead + ?Sized + Unpin> tokio_stream::Stream for CatalogParser<'_, R> {
//     type Item = CatalogRow;
//
//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//
//         let mut buff = String::new();
//         let mut future = self.inner.read_line(&mut buff);
//         let future_pinned = unsafe { Pin::new_unchecked(&mut future)};
//         let res_poll = future_pinned.poll(cx);
//         return match res_poll {
//             Poll::Pending => Poll::Pending,
//             Poll::Ready(result) => {
//                 let size = result.unwrap();
//                 if size == 0 {
//                     Poll::Ready(Option::None)
//                 }
//                 else {
//                     Poll::Ready(Option::Some(CatalogRow {
//                         test: buff
//                     }))
//                 }
//             }
//         };
//     }
// }

#[cfg(test)]
mod tests {
    use futures::{StreamExt};
    use tokio_util::compat::TokioAsyncReadCompatExt;
    use crate::eso::{Tycho2};
    use crate::eso::catalog_row::CatalogRow;
    use crate::resource::CacheableResource;
    use crate::table_streamer::TableStreamer;

    #[tokio::test]
    async fn tycho2_download() {
        env_logger::init();
        let tycho2 = Tycho2::new();

        let catalog_bin_reader = tycho2.catalog_url.download_async().await;
        let csv_reader = csv_async::AsyncReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .create_reader(catalog_bin_reader.compat());

        let mut catalog_row_streamer = TableStreamer::from_csv_reader(csv_reader).await;
        let mut stream = catalog_row_streamer.tokio_stream()
            .map(|r: Vec<String> | -> CatalogRow {
                return tycho2.parse_row_columns(r);
            });

        while let Some(t) = stream.next().await as Option<CatalogRow> {
            log::trace!("{:?}", t);
        }
    }
}
