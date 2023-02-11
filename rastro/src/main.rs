use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::config_file::ConfigFile;
use crate::indi::connection::{IndiConnection};
use crate::indi::enable_blob::{EnableBLOB, EnableBLOBValue};
use crate::indi::get_properties::GetProperties;
use crate::indi::IncomingMsg;
mod indi;
mod config_file;

struct App {
    quit: Arc<AtomicBool>
}

impl App {
    fn new() -> App {
        pretty_env_logger::init();
        log::info!("starting");
        let quit = Arc::new(AtomicBool::new(false));

        signal_hook::flag::register_conditional_shutdown(
            signal_hook::consts::SIGINT,
            1,
            quit.clone()
        ).unwrap();

        signal_hook::flag::register(
            signal_hook::consts::SIGINT,
            quit.clone()
        ).unwrap();

        App {
            quit
        }
    }

    fn should_quit(&self) -> bool {
        return self.quit.load(Ordering::Relaxed)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        log::info!("stopping");
    }
}


//#[tokio::main(flavor = "multi_thread", worker_threads=8)]
fn main() -> Result<(), Box<dyn Error>>{

    let app = App::new();

    let config = ConfigFile::load_default()?;
    for connection_spec in &config.connections {

        let mut conn_control = IndiConnection::connect(connection_spec)?;
        let mut conn_blob = IndiConnection::connect(connection_spec)?;

        conn_blob.send(&IncomingMsg::EnableBLOB(EnableBLOB {value: EnableBLOBValue::Only}))?;
        conn_control.send(&IncomingMsg::EnableBLOB(EnableBLOB {value: EnableBLOBValue::None}))?;

        conn_control.send(&IncomingMsg::GetProperties(GetProperties {version: "1.7".to_string()}))?;
        conn_blob.send(&IncomingMsg::GetProperties(GetProperties {version: "1.7".to_string()}))?;


        let mut counter = 0;
        while !app.should_quit() {
            let msg_control = conn_control.recv_or_none()?;
            match msg_control {
                None => {
                    log::trace!("sleeping");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                },
                Some(_msg) => {
                    log::trace!("getting messages");
                    log::debug!("{}", _msg);
                    while let Some(_msg) = conn_control.recv_or_none()? {
                        log::debug!("{}", _msg);
                    }
                    log::trace!("got messages");
                }
            }

            let msg_blob = conn_blob.recv_or_none()?;
            match msg_blob {
                None => {
                    log::trace!("no blob");
                },
                Some(_msg) => {
                    log::trace!("getting blob");
                    log::debug!("{}", _msg);
                    while let Some(_msg) = conn_blob.recv_or_none()? {
                        log::debug!("{}", _msg);
                    }
                    log::trace!("got blobs");
                }
            }


            counter = counter + 1;
        }
    }

    Ok(())
}
