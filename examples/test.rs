use tarpaulin_missing_coverage::*;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    if args.contains(&"do_the_thing".into()) {
        do_the_thing();
        return;
    }

    let exe_path = std::env::current_exe().unwrap();

    std::process::Command::new(exe_path)
        .args(&["do_the_thing"])
        .spawn()
        .unwrap();
}

#[test]
fn call_main() {
    main();
}
