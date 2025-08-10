use std::{
    sync::LazyLock,
    collections::hash_map::HashMap,
    fs::{
        create_dir_all,
        read_to_string
    },
};

use rand::{
    distributions::Alphanumeric,
    thread_rng,
    Rng
};

use crate::{
    Logger,
    colors::{
        color_text,
        Color
    },
    config::{
        LogStruct,
        LogType,
        OnDropPolicy,
        Verbosity
    },
    format::LogFormatter,
    output::{
        BufferStream,
        FileStream,
        Toggleable
    }
};

const REPEAT_MIN: u32 = 1;
const REPEAT_MAX: u32 = 1024;

const RESET: &str = "\x1b[0m";

static TMP_PATH: LazyLock<String> = LazyLock::new(|| {
    let mut path = std::env::temp_dir();
    path.push("libprettylogger-tests");
    path.to_str().unwrap().to_string()
});

static COLORS: LazyLock<HashMap<Color, String>> = LazyLock::new(|| {
    let mut map: HashMap<Color, String> = HashMap::new();

    map.insert(Color::Black, String::from("\x1b[30m"));
    map.insert(Color::Red, String::from("\x1b[31m"));
    map.insert(Color::Green, String::from("\x1b[32m"));
    map.insert(Color::Yellow, String::from("\x1b[33m"));
    map.insert(Color::Blue, String::from("\x1b[34m"));
    map.insert(Color::Magenta, String::from("\x1b[35m"));
    map.insert(Color::Cyan, String::from("\x1b[36m"));
    map.insert(Color::White, String::from("\x1b[37m"));

    map
});

fn rand_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// Check if logs are properly filtered by the Logger struct
#[test]
fn log_filtering() {
    // Loop over all the possible Verbosity values
    let mut i = 0;
    loop {
        let mut l = Logger::default();
        let verbosity = Verbosity::try_from(i)
            .expect("Failed to get `Verbosity`!");
        l.set_verbosity(verbosity);

        match verbosity {
            Verbosity::ErrorsOnly => {
                if !l.filter_log(LogType::Debug) {
                    panic!("Log should get filtered!");
                }
                if !l.filter_log(LogType::Info) {
                    panic!("Log should get filtered!");
                }
                if !l.filter_log(LogType::Warning) {
                    panic!("Log should get filtered!");
                }
            },
            Verbosity::Quiet => {
                if !l.filter_log(LogType::Debug) {
                    panic!("Log should get filtered!");
                }
                if !l.filter_log(LogType::Info) {
                    panic!("Log should get filtered!");
                }
                if l.filter_log(LogType::Warning) {
                    panic!("Log not should get filtered!");
                }
            },
            Verbosity::Standard => {
                if !l.filter_log(LogType::Debug) {
                    panic!("Log should get filtered!");
                }
                if l.filter_log(LogType::Info) {
                    panic!("Log should not get filtered!");
                }
                if l.filter_log(LogType::Warning) {
                    panic!("Log should not get filtered!");
                }
            },
            Verbosity::All => {
                if l.filter_log(LogType::Debug) {
                    panic!("Log should not get filtered!");
                }
                if l.filter_log(LogType::Info) {
                    panic!("Log should not get filtered!");
                }
                if l.filter_log(LogType::Warning) {
                    panic!("Log should not get filtered!");
                }
            },
        }

        // Error logs cannot be silenced
        if l.filter_log(LogType::Err) {
            panic!("Log should not get filtered!");
        }
        if l.filter_log(LogType::FatalError) {
            panic!("Log should not get filtered!");
        }

        // With log filtering disabled
        l.disable_log_filtering();
        if l.filter_log(LogType::Debug) {
            panic!("Log should not get filtered when filtering is disabled!");
        }
        if l.filter_log(LogType::Info) {
            panic!("Log should not get filtered when filtering is disabled!");
        }
        if l.filter_log(LogType::Warning) {
            panic!("Log should not get filtered when filtering is disabled!");
        }
        if l.filter_log(LogType::Err) {
            panic!("Log should not get filtered!");
        }
        if l.filter_log(LogType::FatalError) {
            panic!("Log should not get filtered!");
        }

        i += 1;
        if i > 3 {
            break;
        }
    }
}

// Test if Logger templates are correctly serialized and deserialized
#[test]
fn templates() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/templates.json";

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

// Test if setting different log header formats works
#[test]
fn log_headers() {
    let header = &rand_string(32);
    let mut f = LogFormatter::default();

    f.set_debug_header(header);
    f.set_info_header(header);
    f.set_warning_header(header);
    f.set_error_header(header);
    f.set_fatal_header(header);

    if f.get_log_type_header(LogType::Debug) !=
    f.colorify(header, f.log_header_color(LogType::Debug)) {
        panic!("Debug headers do not match!");
    }
    if f.get_log_type_header(LogType::Info) !=
    f.colorify(header, f.log_header_color(LogType::Info)) {
        panic!("Info headers do not match!");
    }
    if f.get_log_type_header(LogType::Warning) !=
    f.colorify(header, f.log_header_color(LogType::Warning)) {
        panic!("Warning headers do not match!");
    }
    if f.get_log_type_header(LogType::Err) !=
    f.colorify(header, f.log_header_color(LogType::Err)) {
        panic!("Error headers do not match!");
    }
    if f.get_log_type_header(LogType::FatalError) !=
    f.colorify(header, f.log_header_color(LogType::FatalError)) {
        panic!("Fatal error headers do not match!");
    }
}

// Test if logs are formatted as expected
#[test]
fn formats() {
    let mut f = LogFormatter::default();

    f.set_datetime_format("aaa");
    f.set_debug_header("d");
    f.set_info_header("i");
    f.set_warning_header("W");
    f.set_error_header("E");
    f.set_fatal_header("!");
    f.set_log_format("<l> <h>%h</h> <d>%d</d> <m>%m</m> </l>")
        .expect("Failed to set log format!");

    let mut logstruct = LogStruct::debug("aaa");

    let tests = [
        (LogType::Debug, "d"),
        (LogType::Info, "i"),
        (LogType::Warning, "W"),
        (LogType::Err, "E"),
        (LogType::FatalError, "!"),
    ];

    for (log_type, header) in tests.iter() {
        logstruct.log_type = *log_type;
        let comp = format!(
            "<l> <h>{}</h> <d>aaa</d> <m>aaa</m> </l>\n",
            f.colorify(header, f.log_header_color(*log_type))
        );

        if f.format_log(&logstruct) != comp {
            panic!(
                "Bad log formatting, expected \n'{}', got \n'{}'",
                comp,
                f.format_log(&logstruct)
            );
        }
    }
}

// Test text coloring with standard colors
#[test]
fn test_color_text() {
    for element in COLORS.iter() {
        let text = &rand_string(32);
        let color_test = color_text(text, element.0.to_owned());
        let color_manual
            = element.1.clone() + text + RESET;
        assert_eq!(color_test, color_manual);
    }
}

// Test text coloring with non-standard colors
#[test]
fn color_text_custom() {
    for element in COLORS.iter() {
        let text = &rand_string(32);
        let color = element.0.as_ref().to_string();
        let color_test = color_text(text, Color::Custom(color.clone()));
        let color_manual = color + text + RESET;
        assert_eq!(color_test, color_manual);
    }
}

// Test if formatter is throwing errors when it should
#[test]
fn formatter_errs() {
    let mut f: LogFormatter;

    // Without a message placeholder
    f = LogFormatter::default();
    assert!(f.set_log_format("%h %d").is_err());
    assert!(f.set_log_format("").is_err());

    // With a message placeholder
    f = LogFormatter::default();
    assert!(f.set_log_format("%m").is_ok());
    f = LogFormatter::default();
    assert!(f.set_log_format("%m %h %d").is_ok());
}

// Test if file output is throwing errors when it should
#[test]
fn file_output_errs() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/file_output.log";

    let log = LogStruct::debug("example debug message");
    let mut formatter = LogFormatter::default();

    let mut fo: FileStream;


    // Disabled
    fo = FileStream::default();
    assert!(fo.out(&log, &mut formatter).is_err());
    assert!(fo.flush().is_err());
    assert!(fo.internal_flush(true).is_err());
    assert!(fo.enable().is_err());

    // Enabled, log file unlocked
    fo = FileStream::default();
    fo.set_log_file_path(&path).expect("Failed to set the log file path!");
    fo.enable().expect("Failed to enable file output!");
    fo.unlock_file();
    assert!(fo.flush().is_err());
    assert!(fo.internal_flush(true).is_err());
    assert!(fo.out(&log, &mut formatter).is_ok());
    assert!(fo.flush().is_ok());
    assert!(fo.out(&log, &mut formatter).is_ok());
    assert!(fo.internal_flush(false).is_ok());
    assert!(fo.out(&log, &mut formatter).is_ok());
    assert!(fo.internal_flush(true).is_ok());

    // Enabled, log file locked
    fo = FileStream::default();
    fo.set_log_file_path(&path).expect("Failed to set the log file path!");
    fo.enable().expect("Failed to enable file output!");
    fo.lock_file();
    assert!(fo.flush().is_err());
    assert!(fo.internal_flush(true).is_err());
    assert!(fo.out(&log, &mut formatter).is_ok());
    assert!(fo.flush().is_err());
    assert!(fo.internal_flush(false).is_err());
    fo.set_on_drop_policy(OnDropPolicy::DiscardLogBuffer); // Respect the lock
    assert!(fo.internal_flush(true).is_err());
    fo.set_on_drop_policy(OnDropPolicy::IgnoreLogFileLock); // Ignore the lock
    assert!(fo.internal_flush(true).is_ok());
}

// Test if file logging is working as expected
#[test]
fn file_logging() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/file_logging.log";

    let mut rng = thread_rng();
    let log = LogStruct::debug("example debug message");
    let n = rng.gen_range(REPEAT_MIN..REPEAT_MAX) as usize;

    let mut formatter = LogFormatter::default();

    let mut fo = FileStream::default();
    fo.set_log_file_path(&path)
        .expect("Failed to set log file path!");
    fo.enable()
        .expect("Failed to enable file output!");

    let mut log_vec: Vec<String> = Vec::new();
    for _ in 0..n {
        fo.out(&log, &mut formatter)
            .expect("Failed to out to a file output!");
        log_vec.push(formatter.format_log(&log));
    }
    fo.flush()
        .expect("Failed to flush the file output!");

    match read_to_string(path) {
        Err(e) => {
            panic!("{}", e);
        },
        Ok(contents) => {
            assert_eq!(contents, log_vec.join(""));
        },
    }
}

// Test if automatic log file buffer flushing is working
#[test]
fn auto_file_logging() {
    create_dir_all(TMP_PATH.clone()).expect("Failed to create a directory");
    let path = TMP_PATH.to_owned() + "/auto_file_logging.log";

    let mut rng = thread_rng();
    let log = LogStruct::debug("example debug message");
    let n = rng.gen_range(REPEAT_MIN..REPEAT_MAX) as usize;

    let max_buffer_size = rng.gen_range(1..REPEAT_MAX) as usize;

    let mut formatter = LogFormatter::default();

    let mut fo = FileStream::default();
    fo.set_max_buffer_size(Some(max_buffer_size));
    fo.set_log_file_path(&path)
        .expect("Failed to set log file path!");
    fo.enable()
        .expect("Failed to enable file output!");

    let mut log_vec: Vec<String> = Vec::new();
    for i in 0..n {
        fo.out(&log, &mut formatter)
            .expect("Failed to out to a file output!");

        if i != 0 {
            if i % max_buffer_size == 0 {
                match read_to_string(path.clone()) {
                    Err(e) => {
                        panic!("{}", e);
                    },
                    Ok(contents) => {
                        assert_eq!(contents, log_vec.join(""));
                    },
                }
            }
            else {
                match read_to_string(path.clone()) {
                    Err(e) => {
                        panic!("{}", e);
                    },
                    Ok(contents) => {
                        assert_ne!(contents, log_vec.join(""));
                    },
                }
            }
        }
        log_vec.push(formatter.format_log(&log));
    }
}

// Check if log buffering is working fine
#[test]
fn log_buffering() {
    let mut rng = thread_rng();
    let log = LogStruct::debug("example debug message");
    let n = rng.gen_range(REPEAT_MIN..REPEAT_MAX) as usize;
    let mut bo: BufferStream;

    bo = BufferStream::default();
    bo.enable();
    for _ in 0..n {
        bo.out(&log);
    }

    assert!(n == bo.log_buffer.len());

    for bo_log in bo.log_buffer {
        assert!(bo_log == log);
    }
}
