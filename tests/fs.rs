extern crate sink;

use std::fs::OpenOptions;
use sink::fs::Entry;

#[test]
fn should_read_root_dir() {
    let result = sink::fs::set_root_dir("/tmp");
    match result {
        Err(reason) => panic!(reason),
        Ok(dir) => assert_eq!("/tmp", dir.get_name())
    }
}

#[test]
#[should_panic(expected="Not a directory")]
fn should_fail_when_setting_file_as_root_dir() {
    let path = "/tmp/sink.test";
    match OpenOptions::new().create(true).open(path) {
        Ok(_) => match sink::fs::set_root_dir(path) {
            Err(reason) => panic!(reason),
            Ok(_) => panic!("It should have panicked!")
        },
        Err(_) => panic!("Could not touch")
    }
}
