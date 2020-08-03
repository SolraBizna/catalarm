use std::{
    io::{Read,Write,stdin,stdout},
    process::{Command,Stdio},
    time::{Instant,Duration},
};

fn do_alarm() {
    let mut args_it = std::env::args_os().skip(1);
    let mut command = Command::new(args_it.next().unwrap());
    while let Some(arg) = args_it.next() {
        command.arg(arg);
    }
    command.stdin(Stdio::null());
    let mut spawned = command.spawn().expect("Couldn't spawn command!");
    // make sure we don't leave any zombies!
    std::thread::spawn(move || {
        let _ = spawned.wait();
    });
}

fn main() {
    if std::env::args_os().nth(1).is_none() {
        eprintln!("Usage: catalarm command [args...] < input > output");
        std::process::exit(1);
    }
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut buf = [0u8; 1024];
    let mut next_alarm = Instant::now() - Duration::new(10, 0);
    loop {
        match stdin.read(&mut buf[..]).expect("read failed") {
            0 => return,
            x => {
                let now = Instant::now();
                if now >= next_alarm {
                    next_alarm = now + Duration::new(1, 0);
                    do_alarm();
                }
                stdout.write_all(&buf[..x]).expect("write failed")
            }
        }
    }
}
