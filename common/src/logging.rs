use fern::colors::{Color, ColoredLevelConfig};
use log::debug;

pub fn setup_logging() {
  let colors_line = ColoredLevelConfig::new()
    .error(Color::Red)
    .warn(Color::Yellow);

  let colors_level = colors_line.clone().info(Color::Green);
  fern::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "{level:5} {date} {target:35} {message}",
        date = chrono::Local::now().format("%y-%m-%d %H:%M:%S%.f"),
        target = record.target(),
        level = colors_level.color(record.level()),
        message = message,
      ));
    })
    .level(log::LevelFilter::Info)
    .level_for("maple", log::LevelFilter::Trace)
    .chain(std::io::stdout())
    .apply()
    .unwrap();

  debug!("Logging setup finished");
}
