use tokio::sync::mpsc;
use tokio_serial::SerialPortBuilderExt;
use tokio::signal;
use clap::Parser;

use microcontroller::serial_task::SerialTask;
use microcontroller::dds_task::DDSTask;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    ///The device path to a serial port
    #[arg(short, long, default_value_t = String::from("/dev/boat_control") )]
    port: String,

    /// The baudrate to connect with
    #[arg(short, long, default_value_t = 115200)]
    baudrate: u32,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Starting the microcontroller interface.");
    let cli = Cli::parse();

    let port_name = &cli.port;
    let baud_rate = cli.baudrate;

    let mut port = match tokio_serial::new(port_name, baud_rate).open_native_async() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Failed to create the serial port: {:?}", e);
            ::std::process::exit(1);
        }
    };

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");    
    
    // Setting up the task communication channels.
    let (serial_tx, serial_rx) = mpsc::channel(16);
    let (dds_tx, dds_rx) = mpsc::channel(16);

    let mut serial_task = SerialTask::new (port, serial_rx, dds_tx);
    tokio::spawn(async move {
        serial_task.run().await;
    });

    let mut dds_task = DDSTask::new(serial_tx, dds_rx);
    tokio::spawn(async move {
        dds_task.run().await;
    });

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

}

