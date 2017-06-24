use nes::*;

pub struct SmbHack {
    world: u8,
    level: u8,
    prelevel_skip: bool,
    skip: bool,
}

impl SmbHack {
    pub fn new() -> SmbHack {
        SmbHack {
            world: 0,
            level: 0,
            prelevel_skip: false,
            skip: false,
        }
    }
}

pub fn initial_state(nes: &mut Nes) {
    // Big 'ol hack to skip the title screen
    nes.smb_hack.skip = true;
    set_level(nes);

    for _ in 0..60 {
        nes.tick();
        set_level(nes);
    }

    nes.chipset.controller1.start = true;
    for _ in 0..2 {
        nes.tick();
        set_level(nes);
    }
    nes.chipset.controller1.start = false;

    skip_prelevel(nes);
}

fn skip_prelevel(nes: &mut Nes) {
    nes.smb_hack.skip = true;
    for _ in 0..10 {
        // Hack the prelevel timer to clear so the level starts right away
        nes.chipset.write(0x07A0, 0);
        nes.tick();
    }
    nes.smb_hack.skip = false;
}

fn skip_death(nes: &mut Nes) {
    nes.smb_hack.skip = true;
    for _ in 0..30 {
        nes.tick();
    }
    nes.smb_hack.skip = false;
}

fn choose_level(nes: &mut Nes) {
    nes.smb_hack.level += 1;
    if nes.smb_hack.level >= 4 {
        nes.smb_hack.world += 1;
        nes.smb_hack.level = 0;
    }
}

fn set_level(nes: &mut Nes) {
    nes.chipset.write(0x0760, nes.smb_hack.level); // Level
    nes.chipset.write(0x075F, nes.smb_hack.world); // World
}

pub fn tick(nes: &mut Nes) {
    if nes.smb_hack.skip {
        return;
    }
    let game_engine_subroutine = 0x0E;

    set_level(nes);
    if nes.smb_hack.prelevel_skip {
        if nes.chipset.read(0x07A0) == 7 {
            nes.smb_hack.prelevel_skip = false;
            skip_prelevel(nes);
        }
    }

    // Infinite lives
    nes.chipset.write(0x075A, 8);

    // End of level
    if nes.chipset.read(game_engine_subroutine) == 0x05 {
        choose_level(nes);
        set_level(nes);
        nes.smb_hack.prelevel_skip = true;
    }

    // Player death
    if nes.chipset.read(game_engine_subroutine) == 0x0B // Death by enemy?
        || nes.chipset.read(game_engine_subroutine) == 0x06 { // Death by falling?
        // TODO rewind time
        // For now, we just advance through the pre-level
        if nes.special {
            skip_death(nes);
            skip_prelevel(nes);
        }
        else {
            nes.smb_hack.prelevel_skip = true;
        }
    }
}