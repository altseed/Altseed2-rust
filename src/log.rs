use crate::auto_generated_core_binding::Log as CoreLog;
use crate::auto_generated_core_binding::{LogCategory, LogLevel};

/// ログ出力を行います。
pub struct Log {
    instance: CoreLog,
}

impl Log {
    pub(crate) fn new() -> Option<Self> {
        Some(Log {
            instance: CoreLog::get_instance()?,
        })
    }

    /// ログを出力します。
    pub fn write(&mut self, level: LogLevel, msg: &str) {
        self.instance.write(LogCategory::User, level, msg);
    }

    /// [LogLevel::Trace](enum.LogLevel.html#variant.Trace)でログを出力します。
    pub fn trace(&mut self, msg: &str) {
        self.instance.trace(LogCategory::User, msg);
    }

    /// [LogLevel::Debug](enum.LogLevel.html#variant.Debug)でログを出力します。
    pub fn debug(&mut self, msg: &str) {
        self.instance.debug(LogCategory::User, msg);
    }

    /// [LogLevel::Info](enum.LogLevel.html#variant.Info)でログを出力します。
    pub fn info(&mut self, msg: &str) {
        self.instance.info(LogCategory::User, msg);
    }

    /// [LogLevel::Warn](enum.LogLevel.html#variant.Warn)でログを出力します。
    pub fn warn(&mut self, msg: &str) {
        self.instance.warn(LogCategory::User, msg);
    }

    /// [LogLevel::Error](enum.LogLevel.html#variant.Error)でログを出力します。
    pub fn error(&mut self, msg: &str) {
        self.instance.error(LogCategory::User, msg);
    }

    /// [LogLevel::Critical](enum.LogLevel.html#variant.Critical)でログを出力します。
    pub fn critical(&mut self, msg: &str) {
        self.instance.critical(LogCategory::User, msg);
    }

    /// 指定した[LogCategory](enum.LogCategory.html)が出力する最低の[LogLevel](enum.LogLevel.html)を設定します。
    pub fn set_level(&mut self, category: LogCategory, level: LogLevel) {
        self.instance.set_level(category, level)
    }
}
