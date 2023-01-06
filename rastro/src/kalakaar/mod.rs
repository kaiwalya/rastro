use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::TryRecvError;


pub trait ActorRef {
    type Message;
    fn send(&self, msg: Self::Message);
}

#[derive(Clone)]
pub struct ActorRefImpl<Message> {
    sender: std::sync::mpsc::Sender<Message>
}

impl<Message> ActorRefImpl<Message> {
    pub fn send(&self, msg: Message) -> std::io::Result<()> {
        self.sender.send(msg)
            .map_err(|e| {
                std::io::Error::new(ErrorKind::Other, e.to_string())
            })
    }
}


pub struct ActorSystem {
    thread_count: Arc<AtomicUsize>
}


impl ActorSystem {
    pub fn new() -> Self {
        ActorSystem {
            thread_count: Arc::new(AtomicUsize::new(0))
        }
    }

    pub fn get_thread_count(&self) -> usize {
        self.thread_count.load(Ordering::SeqCst)
    }

    pub fn create_actor<F, T>(&mut self, handler: F) -> ActorRefImpl<T>
        where
            F: Fn(T) -> std::io::Result<()>,
            F: Send + 'static,
            T: Send + 'static
    {
        let (sender, receiver) = std::sync::mpsc::channel::<T>();

        let counter = self.thread_count.clone();
        counter.fetch_add(1, Ordering::SeqCst);
        std::thread::spawn(move||{

            'main_loop: loop {
                match receiver.try_recv(){
                    Ok(t) => match handler(t) {
                        Ok(_) => {},
                        Err(_) => break 'main_loop
                    }
                    Err(err) => match err {
                        TryRecvError::Empty => {},
                        TryRecvError::Disconnected => break 'main_loop
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(50));
            }

            counter.fetch_sub(1, Ordering::SeqCst);
        });

        ActorRefImpl {
            sender
        }
    }

}


