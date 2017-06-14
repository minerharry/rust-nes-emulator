use cpu::*;
use mem::*;
use main_memory::*;
use ppu::*;
use std::io;
use piston_window::*;

pub struct Nes {
    pub cpu: Cpu,
    pub chipset: Chipset
}

pub struct Chipset {
    pub mem: MainMemory,
    pub ppu: Ppu
}

fn get_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input
        }
        Err(_) => panic!("Could not read from stdin"),
    }
}

impl Nes {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, mapper: u8, prg_ram_size: usize, horiz_mapping: bool) -> Nes {
        assert!(mapper == 0, "Only mapper 0 is supported!");

        let mem = MainMemory::new(prg, prg_ram_size);

        Nes {
            cpu: Cpu::new(mem.read16(0xFFFC)),
            chipset: Chipset {
                mem: mem,
                ppu: Ppu::new(chr, horiz_mapping)
            }
        }
    }

    pub fn tick(&mut self) {
        // 523 lines each of 341 cycles and 1 line of 340 cycles
        //      = 178683 PPU cycles per 2 fields
        // http://forums.nesdev.com/viewtopic.php?t=1675
        while self.cpu.count < 178683/3 {
            self.cpu.tick(&mut self.chipset);
            self.chipset.ppu.tick(&mut self.cpu);

            if self.cpu.debug {
                if get_line().starts_with("d") {
                    self.cpu.debug = false;
                }
            }
        }

        self.cpu.count -= 178683/3;
    }

    pub fn prepare_draw(&mut self, window: &mut PistonWindow) {
        self.chipset.ppu.prepare_draw(window)
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        self.chipset.ppu.draw(c, g)
    }
}

impl Mem for Chipset {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.read_main(addr),
            0x4000 ... 0x4017 => 0 /* apu */,
            _ => self.mem.read(addr)
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr as usize {
            0x2000 ... 0x2007 => self.ppu.write_main(addr, val),
            0x4000 ... 0x4017 => () /* apu */,
            _ => self.mem.write(addr, val)
        }
    }
}