#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega32u4_usb_serial::UsbSerial;
use core::sync::atomic::{AtomicBool, Ordering};
use panic_halt as _;

static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

//This function is called on change of pin 2
#[avr_device::interrupt(atmega32u4)]
#[allow(non_snake_case)]
fn INT0() {
    PIN_CHANGED.store(true, Ordering::SeqCst);
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut usb = UsbSerial::new(dp.USB_DEVICE);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // This is wrong, need to look into the avr datasheet
    // Enable the PCINT2 pin change interrupt
    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });

    // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
    dp.EXINT.eimsk.write(|w| w.bits(0b100));

    //digital outputs / control lights
    let mut _port_nav_light = pins.d11.into_output();
    let mut _stb_nav_lights = pins.d3.into_output();
    let mut debug_led = pins.d7.into_output();
    
    // outputs
    let mut _std_pwr_relay = pins.d5.into_output();
    let mut _stb_ctrl = pins.d10.into_output();
    let mut _port_pwr_relay = pins.d13.into_output();
    let mut _port_ctrl = pins.d8.into_output();
    let mut _fan = pins.led_tx.into_output();

    unsafe { avr_device::interrupt::enable() };

    //analog/pwm inputs
    let _throttle = pins.a0.into_analog_input(&mut adc);
    //let direction = pins.d0.into_analog_input(&mut adc);
    //let switch = pins.d1.into_analog_input(&mut adc);

    usb.init(&dp.PLL);

    loop {
        debug_led.toggle();
        arduino_hal::delay_ms(1000);
        ufmt::uwriteln!(usb, "Hello, World!").unwrap();
    }
}
