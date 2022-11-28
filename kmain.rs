#![crate_name = "tinyos"]
#![crate_type = "staticlib"]
#![no_std]

extern crate env_io;
extern crate mdo;

use core::iter::*;
use env_io::{eff, EnvIO, FlatMap, IntoEnvIO};
use mdo::mdo;

fn bind<Env, OutEnv, F>(envio: Env, f: F) -> FlatMap<Env, F>
where
    Env: EnvIO,
    OutEnv: EnvIO,
    F: Fn(Env::Out) -> OutEnv,
{
    envio.flat_map(f)
}

mod frame_buffer;
mod io;
mod serial;

#[no_mangle]
pub extern "C" fn kmain() {
    // Serial logging
    let port = serial::Port::new();
    let p = &port;

    let program = mdo! {
        ret eff!(p.write("effect!! "))
    };
    program.run();
    frame_buffer::move_cursor(0);
    for _ in 0..285 {
        frame_buffer::write(
            "coffee ",
            frame_buffer::Color::Green,
            frame_buffer::Color::Blue,
        );
    }

    // for _ in 0..1000 {
    //     port.write("coffee ");
    // }
}
