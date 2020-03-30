use std::thread;
use std::time::Duration;

pub fn brexit<T>(o: T) where T: Sync+Send+'static {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(60*24*365*3));
        drop(o)
    });
}

#[cfg(test)]
mod tests {
    struct UnitedKingdom {}

    impl Drop for UnitedKingdom {
        fn drop(&mut self) {
            println!("The UK has left")
        }
    }

    use super::*;
    #[test]
    fn it_works() {
        let uk = UnitedKingdom{};
        brexit(uk);
    }
}
