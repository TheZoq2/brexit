mod config;

use std::thread;

use self::config::Config;

const CONFIG: &str = include_str!("../Config.xml");

pub fn brexit<T>(o: T) where T: Sync+Send+'static {
    let config = Config::parse(CONFIG);
    let duration = config.duration();

    thread::spawn(move || {
        thread::sleep(duration);
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
