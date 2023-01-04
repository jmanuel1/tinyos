#![crate_name = "tinyos"]
#![crate_type = "staticlib"]
#![no_std]

#[macro_use]
extern crate operational;
extern crate alloc;
extern crate mdo;

use alloc::boxed::Box;
use operational::instr;
use operational::Program;

mod frame_buffer;
mod io;
mod serial;

enum PortInstr<'a, 's> {
    Write(&'a serial::Port, &'s str),
}

impl<'p, 's> instr::Instr for PortInstr<'p, 's> {
    type Return = ();
}

fn write_to_port<'p, 's>(port: &'p serial::Port, string: &'s str) -> PortInstr<'p, 's> {
    use PortInstr::Write;

    Write(port, string)
}

fn port_do<'a, A>(program: Program<'a, PortInstr<'a, 'a>, A>) -> Box<A> {
    use PortInstr::Write;
    use Program::*;

    // TODO: Use a loop instead?

    match program {
        Then(instr, k) => match instr.as_ref() {
            Write(port, string) => {
                port.write(string);
                let p = k.run(());
                port_do(p)
            }
        },
        Pure(a) => a,
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    use operational::instr::Instr;

    // Serial logging
    let port = serial::Port::new();
    let p = &port;

    let program = seq! {
        write_to_port(p, "effect!! ")
    }
    .to_program();
    port_do(program);
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
