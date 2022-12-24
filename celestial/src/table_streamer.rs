use std::pin::Pin;
use std::task::{Context, Poll};
use csv_async::AsyncReader;


pub struct TableStreamer<R: futures::AsyncRead + Unpin + Send> {
    _reader: AsyncReader<R>,
    _headers: Vec<String>
}

impl<R: futures::AsyncRead + Unpin + Send> TableStreamer<R> {

    pub async fn from_csv_reader(mut reader: AsyncReader<R>) -> TableStreamer<R> {

        let headers = if reader.has_headers() {
                let r = reader.headers().await.unwrap();
                r.iter().map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            };


        return TableStreamer {
            _reader: reader,
            _headers: headers
        };
    }

    fn _pull_records(r: csv_async::Result<csv_async::StringRecord>) -> Vec<String> {
        let values: Vec<String> = r.unwrap().iter().map(|s| s.to_string()).collect();
        return values;
    }

    pub fn futures_stream(&mut self) ->  impl futures::Stream<Item = Vec<String>> + '_{
        let s = futures::StreamExt::map(
            self._reader.records(),
            TableStreamer::<R>::_pull_records
        );
        return s;
    }

    pub fn tokio_stream(&mut self) ->  impl tokio_stream::Stream<Item = Vec<String>> + '_ {
        let s = tokio_stream::StreamExt::map(
            self._reader.records(),
            TableStreamer::<R>::_pull_records
        );
        return s;
    }

    pub fn columns_headers(&self) -> &Vec<String>{
        return &self._headers;
    }
}


#[cfg(test)]
mod tests {
    use crate::table_streamer::TableStreamer;
    use futures::StreamExt;

    #[tokio::test]
    async fn it_works_with_headers() {
        let csv_data = "#comment\r\ncol1,col2\r\nd1,d2\nd3,d4\n";

        let reader = csv_async::AsyncReaderBuilder::new()
            .comment(Some(b'#'))
            .has_headers(true)
            .create_reader(csv_data.as_bytes());

        let mut streamer = TableStreamer::from_csv_reader(reader).await;
        assert_eq!(streamer.columns_headers(), &vec!["col1", "col2"]);
        let mut stream = streamer.futures_stream();


        let row= futures::StreamExt::next(&mut stream).await.unwrap();
        assert_eq!(row, vec!["d1", "d2"], "Row 1 not read correctly");

        let row = futures::StreamExt::next(&mut stream).await.unwrap();
        assert_eq!(row, vec!["d3", "d4"], "Row 2 not read correctly");

        let res = futures::StreamExt::next(&mut stream).await;
        assert!(res.is_none(), "Only 2 records were expected")
    }

    #[tokio::test]
    async fn it_works_without_headers() {
        let csv_data = "#comment\r\nd1,d2\nd3,d4\n";

        let reader = csv_async::AsyncReaderBuilder::new()
            .comment(Some(b'#'))
            .has_headers(false)
            .create_reader(csv_data.as_bytes());

        let mut streamer = TableStreamer::from_csv_reader(reader).await;
        assert_eq!(streamer.columns_headers().len(), 0);
        let mut stream = streamer.futures_stream();

        let row= futures::StreamExt::next(&mut stream).await.unwrap();
        assert_eq!(row, vec!["d1", "d2"], "Row 1 not read correctly");

        let row = futures::StreamExt::next(&mut stream).await.unwrap();
        assert_eq!(row, vec!["d3", "d4"], "Row 2 not read correctly");

        let res = futures::StreamExt::next(&mut stream).await;
        assert!(res.is_none(), "Only 2 records were expected")
    }
}


