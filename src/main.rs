mod indi;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::TryRecvError;
use std::thread::JoinHandle;
use std::time::Duration;
use indi::IncomingMsg;

use crate::indi::connection::Connection;

fn start_reader_loop<T: std::io::Read + Send + 'static>(name: String, sender: std::sync::mpsc::Sender<IncomingMsg>, mut reader: indi::connection::Reader<T>, quit: Arc<AtomicBool>) -> JoinHandle<()> {
    let reader: JoinHandle<()> = std::thread::spawn(move || {
        loop {
            if quit.load(Relaxed) {
                break;
            }
            let doc = reader.read();
            match doc {
                Ok(doc) => {
                    sender.send(doc).unwrap()
                },

                Err(err) => {
                    if let Some(io_error) = err.downcast_ref::<std::io::Error>() {
                        if let ErrorKind::WouldBlock = io_error.kind() {
                            std::thread::sleep(Duration::from_millis(200));
                            continue;
                        }
                    }

                    panic!("{0}", err);
                }
            }
        }
        eprintln!("Quitting {} thread!", name);
    });
    return reader;
}

fn main() {

    let host = "mobile-mini.local:7624";
    let host = "localhost:7624";

    let Connection { r, mut w} = indi::connection::Connection::connect_tcp(host).unwrap();
    let Connection { r : r_blob, w: mut _w_blob } = indi::connection::Connection::connect_tcp(host).unwrap();

    //let mut conn_control = std::sync::Mutex::new(indi::connection::Connection::connect(host).unwrap());

    let (sender, receiver) = std::sync::mpsc::channel();

    let quit = Arc::new(AtomicBool::new(false));
    let control_thread_reader = start_reader_loop(String::from("control"), sender.clone(), r, quit.clone());
    let blob_thread_reader = start_reader_loop(String::from("blob"), sender.clone(), r_blob, quit.clone());

    signal_hook::flag::register_conditional_shutdown(
        signal_hook::consts::SIGINT,
        1,
        quit.clone()
    ).unwrap();

    signal_hook::flag::register(
        signal_hook::consts::SIGINT,
        quit.clone()
    ).unwrap();



    w.send_get_properties().unwrap();
    w.send_enable_blob("", indi::connection::EnableBlobSemantics::Also).unwrap();

    loop {
        if quit.load(Relaxed) {
            eprintln!("Quiting main thread!");
            break;
        }
        let msg = receiver.try_recv();
        match msg {
            Ok(doc) => {

                match doc {
                    IncomingMsg::DefSwitchVector(def) => {
                        eprintln!("{}", def);
                    },
                    IncomingMsg::SetSwitchVector(def) => {
                        eprintln!("{}", def);
                    }

                    IncomingMsg::DefTextVector(def) => {
                        eprintln!("{}", def);
                    },
                    IncomingMsg::SetTextVector(def) => {
                        eprintln!("{}", def);
                    },

                    IncomingMsg::DefLightVector(def) => {
                        eprintln!("{}", def);
                    },

                    IncomingMsg::DefNumberVector(def) => {
                        eprintln!("{}", def);
                    },
                    IncomingMsg::SetNumberVector(def) => {
                        eprintln!("{}", def);
                    },

                    IncomingMsg::DefBlobVector(def) => {
                        eprintln!("{}", def);
                    },

                    IncomingMsg::SetBlobVector(def) => {
                        eprintln!("{}", def);
                    },

                    IncomingMsg::Message(msg) => {
                        eprintln!("{}", msg);
                    }

                    //
                    // IncomingMsg::Unparsed(msg) => {
                    //     eprintln!("{:?}", msg);
                    // }

                    IncomingMsg::Unparsed => {
                        eprintln!("Unparsed Message");
                    }
                }
                continue;
            }
            Err(err) => {
                match err {
                    TryRecvError::Empty => {
                        std::thread::sleep(Duration::from_millis(200));
                        continue;
                    },
                    TryRecvError::Disconnected => {
                        break;
                    }
                }

            }
        }
    }
    std::thread::sleep(Duration::from_millis(200));
    blob_thread_reader.join().unwrap();
    control_thread_reader.join().unwrap();
    return;
}
