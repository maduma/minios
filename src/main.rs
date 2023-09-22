#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"
|==============================================================================|
                                                                                
                           - + -   M I N I O S   - + -                          
                                                                                
|==============================================================================|
                                                                                
                                                                                
                                   SHUTDOWN NOW                                 
                                   ------------                                 
                                                                                ";

fn spin(micros: u64) {
    for _ in 0..(1_000_000 * micros) as u64 {
        unsafe {
            asm!("nop");
        }
    }
}

fn outb(port: u64, byte: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") byte,
        );
    }
}

fn outw(port: u64, word: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") word,
        );
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    // disable cursor
    outb(0x3D4, 0x0A);
    outb(0x3D5, 0x20);

    // move cursor
    let x = 29u16;
    let y = 0u16;
    let position = y * 80 + x;
    outb(0x3D4, 0x0F);
    outb(0x3D5, position as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, (position >> 8) as u8);

    let vga_buffer = 0xb8000 as *mut u8;
    let mut color: u8 = 4 << 4 | 0xf;

    for (i, &byte) in HELLO.iter().filter(|&c| *c != b'\n').enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color;
        }
        if i < 400 { spin(1) }
        if i == 399 {
            color = 8 << 4 | 12;
            spin(2000)
        }
        // color = match color & 0xf {
        //     0xf => 1, // white to blue
        //     1 => 2,   // blue to green
        //     2 => 4,   // green to red
        //     4 => 5,   // red to yello
        //     _ => 0xf,
        // };
        // if i > 33 && i < 39 {
        //     // background to grey
        //     color |= 7 << 4;
        // }
    }

    spin(1000);

    // shutdown virtualbox
    outw(0x4004, 0x3400);

    loop {}
}
