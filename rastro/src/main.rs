use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::config_file::ConfigFile;
use crate::indi::connection::{IndiConnection};
use crate::indi::get_properties::GetProperties;
use crate::indi::IncomingMsg;
mod indi;
mod config_file;
mod context;

//#[tokio::main(flavor = "multi_thread", worker_threads=8)]
fn main() -> Result<(), Box<dyn Error>>{

    //let _log = slog_envlogger::init().unwrap();

    //let ctx = Context::new();

    let quit = Arc::new(AtomicBool::new(false));

    let config = ConfigFile::load_default()?;
    for connection_spec in &config.connections {

        let mut conn = IndiConnection::connect(connection_spec)?;
        conn.send(&IncomingMsg::GetProperties(GetProperties {version: "1.7".to_string()}))?;

        let mut counter = 0;
        while counter < 20 {
            let msg = conn.recv_or_none()?;
            match msg {
                None => {
                    eprintln!("main_thread sleeping");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    //break;
                },
                Some(msg) => {
                    eprintln!("main_thread got msg");
                    while let Some(msg) = conn.recv_or_none()? {
                        eprintln!("main_thread got msg+");
                    }
                }
            }

            counter = counter + 1;
        }


        eprintln!("main_thread after bulk read");
        std::thread::sleep(std::time::Duration::from_millis(3000));

    }

    eprintln!("Dropped");
    std::thread::sleep(std::time::Duration::from_millis(3000));
    eprintln!("Dropped + 3sec");

    //info!(ctx.logger, "{:?}", config);

    signal_hook::flag::register_conditional_shutdown(
        signal_hook::consts::SIGINT,
        1,
        quit.clone()
    ).unwrap();

    signal_hook::flag::register(
        signal_hook::consts::SIGINT,
        quit.clone()
    ).unwrap();

    Ok(())
}
