extern crate env_logger;
extern crate log;
extern crate log4rs;
extern crate log_panics;
#[cfg(target_os = "android")]
extern crate android_logger;

use settings;
use std::sync::{Once, ONCE_INIT};
use std::env;
#[cfg(target_os = "android")]
use self::android_logger::Filter;

pub struct LoggerUtils {}

static LOGGER_INIT: Once = ONCE_INIT;

// From: https://www.tutorialspoint.com/log4j/log4j_logging_levels.htm
//
//DEBUG	Designates fine-grained informational events that are most useful to debug an application.
//ERROR	Designates error events that might still allow the application to continue running.
//FATAL	Designates very severe error events that will presumably lead the application to abort.
//INFO	Designates informational messages that highlight the progress of the application at coarse-grained level.
//OFF	The highest possible rank and is intended to turn off logging.
//TRACE	Designates finer-grained informational events than the DEBUG.
//WARN	Designates potentially harmful situations.


impl LoggerUtils {
    pub fn init() {
        log_panics::init(); //Logging of panics is essential for android. As android does not log to stdout for native code
        
        if cfg!(target_os = "android") {
            LOGGER_INIT.call_once(|| {
                #[cfg(target_os = "android")]
                    android_logger::init_once(
                    Filter::default().with_min_level(log::Level::Trace)

                );
            });


                // logger for testing purposes, sends to stdout (set env RUST_LOG to configure log level
//                env::set_var("RUST_LOG", "debug");
//                LOGGER_INIT.call_once(|| {
//                    env_logger::init().unwrap();
//                });

        } else {
            // turn libindy logging off if RUST_LOG is not specified
    //        match env::var("RUST_LOG") {
    //            Err(_) => {
    //                env::set_var("RUST_LOG", "off");
    //            },
    //            Ok(value) =>  (),
    //        };
                    env::set_var("RUST_LOG", "trace");
            LOGGER_INIT.call_once(|| {
                env_logger::init().unwrap();
    //            match settings::get_config_value(settings::CONFIG_LOG_CONFIG) {
    //                Err(_) => {/* NO-OP - no logging configured */},
    //                Ok(x) => {
    //                    match log4rs::init_file(&x, Default::default()) {
    //                        Err(e) => info!("invalid log configuration: {}", e),
    //                        Ok(_) => {},
    //                    }
    //                }
    //            }
            });
        }
    }

    pub fn init_test_logging() {
        // logger for testing purposes, sends to stdout (set env RUST_LOG to configure log level
        env::set_var("RUST_LOG", "trace");
        LOGGER_INIT.call_once(|| {
            env_logger::init().unwrap();
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_logger() {
        LoggerUtils::init();
    }
}
