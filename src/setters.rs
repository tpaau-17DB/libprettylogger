use crate::{
    filtering::*,
    colors::*,
    logging::*,
};


impl Logger {
    // FILTERING

    /// Lets you set **verbosity** to desired level
    /// * `All`        -> Don't filter any logs
    /// * `Standard`   -> Just filter debug logs
    /// * `Quiet`      -> Only let warnings and errors to be displayed
    /// * `ErrorsOnly` -> I'm not gonna explain this one
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    /// Lets you toggle log filtering
    /// * **true**  -> logs will get filtered based on verbosity
    /// * **false** -> log filtering will be disabled globally
    pub fn toggle_log_filtering(&mut self, enabled: bool) {
        self.filtering_enabled = enabled;
    }


    // FORMATTING && COLORS

    /// Toggles colored log headers
    /// * `true`  -> Log headers will have colors 
    /// * `false` -> No colors :(
    pub fn toggle_log_color(&mut self, enabled: bool) {
        self.log_color_enabled = enabled;
    }

    /// Sets **debug log header** color
    pub fn set_debug_color(&mut self, color: Color) {
        self.debug_color = color;
    }

    /// Sets **info log header** color
    pub fn set_info_color(&mut self, color: Color) {
        self.info_color = color;
    }

    /// Sets **warning header** color
    pub fn set_warning_color(&mut self, color: Color) {
        self. warning_color = color;
    }

    /// Sets **error header** color
    pub fn set_error_color(&mut self, color: Color) {
        self.error_color = color;
    }

    /// Sets **fatal error header** color
    pub fn set_fatal_color(&mut self, color: Color) {
        self.fatal_color = color;
    }

    /// Sets **debug log header** format
    pub fn set_debug_header(&mut self, format: &str) {
        self.debug_header = format.to_string();
    }

    /// Sets **info log header** format
    pub fn set_info_header(&mut self, format: &str) {
        self.info_header = format.to_string();
    }

    /// Sets **warning header** format
    pub fn set_warning_header(&mut self, format: &str) {
        self.warning_header = format.to_string();
    }

    /// Sets **error header** format
    pub fn set_error_header(&mut self, format: &str) {
        self.error_header = format.to_string();
    }

    /// Sets **fatal error header** format
    pub fn set_fatal_header(&mut self, format: &str) {
        self.fatal_header = format.to_string();
    }

    /// # Sets datetime header format
    ///
    /// Note that this function sets the **header format** alone, 
    /// not the **datetime format**!
    ///
    /// Format must contain the `{}` string to tell l where
    /// to put datetime string.
    ///
    /// For example, if you set the format to `<d>{}</d>`, 
    /// a datetime header will be displayed like this:
    /// **<d>YYYY:MM:DD:HH:mm</d>**
    ///
    /// # Return values:
    /// * `0` -> Success
    /// * `1` -> Invalid format
    pub fn set_datetime_header_format(&mut self, format: &str) -> i32 {
        let mut open_index = None;
        let mut close_index = None;

        for (i, c) in format.char_indices() {
            if c == '{' {
                open_index = Some(i);
            }
            else if c == '}' && open_index.unwrap() == i - 1 {
                close_index = Some(i);
                break;
            }
            else {
                open_index = None;
                close_index = None;
            }
        }

        if open_index.is_none() || close_index.is_none() 
        {
            // error(&format!("Invalid datetime header format: '{format}'"));
            return 1;
        }
        else {
            let opening = &format[0..open_index.unwrap()];
            let closing = &format[close_index.unwrap() + 1..format.len()];

            self.datetime_header_left = opening.to_string();
            self.datetime_header_right = closing.to_string();
        }

        return 0;
    }

    /// Toggles datetime header for every log
    /// * `true`  -> Every log will have a datetime header
    /// * `false` -> No datetime headers
    pub fn toggle_show_datetime(&mut self, enabled: bool) {
        self.show_datetime = enabled;
    }
}    
