use tarpaulin_missing_coverage::*;

use std::net::TcpListener;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    println!("args: {:?}", args);
    if args.contains(&"do_the_thing".into()) {
        do_the_thing();
        let listener = TcpListener::bind("127.0.0.1:1247").unwrap();
        for _ in listener.incoming() {
            println!("this will never be printed");
        }
    }
    else if args.contains(&"do_other_thing".into()) {
        do_other_thing();
        let listener = TcpListener::bind("127.0.0.1:1248").unwrap();
        for _ in listener.incoming() {
            println!("this will also never be printed");
        }
    }

    let exe_path = std::env::current_exe().unwrap();

    let mut child1 = std::process::Command::new(exe_path.clone())
        .args(&["call_main", "--", "do_the_thing"])
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut child2 = std::process::Command::new(exe_path)
        .args(&["call_main", "--", "do_other_thing"])
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    child1.kill().unwrap();
    child2.kill().unwrap();
}
