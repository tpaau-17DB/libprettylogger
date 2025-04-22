use std::{
    fs::{
        create_dir_all,
        read_to_string
    },
    collections::hash_map::HashMap,
};
use rand::{
    distributions::Alphanumeric,
    Rng,
};
use crate::{
    colors::{color_text, Color}, config::{LogStruct, LogType, OnDropPolicy, Verbosity}, Logger
};

lazy_static::lazy_static! {
    static ref TMP_PATH: String = {
        let mut path = std::env::temp_dir();
        path.push("libprettylogger-tests");
        path.to_str().unwrap().to_string()
    };
}

lazy_static::lazy_static! {
    static ref COLORS: HashMap<Color, String> = {
        let mut map: HashMap<Color, String> = HashMap::new();

        map.insert(Color::Black, String::from("\x1b[30m"));
        map.insert(Color::Red, String::from("\x1b[31m"));
        map.insert(Color::Green, String::from("\x1b[32m"));
        map.insert(Color::Yellow, String::from("\x1b[33m"));
        map.insert(Color::Blue, String::from("\x1b[34m"));
        map.insert(Color::Magenta, String::from("\x1b[35m"));
        map.insert(Color::Cyan, String::from("\x1b[36m"));
        map.insert(Color::White, String::from("\x1b[37m"));

        map.insert(Color::BrightBlack, String::from("\x1b[90m"));
        map.insert(Color::BrightRed, String::from("\x1b[91m"));
        map.insert(Color::BrightGreen, String::from("\x1b[92m"));
        map.insert(Color::BrightYellow, String::from("\x1b[93m"));
        map.insert(Color::BrightBlue, String::from("\x1b[94m"));
        map.insert(Color::BrightMagenta, String::from("\x1b[95m"));
        map.insert(Color::BrightCyan, String::from("\x1b[96m"));
        map.insert(Color::BrightWhite, String::from("\x1b[97m"));

        return map;
    };
}

fn rand_string(length: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[test]
fn test_log_filtering() {
    let mut l = Logger::default();
    l.toggle_log_filtering(true);

    l.set_verbosity(Verbosity::ErrorsOnly);
    if !l.filter_log(LogType::Debug) {
        panic!("Log should get filtered!");
    }
    if !l.filter_log(LogType::Info) {
        panic!("Log should get filtered!");
    }
    if !l.filter_log(LogType::Warning) {
        panic!("Log should get filtered!");
    }

    l.set_verbosity(Verbosity::Quiet);
    if !l.filter_log(LogType::Debug) {
        panic!("Log should get filtered!");
    }
    if !l.filter_log(LogType::Info) {
        panic!("Log should get filtered!");
    }
    if l.filter_log(LogType::Warning) {
        panic!("Log not should get filtered!");
    }

    l.set_verbosity(Verbosity::Standard);
    if !l.filter_log(LogType::Debug) {
        panic!("Log should get filtered!");
    }
    if l.filter_log(LogType::Info) {
        panic!("Log should not get filtered!");
    }
    if l.filter_log(LogType::Warning) {
        panic!("Log should not get filtered!");
    }

    l.set_verbosity(Verbosity::All);
    if l.filter_log(LogType::Debug) {
        panic!("Log should not get filtered!");
    }
    if l.filter_log(LogType::Info) {
        panic!("Log should not get filtered!");
    }
    if l.filter_log(LogType::Warning) {
        panic!("Log should not get filtered!");
    }

    let mut i = 0;
    loop {
        l.set_verbosity(Verbosity::try_from(i)
            .expect("Failed to get `Verbosity` from i32!"));

        l.toggle_log_filtering(false);
        if l.filter_log(LogType::Debug) {
            panic!("A debug log should not get filtered for verbosity set to: {}",
                Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Info) {
            panic!("An informative log should not get filtered for verbosity set to: {}",
                Verbosity::ErrorsOnly);
        }
        if l.filter_log(LogType::Warning) {
            panic!("A warning log not should get filtered for verbosity set to: {}",
                Verbosity::ErrorsOnly);
        }
        i += 1;

        if i > 3 {
            break;
        }
    }
}

#[test]
fn test_log_headers() {
    // Test if header format setting works
    let header = "askljdfh";

    let mut l = Logger::default();

    l.set_debug_header(header);
    if l.get_log_type_header(LogType::Debug) !=
    l.colorify(header, l.log_header_color(LogType::Debug)) {
        panic!("Debug headers do not match!");
    }
    l.set_info_header(header);
    if l.get_log_type_header(LogType::Info) !=
    l.colorify(header, l.log_header_color(LogType::Info)) {
        panic!("Info headers do not match!");
    }
    l.set_warning_header(header);
    if l.get_log_type_header(LogType::Warning) !=
    l.colorify(header, l.log_header_color(LogType::Warning)) {
        panic!("Warning headers do not match!");
    }
    l.set_error_header(header);
    if l.get_log_type_header(LogType::Err) !=
    l.colorify(header, l.log_header_color(LogType::Err)) {
        panic!("Error headers do not match!");
    }
    l.set_fatal_header(header);
    if l.get_log_type_header(LogType::FatalError) !=
    l.colorify(header, l.log_header_color(LogType::FatalError)) {
        panic!("Fatal error headers do not match!");
    }
}

#[test]
fn test_log_colors() {
    let l = Logger::default();
    let reset = "\x1b[0m";

    for element in COLORS.iter() {
        let text = &rand_string(32);
        let color_test = l.colorify(text, element.0.clone());
        let color_manual
            = element.1.clone() + text + reset;
        assert_eq!(color_test, color_manual);
    }
}

#[test]
fn test_templates() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/test_templates.json";
    Logger::default().save_template(&path)
        .expect("Failed to save logger template");
    let l = Logger::from_template(&path)
        .expect("Failed to load Logger from a template");

    if l != Logger::default() {
        panic!("Templates don't match!\n
            first: {:?}\n
            second: {:?}",
            l,
            Logger::default());
    }
}

#[test]
fn test_formats() {
    let mut l = Logger::default();

    l.set_datetime_format("aaa");
    l.set_debug_header("d");
    l.set_info_header("i");
    l.set_warning_header("W");
    l.set_error_header("E");
    l.set_fatal_header("!");
    let _ = l.set_log_format("<l> <h>%h</h> <d>%d</d> <m>%m</m> </l>");

    let mut logstruct = LogStruct::debug("aaa");
    let mut comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
        l.colorify("d", l.log_header_color(LogType::Debug))
    );

    if l.format_log(&logstruct) != comp {
        panic!("Bad log formatting, expected \n'{}', got \n'{}'",
            comp,
            l.format_log(&logstruct));
    }

    logstruct.log_type = LogType::Info;
    comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
        l.colorify("i", l.log_header_color(LogType::Info))
    );
    if l.format_log(&logstruct) != comp {
        panic!("Bad log formatting, expected \n'{}', got \n'{}'",
            comp,
            l.format_log(&logstruct));
    }

    logstruct.log_type = LogType::Warning;
    comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
        l.colorify("W", l.log_header_color(LogType::Warning))
    );
    if l.format_log(&logstruct) != comp {
        panic!("Bad log formatting, expected \n'{}', got \n'{}'",
            comp,
            l.format_log(&logstruct));
    }

    logstruct.log_type = LogType::Err;
    comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
        l.colorify("E", l.log_header_color(LogType::Err))
    );
    if l.format_log(&logstruct) != comp {
        panic!("Bad log formatting, expected \n'{}', got \n'{}'",
            comp,
            l.format_log(&logstruct));
    }

    logstruct.log_type = LogType::FatalError;
    comp = format!("<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
        l.colorify("!", l.log_header_color(LogType::FatalError))
    );
    if l.format_log(&logstruct) != comp {
        panic!("Bad log formatting, expected \n'{}', got \n'{}'",
            comp,
            l.format_log(&logstruct));
    }
}

#[test]
fn test_auto_file_logging() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/auto_file_logging.log";
    let max_size = 16;
    let mut l = Logger::default();
    l.set_max_log_buffer_size(max_size as u32);

    l.set_log_file_path(&path)
        .expect("Failed to set the log file path");

    l.toggle_file_logging(true).expect("Failed to enable file logging");
    let mut i = 0;
    loop {
        l.fatal(&format!("i: {}", i));

        if i >= max_size - 1 {
            break;
        }
        i += 1;
    }
    let contents = read_to_string(path)
        .expect("Failed to read the log file");
    assert_eq!(contents,
                        "[[35mFATAL[0m] i: 0\n[[35mFATAL[0m] i: 1\n[[35mFATAL[0m] i: 2\n[[35mFATAL[0m] i: 3\n[[35mFATAL[0m] i: 4\n[[35mFATAL[0m] i: 5\n[[35mFATAL[0m] i: 6\n[[35mFATAL[0m] i: 7\n[[35mFATAL[0m] i: 8\n[[35mFATAL[0m] i: 9\n[[35mFATAL[0m] i: 10\n[[35mFATAL[0m] i: 11\n[[35mFATAL[0m] i: 12\n[[35mFATAL[0m] i: 13\n[[35mFATAL[0m] i: 14\n[[35mFATAL[0m] i: 15\n");
}

#[test]
fn test_manual_file_log_flushing() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/manual_file_log_flushing.log";
    let max_size = 16;
    let mut l = Logger::default();
    l.set_max_log_buffer_size(max_size as u32);

    l.set_log_file_path(&path)
        .expect("Failed to set the log file path");

    l.toggle_file_logging(true)
        .expect("Failed to enable file logging");

    l.set_datetime_format("aaa");

    l.set_log_format("<l><d>%d</d><h>%h</h><m>%m</m></l>")
        .expect("Failed to set the log format");

    l.error("eror");

    println!("{}", l.file_logging_enabled);
    l.flush().expect("Failed to flush the logger");

    let contents = read_to_string(path)
        .expect("Failed to read the log file");
    assert_eq!(contents,
        "<l><d>aaa</d><h>[31mERR[0m</h><m>eror</m></l>\n");
}

#[test]
fn test_custom_log_buffer() {
    let iter = 100;
    let mut logger = Logger::default();
    logger.toggle_log_filtering(false);
    logger.toggle_custom_log_buffer(true);

    let mut i = 0;
    loop {
        logger.debug("debug");
        i += 1;
        if i > iter - 1 {
            break;
        }
    }

    let log_buffer = logger.log_buffer();

    for log in log_buffer {
        if log.message != "debug" {
            panic!("Unexpected log message!");
        }
        if log.log_type != LogType::Debug {
            panic!("Unexpected log type!");
        }
    }

    if log_buffer.len() != iter {
        panic!("Expected a buffer size of {}, got {}.",
            iter,
            log_buffer.len());
    }
}

#[test]
fn test_color_text() {
    let reset = "\x1b[0m";

    for element in COLORS.iter() {
        let text = &rand_string(32);
        let color_test = color_text(text, element.0.to_owned());
        let color_manual
            = element.1.clone() + text + reset;
        assert_eq!(color_test, color_manual);
    }
}

#[test]
fn test_logger_errs() {
    let mut l = Logger::default();
    assert!(l.set_log_format("%h %c %d").is_err());
    assert!(l.set_log_file_path("/asjkhdfahjksdfk").is_err());
    assert!(l.toggle_file_logging(true).is_err());
    assert!(l.toggle_file_logging(false).is_ok());
}

#[test]
fn test_flush_lock() {
    let mut l = Logger::default();

    assert!(l.flush().is_err());
    assert!(l.flush_file_log_buffer(false).is_err());

    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/flush_lock.log";
    let max_size = 16;
    let mut l = Logger::default();
    l.set_max_log_buffer_size(max_size as u32);

    l.toggle_log_file_lock(true);
    l.set_log_file_path(&path)
        .expect("Failed to set the log file path");
    l.toggle_file_logging(true).expect("Failed to enable file logging");

    assert!(l.flush().is_err());

    assert!(l.flush_file_log_buffer(true).is_err());
    l.set_on_drop_file_policy(OnDropPolicy::IgnoreLogFileLock);
    assert!(l.flush_file_log_buffer(true).is_ok());
}
