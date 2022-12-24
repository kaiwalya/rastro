use std::error::Error;
use std::io::{BufReader, Cursor};
use std::net::TcpStream;
use std::time::Duration;
use quick_xml::DeError;
use quick_xml::Error::Io;
use quick_xml::events::BytesText;
use crate::{IncomingMsg};

pub struct Connection<R: std::io::Read, W: std::io::Write> {
    pub r: Reader<R>,
    pub w: Writer<W>
}

pub trait OutgoingMsg {
    fn to_xml(&self) -> Vec<u8>;
}

#[derive(serde::Serialize)]
pub enum EnableBlobSemantics {
    Never,
    Also,
    Only
}

pub struct Reader<T: std::io::Read> {
    pub r: BufReader<T>
}

pub struct Writer<T: std::io::Write> {
    pub w: T,
    dbg_name: String
}

fn normalize_error(err: DeError) -> Box<dyn Error> {
    return if let DeError::Xml(inner_error) = err {
        if let Io(io_error) = inner_error {
            io_error.into()
        } else {
            DeError::Xml(inner_error).into()
        }
    } else {
        err.into()
    }
}

impl<T: std::io::Read> Reader<T> {
    pub fn new(r: T) -> Self {
        return Self {r: BufReader::new(r)};
    }

    pub fn read(&mut self) -> Result<IncomingMsg, Box<dyn Error>> {

        return quick_xml::de::from_reader::<_, IncomingMsg>(&mut self.r)
            .map(|res| {
                return res;
            })
            .map_err(|err| normalize_error(err));
    }

}

impl<T: std::io::Write> Writer<T> {
    pub fn new(w: T, dbg_name: &str) -> Self {
        return Self {w, dbg_name: String::from(dbg_name)}
    }
    fn write_bytes(&mut self, bytes: Vec<u8>) -> Result<(), Box<dyn Error>> {
        log::trace!("{} writing {}", self.dbg_name, String::from_utf8(bytes.clone()).unwrap());
        return std::io::Write::write(&mut self.w, bytes.as_slice())
            .map(|_| ())
            .map_err(|err| err.into());
    }
    //
    // pub fn send_message(&mut self, msg: &dyn OutgoingMsg) -> Result<(), Error> {
    //     self.write_bytes(msg.to_xml())
    // }

    pub fn send_get_properties(&mut self) -> Result<(), Box<dyn Error>>{
        let mut writer = quick_xml::Writer::new(Cursor::new(Vec::new()));
        let el_writer = writer.create_element("getProperties");
        el_writer
            .with_attribute(("version", "1.7"))
            .write_empty()
            .unwrap();

        let vector = writer.into_inner().into_inner();
        return self.write_bytes(vector);
    }

    pub fn send_enable_blob(&mut self, semantics: EnableBlobSemantics) -> Result<(), Box<dyn Error>>{
        let mut writer = quick_xml::Writer::new(Cursor::new(Vec::new()));

        let el_writer = writer.create_element("enableBLOB");

        match semantics {
            EnableBlobSemantics::Also => {
                el_writer.write_text_content(BytesText::from_plain_str("Also")).unwrap();
            },
            EnableBlobSemantics::Only => {
                el_writer.write_text_content(BytesText::from_plain_str("Only")).unwrap();
            },
            EnableBlobSemantics::Never => {
                el_writer.write_text_content(BytesText::from_plain_str("Never")).unwrap();
            }
        }

        let vector = writer.into_inner().into_inner();
        return self.write_bytes(vector);
    }
}

impl Connection<TcpStream, TcpStream> {
    pub fn connect_tcp(host: &str) -> Result<Self, &dyn Error> {

        let stream= TcpStream::connect(host).unwrap();
        stream.set_read_timeout(Some(Duration::from_millis(200))).unwrap();

        let local_address = stream.local_addr().unwrap();
        let dbg_name = local_address.to_string();


        let w = stream.try_clone().unwrap();
        let r = stream;


        let reader = Reader::new(r);
        let writer= Writer::new(w, &dbg_name);

        return Ok(Self {
            r: reader,
            w: writer,
        });
    }
}
