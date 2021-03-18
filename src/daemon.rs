pub mod demo {
    extern crate daemonize;

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::{
        fs::File,
        thread::{self, sleep},
    };

    use daemonize::Daemonize;
    pub fn execute(bool: bool) {
        if bool {
            let stdout = File::create("./tmp/daemon.out").unwrap();
            let stderr = File::create("./tmp/daemon.err").unwrap();
            let daemonize = Daemonize::new()
                .pid_file("./tmp/test.pid") // Every method except `new` and `start`
                .chown_pid_file(false) // is optional, see `Daemonize` documentation
                .working_directory("./tmp") // for default behaviour.
                .user("nobody")
                .group("staff") // Group name
                .group(2) // or group id.
                .umask(0o777) // Set umask, `0o027` by default.
                .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
                .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
                .privileged_action(|| "Executed before drop privileges");

            match daemonize.start() {
                Ok(_) => {
                    run();
                }
                Err(e) => eprintln!("Error, {}", e),
            }
        } else {
            run();
        }
    }
    fn run() {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        println!("Waiting for Ctrl-C...");
        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(3));
            println!("Got ...");
        }
        println!("Got it! Exiting...");
    }
}
