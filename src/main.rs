use std::str::{self, FromStr};
use std::time::Duration;
use std::{io::Write, thread::sleep};

use anyhow::Result;
use clap::Parser;
use serialport::{DataBits, Parity, StopBits};

#[derive(Debug)]
enum Command {
    GeneralStatus,
    BatteryStatus,
    OperationalModeStatus,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "general" => Ok(Command::GeneralStatus),
            "battery" => Ok(Command::BatteryStatus),
            "operation" => Ok(Command::OperationalModeStatus),
            _ => Err(anyhow::anyhow!("")),
        }
    }
}

impl Command {
    fn to_command(&self) -> &'static str {
        match *self {
            Command::GeneralStatus => "QGS\r",
            Command::BatteryStatus => "QBV\r",
            Command::OperationalModeStatus => "QMOD\r",
        }
    }
}

#[derive(Debug, Parser)]
#[clap(version)]
struct Opts {
    /// Path to or name of serial port
    #[clap(short, long)]
    serialport: String,

    /// Print hex dump of data read from port
    #[clap(short, long, action)]
    hexdump: bool,

    /// Status command to run. Must be one of 'general' or 'battery' or 'operation'.
    #[clap(short, long, default_value = "general")]
    command: Command,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let mut port = serialport::new(&opts.serialport, 2400)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(1000))
        .parity(Parity::None)
        .open()?;

    let cmd = opts.command.to_command();
    port.write(cmd.as_bytes())?;

    sleep(Duration::from_secs(1));

    let mut buf = vec![0u8; 1024];
    let cb = port.read(buf.as_mut_slice())?;

    let outp = str::from_utf8(&buf[..cb])?;
    println!("{}\n", outp);

    if opts.hexdump {
        hexdump::hexdump(&buf[..cb]);
    }

    Ok(())
}
