extern crate core;

use std::sync::Arc;
use std::thread::{sleep};
use std::time::Duration;
use esp_idf_svc::wifi::*;
use embedded_svc::wifi::*;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use embedded_hal::digital::blocking::OutputPin;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    const SSID: &str = "WIFI-Detector";

    #[allow(unused)]
        let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    #[allow(unused)]
        let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    #[allow(unused)]
        let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let mut led = Peripherals::take().unwrap().pins.gpio2.into_output().unwrap();


    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs).unwrap());

    loop {
        let ap_infos = wifi.scan().unwrap();

        let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

        if let Some(ap) = ours {
            println!("Found: {}", ap.ssid);
            led.set_high();
        }
        else {
            println!("Nothing found");
            led.set_low();
        }

        sleep(Duration::from_millis(300))
    }
}
