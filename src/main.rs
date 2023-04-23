use ctru::prelude::*;
use ctru::services::cfgu::Cfgu;
use ctru::applets::swkbd::{Button, Swkbd, ValidInput, Filters, Features};
use std::net::Ipv4Addr;
//use thrussh::*;
//use thrussh::server::{Auth, Session};
//use thrussh_keys::*;

fn main() {
    ctru::use_panic_handler();
    let apt = Apt::init().unwrap();
    let mut hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let soc = Soc::init().unwrap();
    let cfgu = Cfgu::init().unwrap();
    let _console = Console::init(gfx.top_screen.borrow_mut());

    //let address = soc.host_address();

    setup(cfgu, soc);

    let mut status = AppStatus::NotConnected;

    while apt.main_loop() {

        hid.scan_input();
        let keys = hid.keys_held();


        if keys.intersects(KeyPad::START) {
            println!("Exiting...");
            if status == AppStatus::Connected
            {

            }
            break;   
        }

        if (keys.intersects(KeyPad::A) && status == AppStatus::NotConnected)
        {
            let mut keyboard = Swkbd::default();
            
            keyboard.set_hint_text("user@server");

            keyboard.set_max_text_len(32);

            keyboard.set_validation(ValidInput::NotEmptyNotBlank, Filters::BACKSLASH);

            // Raise the software keyboard. You can perform different actions depending on which
            // software button the user pressed
            match keyboard.get_string(32) {
                Ok((text, Button::Right)) => 
                {
                    println!("Connecting to {}...", text);
                    status = AppStatus::Connected;
                }
                Ok((_, Button::Left)) => println!("Cancelled"),
                Ok((_, Button::Middle)) => println!("How did you even press this?"),
                Err(_) => println!("An error occurred."),
            }
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

fn setup(cfgu: Cfgu, soc: Soc) {
    println!("ctr-ssh-rs v0.1.0 by Lena");
    println!("https://github.com/adryzz/ctr-ssh-rs");
    println!("IP: {}, running on {:?}", soc.host_address(), cfgu.model().unwrap());
    println!("Press START to exit or A to connect to a server.");

    println!("\u{001b}[46;1m                \u{001b}[0m");
    println!("\u{001b}[45;1m                \u{001b}[0m");
    println!("\u{001b}[47m                \u{001b}[0m");
    println!("\u{001b}[45;1m                \u{001b}[0m");
    println!("\u{001b}[46;1m                \u{001b}[0m");
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum AppStatus {
    NotConnected,
    Connected
}