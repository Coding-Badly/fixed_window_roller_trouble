use std::fs::remove_dir_all;

use log::{info, LevelFilter};
use log4rs::append::rolling_file::{
    policy::compound::{
        roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
    },
    RollingFileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

const DEFAULT_DAEMON_FILE_FORMAT: &str = "{m}\n";
const LOG: &str = "log";

const ROLL_PATTERN: &str = "./tmp/daemon.log.{}.gz";
const LOG_FILE_PATH: &str = "./tmp/daemon.log";

const LINE_100: &str = "123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789";
const LINE_24: &str = "123456789 123456789 123";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = remove_dir_all("./tmp");

    let fixed_window_roller = FixedWindowRoller::builder().build(ROLL_PATTERN, 10)?;

    let size_trigger = SizeTrigger::new(1024);

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let encoder = PatternEncoder::new(DEFAULT_DAEMON_FILE_FORMAT);

    let appender = RollingFileAppender::builder()
        .encoder(Box::new(encoder))
        .append(true)
        .build(LOG_FILE_PATH, Box::new(compound_policy))?;

    let file = Appender::builder().build(LOG, Box::new(appender));

    let root = Root::builder().appender(LOG).build(LevelFilter::Debug);

    let config = Config::builder().appender(file).build(root)?;

    let _ = log4rs::init_config(config)?;

    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_100);
    info!("{}", LINE_24);

    // If everything went according to plan, daemon.log is now precisely 1024 bytes.  One more
    // write will trigger a roll.

    info!("");

    // There should be an uncompressed daemon.1617070570 in the tmp directory where 1617070570 is
    // essentially a timestamp used to make the file unique.  The file is orphaned.  It will never
    // be compressed nor purged and at the next roll it will be out of sequence.

    Ok(())
}
