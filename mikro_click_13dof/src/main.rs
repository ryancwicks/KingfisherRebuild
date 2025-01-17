#![no_std]
#![no_main]

use defmt::{info, error};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use embassy_futures::{join::join, select::{select, Either}};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::{Driver, Instance};
use embassy_stm32::{bind_interrupts, peripherals, usb, i2c, Config};
use embassy_stm32::i2c::I2c;
use embassy_usb::class::{cdc_acm, cdc_acm::{CdcAcmClass, State}};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Sender, Receiver},
};
use postcard::to_vec;
use {defmt_rtt as _, panic_probe as _};

mod click_driver;
use click_driver::click_driver::{ClickDriver, run_click_driver};

mod imu_types;

use imu_types::ImuMessages;

static CHANNEL: Channel<ThreadModeRawMutex, ImuMessages, 64> = Channel::new();

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program Starting");
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25, 
            mul: PllMul::MUL192, 
            divp: Some(PllPDiv::DIV2), // 25mhz / 25 * 192 / 2 = 96Mhz.
            divq: Some(PllQDiv::DIV4), // 25mhz / 25 * 192 / 4 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);
    
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    //Create the i2c 
    let i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH0,
        Hertz(400_000),
        Default::default(),
    );
    
    // Create the driver, from the HAL.
    let mut ep_out_buffer = [0u8; 256];
    let mut config = embassy_stm32::usb::Config::default();
    
    // Do not enable vbus_detection. This is a safe default that works in all boards.
    // However, if your USB device is self-powered (can stay powered on if USB is unplugged), you need
    // to enable vbus_detection to comply with the USB spec. If you enable it, the board
    // has to support it or USB won't work at all. See docs on `vbus_detection` for details.
    config.vbus_detection = false;
    
    let driver = Driver::new_fs(p.USB_OTG_FS, Irqs, p.PA12, p.PA11, &mut ep_out_buffer, config);
    
    // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Kingfisher");
    config.product = Some("Kingfisher IMU");
    config.serial_number = Some("12345678");
    
    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    
    let mut state = State::new();
    
    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    
    // Create classes on the builder.
    let class = CdcAcmClass::new(&mut builder, &mut state, 64);
    
    // Build the builder.
    let mut usb = builder.build();
    
    let sender_from_system = CHANNEL.sender();
    let click_driver = ClickDriver::new(i2c);
    spawner.spawn(run_click_driver(click_driver, sender_from_system)).unwrap();
    //unwrap!(spawner.spawn(button_handler(button_boot0, sender_from_system)));
    
    // Run the USB device.
    let usb_fut = usb.run();
    
    let receive_from_system = CHANNEL.receiver();
    let sender_from_system = CHANNEL.sender();
    // Do stuff with the class!
    let serial_fut = async {
        let (mut usb_sender, mut usb_receiver) = class.split();
        loop {
            usb_receiver.wait_connection().await;
            info!("USB Connected");
            let _ = serial_handle(&mut usb_sender, &mut usb_receiver, receive_from_system, sender_from_system).await;
            info!("USB Disconnected");
        }
    };
    
    
    let led_fut = async {
        loop {
            led.set_high();
            Timer::after_millis(300).await;
            
            led.set_low();
            Timer::after_millis(300).await;
        }
    };
    
    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(serial_fut, led_fut)).await;
    
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => defmt::panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn serial_handle<'d, T: Instance + 'd>(
    usb_sender: &mut cdc_acm::Sender<'d, Driver<'d, T>>,
    usb_receiver: &mut cdc_acm::Receiver<'d, Driver<'d, T>>, 
    event_channel_receiver: Receiver<'static, ThreadModeRawMutex, ImuMessages, 64>,
    _event_channel_sender: Sender<'static, ThreadModeRawMutex, ImuMessages, 64>) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    
    loop {
        match select(
            async {
                let _n = usb_receiver.read_packet(&mut buf).await?;
                let mut data = [0; 64];
                for (i, val) in buf.iter().enumerate() {
                    data[i] = *val;
                }
                info!("data: {:x}", data);
                Ok(())
            } ,
            async {
                let val = event_channel_receiver.receive().await;
                let data = to_vec::<ImuMessages, 50>(&val).unwrap();
                info!("data: {:?}", data.as_slice());
                match usb_sender.write_packet(&data).await {
                    Ok(_) => (),
                    Err(e) => error!("USB write error: {:?}", e)
                };
                Ok(())
            }
        ).await {
            Either::First(val) => {
                match val {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(e);
                    }
                };
            },
            Either::Second(val) => {
                match val {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            
        };
    }
}

// #[embassy_executor::task]
// async fn button_handler(button: Input<'static>, event_channel: Sender<'static, ThreadModeRawMutex, DataEvents, 64>) {
//     let mut last_state = button.get_level();
//     let mut count: u16 = 0;
//     loop {
//         let current_state = button.get_level();
//         if current_state != last_state {
//             last_state = current_state;
//             count = count.wrapping_add(1);
//             info!("State Change {}", count);
//             event_channel.send(DataEvents::ButtonChangeEvent(count)).await;    
//         }
//         Timer::after_millis(50).await;
//     }
// }