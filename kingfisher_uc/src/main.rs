#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

//use arduino_hal::prelude::*;
use core::cell;
use panic_halt as _;
//use heapless::String;
mod usb_comms;
use usb_comms::{UsbComms, UsbCommsError};
use atmega32u4_usb_serial::UsbSerial;
use arduino_hal::simple_pwm::*;
use kingfisher_data_types::microcontroller_types::{MicroControlMessages, MicroStatusMessages, Output, ControllerState, State};

// static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

// //This function is called on change of pin 2
// // Interrupt names are INT0-3
// #[avr_device::interrupt(atmega32u4)]
// #[allow(non_snake_case)]
// fn INT0() {
//     PIN_CHANGED.store(true, Ordering::SeqCst);
// }

const OVERRIDE_SWITCH_ON: u16 = 6000;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Setup the USB serial device.
    let usb = UsbSerial::new(dp.USB_DEVICE);
    usb.init(&dp.PLL);
    let mut usb = UsbComms::new(usb);

    // Setup timers for PWM and millis functions.
    millis_init(dp.TC0);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    //enable adc
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // This is wrong, need to look into the avr datasheet
    // Enable the PCINT2 pin change interrupt
    //dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });

    // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
    //dp.EXINT.eimsk.write(|w| w.bits(0b100));

    //digital outputs / control lights
    let mut port_nav_lights = pins.d11.into_output();
    let mut stb_nav_lights = pins.d3.into_output();
    let mut debug_led = pins.d7.into_output();
    
    // outputs
    let mut stb_pwr_relay = pins.d5.into_output();
    let mut stb_ctrl = pins.d9.into_output().into_pwm(&timer1);
    stb_ctrl.enable();
    let mut port_pwr_relay = pins.d13.into_output();
    let mut port_ctrl = pins.d10.into_output().into_pwm(&timer1);
    port_ctrl.enable();


    unsafe { avr_device::interrupt::enable() };

    //analog/pwm inputs
    let throttle = pins.a0.into_analog_input(&mut adc);
    let direction = pins.a1.into_analog_input(&mut adc);
    let switch = pins.a2.into_analog_input(&mut adc);
    

    // LED switching time
    let mut next_led_switch_time: u32 = 0;
    loop {

        //try to parse that serial data.
        let topside_request = match usb.poll() {
            Ok(val) => {
                // handle the incoming messages.
                Some(val)
            },
            Err(e) => {
                match e {
                    UsbCommsError::NothingRead => (),
                    UsbCommsError::ParseError => (),
                    UsbCommsError::BufferOverflow => (),
                    UsbCommsError::DecodeError => ()
                }
                None
            }
        };

        // handle requests
        if let Some(request) = topside_request {
            // handle the topside request.
            match request {
                MicroControlMessages::RequestControllerState => {
                    let switch_val = switch.analog_read(&mut adc);
                    let controller_state = ControllerState {                    
                        overridden: switch_val > OVERRIDE_SWITCH_ON,
                        throttle: throttle.analog_read(&mut adc),
                        turn: direction.analog_read(&mut adc),
                        switch: switch_val
                    };

                    match usb.write_packet(&MicroStatusMessages::ControllerState(controller_state)) {
                        Ok(_) => (),
                        Err(_e) => () //todo!()
                    }
                },
                MicroControlMessages::RequestState => {
                    let state = State {
                        starboard_light: stb_nav_lights.is_set_high(),
                        port_lights: port_nav_lights.is_set_high(),
                        starboard_power: stb_pwr_relay.is_set_high(),
                        port_power: port_pwr_relay.is_set_high(),
                        starboard_throttle:  stb_ctrl.get_duty(),
                        port_throttle: port_ctrl.get_duty()
                    };

                    match usb.write_packet(&MicroStatusMessages::State(state)) {
                        Ok(_) => (),
                        Err(_e) => () //todo!()
                    }
                },
                MicroControlMessages::SetOutput(output_vec) => {
                    for item in output_vec {
                        match item {
                            Output::PortLight(val) => {
                                if val {
                                    port_nav_lights.set_high();
                                } else {
                                    port_nav_lights.set_low();
                                }
                            },
                            Output::StarboardLight(val) => {
                                if val {
                                    stb_nav_lights.set_high();
                                } else {
                                    stb_nav_lights.set_low();
                                }
                            },
                            Output::PortPower(val) => {
                                if val {
                                    port_pwr_relay.set_high();
                                } else {
                                    port_pwr_relay.set_low();
                                }
                            },
                            Output::StarboardPower(val) => {
                                if val {
                                    stb_pwr_relay.set_high();
                                } else {
                                    stb_pwr_relay.set_low();
                                }
                            },
                            Output::PortThrottle(val) => {
                                if switch.analog_read(&mut adc) > OVERRIDE_SWITCH_ON {
                                    port_ctrl.set_duty((val as i16 + 127) as u8);
                                }
                            },
                            Output::StarboardThrottle(val) => {
                                if switch.analog_read(&mut adc) > OVERRIDE_SWITCH_ON {
                                    stb_ctrl.set_duty((val as i16 + 127) as u8);
                                }
                            },
                        };
                    }
                }
            }
        }

        // Pass through the radio control inputs if the override switch is enabled
        if switch.analog_read(&mut adc) > OVERRIDE_SWITCH_ON {
            let _throttle_val = throttle.analog_read(&mut adc) as i32;
            let _turn = throttle.analog_read(&mut adc) as i32 - 32768; 

            //Handle the controller mixing here.
            //todo!()

        }

        //update the system state.
        if millis() > next_led_switch_time {
            debug_led.toggle();
            next_led_switch_time = next_led_switch_time.wrapping_add(1000);
        }
    }
}

// Setup the millis counter to use for even timing. From https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-millis.rs
//
// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

fn millis_init(tc0: arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega32u4)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}