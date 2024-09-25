use std::path::Path;

pub struct Config {}

struct serverName {
    address: String,
    port: u16,
    protocol: Protocol,
    logLevel: Option<LogLevel>,
}
struct tls {
    certPath: String,
    keyPath: String,
}
struct logging {
    accessLogPath: String,
    errorLogPath: String,
}
struct rateLimiting {
    requestPerSecond: Option<i32>,
}
struct metrics {
    endPoint: Option<String>,
}
struct serviceDiscovary {
    discovaryUrl: Option<String>,
}
struct api {
    version: String,
    basePath: String,
}
struct helthCheck {
    interval: Option<u32>,
    timeout: u32,
}
enum LogLevel {
    trace,
    debug,
    info,
    warn,
    error,
}
enum Protocol {
    http,
    https,
}
