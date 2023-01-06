mod kalakaar;

pub fn main() {
}




#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::SeqCst;
    use crate::kalakaar::{ActorSystem};

    #[test]
    fn it_does_not_leave_threads_hanging() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut sys = ActorSystem::new();
        {
            let counter_clone = counter.clone();
            let actor_ref = sys.create_actor(move |_: String| {
                counter_clone.fetch_add(1, SeqCst);
                Ok(())
            });

            let handle = std::thread::spawn(move|| {
                actor_ref.send(format!("test 1").to_string()).unwrap();
                actor_ref.send(format!("test 2").to_string()).unwrap();
            });

            assert_eq!(sys.get_thread_count(), 1, "actor threads not found");
            handle.join().unwrap();

        }
        std::thread::sleep(std::time::Duration::from_millis(500));
        assert_eq!(sys.get_thread_count(), 0, "actor threads left alive");
        assert_eq!(counter.load(SeqCst), 2, "actor did not get all messages");

    }

    #[test]
    fn it_deals_with_erroring_actors() {
        let mut sys = ActorSystem::new();
        {
            let actor_ref = sys.create_actor(|_: String| {
                Err(std::io::Error::from(ErrorKind::AddrInUse))
            });
            assert_eq!(sys.get_thread_count(), 1, "actor threads not found");


            let handle = std::thread::spawn(move|| {
                actor_ref.send(format!("test 1").to_string()).unwrap();
            });

            handle.join().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(200));
            assert_eq!(sys.get_thread_count(), 0, "actor threads left alive")
        }
    }

    #[test]
    fn it_cannot_send_to_stopped_actors() {

        let actor_ref: Option<_>;
        {
            let mut sys = ActorSystem::new();
            actor_ref = Some(sys.create_actor(|_: String| {
                Err(std::io::Error::from(ErrorKind::AddrInUse))
            }));
            assert_eq!(sys.get_thread_count(), 1, "actor threads not found");


            let actor_ref = actor_ref.clone();
            let handle = std::thread::spawn(move|| {
                actor_ref.unwrap().send(format!("test 1").to_string()).unwrap()
            });

            handle.join().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(200));
            assert_eq!(sys.get_thread_count(), 0, "actor threads left alive")
        }
        assert_eq!(actor_ref.unwrap().send("test".to_string()).is_err(), true, "sends should fail after stopping actor system")

    }
}
