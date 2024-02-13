
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub enum LogType {
    Info,
    Debug,
    Error,
    Warning,
    Event,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub user_id: Option<String>,
    pub system: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub log_level: Option<LogType>,
    pub timestamp: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub service: Option<String>,
    pub ip: Option<String>,
}

pub trait LogExt {
    fn new() -> Self;
    fn user_id(self, user_id: String) -> Self;
    fn system(self, system: String) -> Self;
    fn file(self, file: String) -> Self;
    fn line(self, line: u32) -> Self;
    fn message(self, message: String) -> Self;
    fn source(self, source: Source) -> Self;
    fn log_level(self, log_level: LogType) -> Self;
    fn timestamp(self, timestamp: SystemTime) -> Self;
}

pub trait SourceExt {
    fn new() -> Self;
    fn name(self, name: String) -> Self;
    fn version(self, version: String) -> Self;
    fn environment(self, environment: String) -> Self;
    fn service(self, service: String) -> Self;
    fn ip(self, ip: String) -> Self;
}