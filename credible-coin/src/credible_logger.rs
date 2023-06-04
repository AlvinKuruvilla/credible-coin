use env_logger::Env;
use std::fs::{File, OpenOptions};
use std::io::Write;

const FILE_PATH: &str = "credible.log";

pub fn create_file_if_not_exist(path: &str) {
    if std::path::Path::new(path).exists() {
        println!("File exists already");
        return;
    }
    File::create(FILE_PATH).expect("Error encountered while creating file!");
}
pub fn warn(msg: &str) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .unwrap();
    create_file_if_not_exist(FILE_PATH);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_PATH)
        .unwrap();

    writeln!(file, "WARN: {}", msg).expect("Couldn't write to file");
    log::warn!("{}", msg);
}
pub fn info(msg: &str) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .unwrap();
    create_file_if_not_exist(FILE_PATH);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_PATH)
        .unwrap();

    writeln!(file, "INFO: {}", msg).expect("Couldn't write to file");
    log::info!("{}", msg);
}
pub fn error(msg: &str) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .unwrap();
    create_file_if_not_exist(FILE_PATH);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_PATH)
        .unwrap();

    writeln!(file, "ERROR: {}", msg).expect("Couldn't write to file");
    log::error!("{}", msg);
}
