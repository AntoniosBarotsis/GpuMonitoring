use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use std::{io::Write, thread, time::Duration};

fn main() {
  const GREEN: &str = "\x1b[38;5;82m";
  const ORANGE: &str = "\x1b[38;5;202m";
  const RED: &str = "\x1b[38;5;196m";
  const RESET: &str = "\x1b[0m";

  let wait_time = Duration::from_millis(1000);

  loop {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let temperature = device.temperature(TemperatureSensor::Gpu).unwrap();

    let temperature = match temperature {
      75.. => format!("{RED}{}{RESET}", temperature),
      65..=74 => format!("{ORANGE}{}{RESET}", temperature),
      _ => format!("{GREEN}{}{RESET}", temperature),
    };

    let msg = format!(
      "Device: {}\nTemperature: {}C",
      device.name().unwrap(),
      temperature
    );

    // https://stackoverflow.com/a/34837038/12756474
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{msg}");
    std::io::stdout().flush().ok();
    thread::sleep(wait_time);
  }
}
