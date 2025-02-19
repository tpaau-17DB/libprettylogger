use std::fs::{create_dir_all, read_to_string};
use crate::*;

static TEST_PATH : &str = "/tmp/libprettylogger-tests";

#[test]
fn test_log_filtering() {
    let mut l = Logger::default();
    l.toggle_log_filtering(true);

    l.set_verbosity(Verbosity::ErrorsOnly);
    if !l.filter_log(LogType::Debug) {
        panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if !l.filter_log(LogType::Info) {
        panic!("An informative log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if !l.filter_log(LogType::Warning) {
        panic!("A warning log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }

    l.set_verbosity(Verbosity::Quiet);
    if !l.filter_log(LogType::Debug) {
        panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if !l.filter_log(LogType::Info) {
        panic!("An informative log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Warning) {
        panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }

    l.set_verbosity(Verbosity::Standard);
    if !l.filter_log(LogType::Debug) {
        panic!("A debug log should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Info) {
        panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Warning) {
        panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }

    l.set_verbosity(Verbosity::All);
    if l.filter_log(LogType::Debug) {
        panic!("A debug log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Info) {
        panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Warning) {
        panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }

    l.set_verbosity(Verbosity::All);
    l.toggle_log_filtering(false);
    if l.filter_log(LogType::Debug) {
        panic!("A debug log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Info) {
        panic!("An informative log should not get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
    }
    if l.filter_log(LogType::Warning) {
        panic!("A warning log not should get filtered for verbosity set to: {}", Verbosity::ErrorsOnly);
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
    // Test if colorify works
    let l = Logger::default();
    if l.colorify("a", Color::Red) != "\x1b[31ma\x1b[0m"
    {
        panic!("Failed to colorify a string!");
    }
}

#[test]
fn test_templates() {
    let _ = create_dir_all(TEST_PATH);
    let path = TEST_PATH.to_owned() + "/test_templates.json";
    Logger::default().save_template(&path);
    let l = Logger::from_template(&path);

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
    let _ = create_dir_all(TEST_PATH);
    let path = TEST_PATH.to_owned() + "/auto_file_logging.log";
    let max_size = 16;
    let mut l = Logger::default();
    l.set_max_log_buffer_size(max_size as u32);

    l.set_log_file_path(&path)
        .expect("Failed to seth the log file path!");

    l.toggle_file_logging(true).expect("Failed to enable file logging!");
    let mut i = 0;
    loop {
        l.fatal(&format!("i: {}", i));

        if i >= max_size - 1 {
            break;
        }
        i += 1;
    }
    let contents = read_to_string(path)
        .expect("Failed to read the log file!");
    assert_eq!(contents,
                        "[[35mFATAL[0m] i: 0\n[[35mFATAL[0m] i: 1\n[[35mFATAL[0m] i: 2\n[[35mFATAL[0m] i: 3\n[[35mFATAL[0m] i: 4\n[[35mFATAL[0m] i: 5\n[[35mFATAL[0m] i: 6\n[[35mFATAL[0m] i: 7\n[[35mFATAL[0m] i: 8\n[[35mFATAL[0m] i: 9\n[[35mFATAL[0m] i: 10\n[[35mFATAL[0m] i: 11\n[[35mFATAL[0m] i: 12\n[[35mFATAL[0m] i: 13\n[[35mFATAL[0m] i: 14\n[[35mFATAL[0m] i: 15\n");
}

#[test]
fn test_manual_file_log_flushing() {
    let _ = create_dir_all(TEST_PATH);
    let path = TEST_PATH.to_owned() + "/manual_file_log_flushing.log";
    let max_size = 16;
    let mut l = Logger::default();
    l.set_max_log_buffer_size(max_size as u32);

    l.set_log_file_path(&path)
        .expect("Failed to seth the log file path!");

    l.toggle_file_logging(true)
        .expect("Failed to enable file logging!");

    l.set_datetime_format("aaa");

    l.set_log_format("<l><d>%d</d><h>%h</h><m>%m</m></l>")
        .expect("Failed to set the log format!");

    l.error("eror");
    l.flush()
        .expect("Failed to flush the logger!");

    let contents = read_to_string(path)
        .expect("Failed to read the log file!");
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

    let log_buffer = logger.clone_log_buffer();

    for log in &log_buffer {
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
    assert_eq!(color_text("aaa", Color::None), "aaa");
    assert_eq!(color_text("aaa", Color::Black), "\x1b[30maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Blue), "\x1b[34maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Cyan), "\x1b[36maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Green), "\x1b[32maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Gray), "\x1b[90maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Magenta), "\x1b[35maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Red), "\x1b[31maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::White), "\x1b[37maaa\x1b[0m");
    assert_eq!(color_text("aaa", Color::Yellow), "\x1b[33maaa\x1b[0m");
}
