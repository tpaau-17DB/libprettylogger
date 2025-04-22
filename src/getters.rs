use crate::{
    Logger,
    config::LogStruct,
};

impl Logger {
    /// Returns a reference to the custom log buffer.
    pub fn log_buffer(&self) -> &Vec<LogStruct> {
        return &self.custom_log_buffer;
    }

    /// Returns the number of logs printed since the struct was instantiated.
    pub fn log_count(&self) -> &u128 {
        return &self.log_count;
    }
}
