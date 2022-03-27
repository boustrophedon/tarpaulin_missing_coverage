use tarpaulin_missing_coverage::*;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    println!("args: {:?}", args);
    if args.contains(&"do_the_thing".into()) {
        do_the_thing();
        return;
    }
    else if args.contains(&"do_other_thing".into()) {
        do_other_thing();
        return;
    }

    let exe_path = std::env::current_exe().unwrap();

    std::process::Command::new(exe_path.clone())
        .args(&["call_main", "--", "do_the_thing"])
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    std::process::Command::new(exe_path)
        .args(&["call_main", "--", "do_other_thing"])
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));
}
