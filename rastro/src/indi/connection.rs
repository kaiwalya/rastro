use std::error::Error;
use std::io::{ErrorKind, Write};
use std::io::ErrorKind::WouldBlock;
use std::net::TcpStream;
use quick_xml::events::{BytesText, Event};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use crate::{IncomingMsg};
use crate::config_file::ConnectionSpec;

pub struct IndiConnection {
    stream: TcpStream,
    read_handle: IndiReaderLoopHandle,
    rx: std::sync::mpsc::Receiver<IncomingMsg>
}

struct IndiReaderLoop {
    stream: TcpStream,
    output: std::sync::mpsc::Sender<IncomingMsg>
}

/**
This helps with controlled dropping of the connection which kills the thread as well.
*/
struct IndiReaderLoopHandle {
    stream: TcpStream,
    pub handle: Option<std::thread::JoinHandle<()>>
}

impl IndiReaderLoopHandle {
    fn on_drop(&mut self) {
        let handle = self.handle.take().unwrap();
        self.stream.set_nonblocking(true).unwrap();
        handle.join().unwrap();
    }
}

impl Drop for IndiReaderLoopHandle {
    fn drop(&mut self) {self.on_drop()}
}

struct IndiReaderLoopXMLProcessor {
    event_reader: quick_xml::Reader<std::io::BufReader<Box<dyn std::io::Read>>>,
    buff: Vec<u8>,
    depth: usize,
}

impl IndiReaderLoopXMLProcessor {
    fn new<Read>(r: Read) -> IndiReaderLoopXMLProcessor where Read: std::io::Read + 'static {
        let buff = std::io::BufReader::new(Box::new(r) as Box<dyn std::io::Read>);
        let event_reader = quick_xml::Reader::from_reader(buff);
        IndiReaderLoopXMLProcessor {
            event_reader,
            buff: Vec::new(),
            depth: 0
        }
    }

    fn should_quit(&self, event: &quick_xml::Result<Event>) -> bool {
        match event {
            //we if quit the reader cannot read anymore
            Ok(Event::Eof) => true,

            //or if "someone" (aka ReaderLoopHandle) set the socket as non-blocking or having a read_timeout
            Err(quick_xml::Error::Io(io)) if io.kind() == WouldBlock => true,
            _ => false
        }
    }

    fn next(&mut self) -> Result<(Option<IncomingMsg>, bool), Box<dyn Error>> {
        let mut buf = Vec::<u8>::new();
        let event = self.event_reader.read_event_into(&mut buf);

        return match &event {

            Err(_) | Ok(_) if self.should_quit(&event) => Ok((None, false)),

            Err(e) => {
                eprintln!("unhandled error while reading xml {:?}{:?}", e, e.source());
                Err(e.clone().into())
            },

            Ok(ref event) => match event {
                Event::Start(ref start) => {
                    self.buff.extend_from_slice("<".as_bytes());
                    self.buff.extend(&**event);
                    self.buff.extend_from_slice(">".as_bytes());
                    self.depth = self.depth + 1;
                    Ok((None, true))
                },

                Event::End(ref end) => {
                    self.buff.extend_from_slice("</".as_bytes());
                    self.buff.extend(&**event);
                    self.buff.extend_from_slice(">".as_bytes());

                    self.depth = self.depth - 1;
                    match self.depth {
                        0 => {

                            let msg = {
                                let str_ref = std::str::from_utf8(&self.buff).unwrap();
                                let mut de = quick_xml::de::Deserializer::from_str(str_ref);
                                IncomingMsg::deserialize(&mut de)
                            };

                            self.buff.clear();
                            match msg {
                                Ok(msg) => Ok((Some(msg), true)),
                                Err(e) => {
                                    let str_ref = std::str::from_utf8(&self.buff).unwrap();
                                    eprintln!("could not parse {str_ref}");
                                    eprintln!("{:?}", e);
                                    Err(e.into())
                                }
                            }
                        },
                        _ => Ok((None,  true))
                    }
                },

                Event::Text(ref t) => {
                    self.buff.extend(&**event);
                    Ok((None, true))
                },

                //ignore empty parts
                Event::Empty(_) => Ok((None, true)),

                Event::Eof => {
                    eprintln!("Read what we could right now");
                    Ok((None, false))
                }

                e => {
                    eprintln!("Unhandled event {:?}", e);
                    let err = std::io::Error::new(ErrorKind::Unsupported, "Unhandled message").into();
                    Err(err)
                }
            }
        };
    }
}

impl IndiReaderLoop {
    fn create(stream: TcpStream, output: std::sync::mpsc::Sender<IncomingMsg>) -> IndiReaderLoopHandle {
        let unblock_stream = stream.try_clone().unwrap();
        let handle = std::thread::spawn(move || {
            let mut r_loop = IndiReaderLoop { stream, output };

            eprintln!("reader entered");
            let output = r_loop.reader_main();
            if let Err(output) = output {
                eprintln!("reader exited with error {}", output);
            } else {
                eprintln!("reader exited");
            }
        });

        IndiReaderLoopHandle {
            stream: unblock_stream,
            handle: Some(handle)
        }
    }

    pub fn reader_main(&mut self) -> Result<(), Box<dyn Error>> {

        let r_stream = std::io::BufReader::new(self.stream.try_clone().unwrap());
        let mut event_reader = quick_xml::reader::Reader::from_reader(r_stream);

        let mut buff = Vec::new();
        let mut depth = 0;
        let mut buf = Vec::<u8>::new();

        event_reader.trim_text(true);

        loop {
            let should_quit = |event: & quick_xml::Result<Event>| -> bool {
                match event {
                    //we if quit the reader cannot read anymore
                    Ok(Event::Eof) => true,

                    //or if "someone" (aka ReaderLoopHandle) set the socket as non-blocking or having a read_timeout
                    Err(quick_xml::Error::Io(io)) if io.kind() == WouldBlock => true,
                    _ => false
                }
            };

            let event = event_reader.read_event_into(&mut buf);
            match &event {

                Err(_) | Ok(_) if should_quit(&event) => break,

                Err(e) => {
                    eprintln!("unhandled error while reading xml {:?}{:?}", e, e.source());
                    break;
                }

                Ok(ref event) => match event {
                    Event::Start(ref start) => {
                        buff.extend_from_slice("<".as_bytes());
                        buff.extend(&**event);
                        buff.extend_from_slice(">".as_bytes());
                        depth = depth + 1;
                    },

                    Event::End(ref end) => {
                        buff.extend_from_slice("</".as_bytes());
                        buff.extend(&**event);
                        buff.extend_from_slice(">".as_bytes());

                        depth = depth - 1;
                        if depth == 0 {
                            let str_ref = std::str::from_utf8(&buff).unwrap();
                            //eprintln!("{}", str_ref);
                            let mut de = quick_xml::de::Deserializer::from_str(str_ref);
                            let msg = IncomingMsg::deserialize(&mut de);
                            // let mut buff_new = Vec::new();
                            // buff_new.extend(buff.drain(..));
                            match msg {
                                Ok(msg) => {
                                    //println!("reader sent msg");
                                    self.output.send(msg)?;
                                    //println!("{}", msg)
                                },
                                Err(e) => {
                                    eprintln!("could not parse {str_ref}");
                                    eprintln!("{:?}", e);
                                    return Err(e.into());
                                }
                            }
                            buff.clear();
                        }
                    },

                    Event::Text(ref t) => {
                        buff.extend(&**event);
                    },

                    //ignore empty parts
                    Event::Empty(_) => {},

                    Event::Eof => {
                        eprintln!("Read what we could right now");
                        break;
                    }

                    e => {
                        panic!("Unhandled event {:?}", e);
                    }
                }
            }
        }
        Ok(())
    }
}


impl IndiConnection {
    pub fn connect(spec: &ConnectionSpec) -> Result<IndiConnection, Box<dyn Error>> {
        let connection_str = format!("{}:{}", spec.host, spec.port);

        let mut stream = std::net::TcpStream::connect(&connection_str).unwrap();
        let (tx, rx) = std::sync::mpsc::channel::<IncomingMsg>();
        let r_stream = stream.try_clone().unwrap();

        Ok(IndiConnection {
            stream,
            read_handle: IndiReaderLoop::create(r_stream, tx),
            rx
        })
    }

    pub fn send(&mut self, msg: &IncomingMsg) -> Result<(), Box<dyn Error>> {
        let str = quick_xml::se::to_string(msg)?;
        self.stream.write(str.as_bytes())?;
        Ok(())
    }

    pub fn recv_or_none(&self) -> Result<Option<Box<IncomingMsg>>, Box<dyn Error>> {
        match self.rx.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => Ok(None),
            Err(e) => Err(e.into()),
            Ok(msg) => Ok(Some(Box::new(msg)))
        }
    }

    pub fn recv_or_block(&self) -> Result<Box<IncomingMsg>, Box<dyn Error>> {
        let msg = self.rx.recv()?;
        Ok(Box::new(msg))
    }

}