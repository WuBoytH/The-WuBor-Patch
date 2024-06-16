use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    wubor_utils::controls::*
};

// #[repr(C)]
// struct BufferState {
//     pub on_last_frame: u32,
//     pub should_hold: [bool; 32],
//     pub hold_frame: [i32; 32],
//     pub hold_frame_max: [i32; 32],
// }

// impl BufferState {
//     pub fn new() -> Self {
//         Self {
//             on_last_frame: 0,
//             should_hold: [false; 32],
//             hold_frame: [0; 32],
//             hold_frame_max: [-1; 32],
//         }
//     }

//     pub fn clear(&mut self) {
//         self.on_last_frame = 0;
//         self.should_hold = [false; 32];
//         self.hold_frame = [0; 32];
//         self.hold_frame_max = [-1; 32];
//     }

//     pub fn update(
//         &mut self,
//         game_held: &mut [u8],
//         max_hold_frame: i32,
//         press_frame: i32,
//         should_hold: bool,
//     ) {
//         self.on_last_frame = 0;
//         for (idx, x) in game_held.iter_mut().enumerate() {
//             if *x != 0
//                 && (self.hold_frame[idx] < press_frame
//                     || self.should_hold[idx]
//                     || should_hold
//                     || *x != 1)
//             {
//                 self.hold_frame[idx] += 1;
//                 if self.hold_frame[idx] < press_frame {
//                     continue;
//                 }
//                 if *x == 1 {
//                     if self.should_hold[idx] {
//                         if self.hold_frame_max[idx] != -1
//                             && self.hold_frame_max[idx] < self.hold_frame[idx]
//                         {
//                             *x = 0;
//                             self.hold_frame[idx] = 0;
//                             continue;
//                         }
//                     }
//                 } else if should_hold {
//                     if max_hold_frame != -1 && max_hold_frame < self.hold_frame[idx] {
//                         *x = 0;
//                         self.hold_frame[idx] = 0;
//                         continue;
//                     }
//                 }
//                 self.on_last_frame |= 1 << idx;
//             } else {
//                 self.hold_frame[idx] = 0;
//                 *x = 0;
//             }
//         }
//     }
// }

// #[repr(C)]
// struct CommandFlagCat {
//     flags: u32,
//     _x4: u32,
//     count: usize,
//     lifetimes: *mut u8,
//     lifetimes2: *mut u8,
//     lifetimes3: *mut u64,
// }

// impl CommandFlagCat {
//     fn lifetimes(&self) -> &[u8] {
//         unsafe { std::slice::from_raw_parts(self.lifetimes, self.count) }
//     }

//     fn lifetimes_mut(&self) -> &mut [u8] {
//         unsafe { std::slice::from_raw_parts_mut(self.lifetimes, self.count) }
//     }

//     fn lifetimes2(&self) -> &[u8] {
//         unsafe { std::slice::from_raw_parts(self.lifetimes2, self.count) }
//     }
// }

// #[skyline::hook(offset = offsets::get_command_flag_cat())]
// fn get_command_flag_cat_replace(control_module: u64, cat: i32) -> u32 {
//     let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };
//     let battle_object = unsafe { get_battle_object_from_id((*boma).battle_object_id) };

//     if cat == 4 {
//         if !has_input_module!(battle_object) {
//             return 0;
//         }

//         let im = require_input_module!(battle_object);

//         let mut output = 0;
//         // this iterates across all 32 bits of the output bitmask, where valid_frames represents how many frames
//         // left any given custom input may have left in its internal buffer state.
//         for x in 0..32 {
//             if im.hdr_cat.valid_frames[x] != 0 {
//                 output |= 1 << x;
//             }
//         }

//         return output;
//     }

//     let cats =
//         unsafe { std::slice::from_raw_parts((control_module + 0x568) as *mut CommandFlagCat, 4) };

//     let mut output = 0;
//     let lifetimes = cats[cat as usize].lifetimes();
//     let lifetimes2 = cats[cat as usize].lifetimes2();
//     for x in 0..cats[cat as usize].count {
//         if lifetimes[x] > 0 && lifetimes2[x] <= 1 {
//             output |= 1 << x;
//         }
//     }
//     output
// }

fn exec_internal(module_accessor: *mut BattleObjectModuleAccessor) {
    unsafe {
        // Prevent game from thinking you are inputting a flick on the frame the cstick stops overriding left stick
        if Buttons::from_bits_retain(ControlModule::get_release(module_accessor)).intersects(Buttons::CStickOverride) {
            ControlModule::reset_flick_x(module_accessor);
            ControlModule::reset_flick_y(module_accessor);
        }
    }
}

fn exec_post(module_accessor: *mut BattleObjectModuleAccessor, cat1_prev: i32) {
    unsafe {
        // Cull Attack inputs if grab is used
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
        && cat1_prev & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0 {
            if !ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                for attack in 0..7 {
                    ControlModule::clear_command_one(module_accessor, 0, attack);
                }
            }
            else {
                ControlModule::clear_command_one(module_accessor, 0, 0x1d);
            }
        }
    }
}

pub static mut EXEC_CONTROL_MODULE : u64 = 0;

#[skyline::hook(offset = 0x6bac10)]
fn exec_command_hook(control_module: u64, flag: bool) {
    unsafe {EXEC_CONTROL_MODULE = control_module;}
    let module_accessor = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };

    exec_internal(module_accessor);
    let cat1_prev = unsafe{ ControlModule::get_command_flag_cat(module_accessor, 0) };
    call_original!(control_module, flag);
    exec_post(module_accessor, cat1_prev);
}

// These 2 hooks prevent buffered nair after inputting C-stick on first few frames of jumpsquat
// Both found in ControlModule::exec_command
#[skyline::hook(offset = 0x6be630)]
unsafe fn set_attack_air_stick_hook(control_module: u64, arg: u32) {
    // This check passes on the frame FighterControlModuleImpl::reserve_on_attack_button is called
    // Only happens during jumpsquat currently
    // let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if *((control_module + 0x645) as *const bool) {
        return;
    }
    call_original!(control_module, arg);
}

// #[skyline::hook(offset = 0x6bd6c4, inline)]
// unsafe fn exec_command_reset_attack_air_kind_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     let control_module = *ctx.registers[21].x.as_ref();
//     let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
//     // For some reason, the game resets your attack_air_kind value every frame
//     // even though it resets as soon as you perform an aerial attack
//     // We don't want this to reset while in jumpsquat
//     // to allow the game to use your initial C-stick input during jumpsquat for your attack_air_kind
//     if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
//         ControlModule::reset_attack_air_kind(boma);
//     }
// }

const PRECEDE_EXTENSION : u8 = 24;

#[skyline::hook(offset = 0x6bd5b4, inline)]
unsafe fn set_hold_buffer_value(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *(EXEC_CONTROL_MODULE as *mut *mut BattleObjectModuleAccessor).add(1);
    let cat = *ctx.registers[24].w.as_ref();
    let flag = *ctx.registers[22].w.as_ref() as i32;
    let current_buffer = *ctx.registers[8].w.as_ref();
    let threshold = u8::MAX - PRECEDE_EXTENSION;
    let mut buffer = if current_buffer == 1 {
        // println!("Starting Hold Buffer");
        u8::MAX as u32
    }
    else if current_buffer == threshold as u32 {
        // println!("Hold Threshold Reached");
        1
    }
    else {
        current_buffer
    };

    if buffer > threshold as u32 {
        // special handling for spotdodge and roll
        if cat == 0 {
            if flag == *FIGHTER_PAD_CMD_CAT1_ESCAPE {
                let stick_y = ControlModule::get_stick_y(module_accessor);
                let escape_stick_y = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("escape_stick_y"));
                if stick_y > escape_stick_y {
                    buffer = 1;
                }
            }
            else if flag == *FIGHTER_PAD_CMD_CAT1_ESCAPE_F
            || flag == *FIGHTER_PAD_CMD_CAT1_ESCAPE_B {
                let stick_x = ControlModule::get_stick_x(module_accessor);
                let escape_fb_stick_x = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("escape_fb_stick_x"));
                if stick_x.abs() < escape_fb_stick_x {
                    buffer = 1;
                }
            }
        }
    }

    *ctx.registers[8].w.as_mut() = buffer;
}

#[skyline::hook(offset = 0x6bd51c, inline)]
unsafe fn set_release_value_in_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    set_release_value_internal(ctx);
}

#[skyline::hook(offset = 0x6bd5d8, inline)]
unsafe fn set_release_value(ctx: &mut skyline::hooks::InlineCtx) {
    set_release_value_internal(ctx);
}

unsafe fn set_release_value_internal(ctx: &mut skyline::hooks::InlineCtx) {
    // let control_scuffed = *ctx.registers[21].x.as_ref() as *mut ControlModuleScuffed;
    // let precede_extension = (*(*control_scuffed).something).precede_extension;
    // println!("precede_extension: {:#x}", precede_extension);
    let threshold = u8::MAX - PRECEDE_EXTENSION;
    // println!("precede_extension threshold: {:#x}", threshold);
    let current_buffer = ctx.registers[9].w.as_ref();
    // println!("current: {:#x}", *current_buffer);
    let buffer = if *current_buffer < threshold as u32 {
        *current_buffer
    }
    else {
        1
    };
    *ctx.registers[8].w.as_mut() = buffer as u32;
}

// #[skyline::hook(offset = 0x6bd5ec, inline)]
// unsafe fn get_buffer_value(ctx: &mut skyline::hooks::InlineCtx) {
//     let cat = *ctx.registers[24].w.as_ref();
//     let flag = *ctx.registers[22].w.as_ref();
//     if cat == 0 && flag == 25 {
//         let buffer = *ctx.registers[8].w.as_ref();
//         println!("ESCAPE_F buffer: {:#x}", buffer);
//     }
// }

pub fn install() {
    // Prevents buffered C-stick aerials from triggering nair
    skyline::patching::Patch::in_text(0x6be664).data(0x52800040);

    // Prevents Aerial Kind resetting every frame
    skyline::patching::Patch::in_text(0x6bd6c4).nop();

    // Removes 10f C-stick lockout for tilt stick and special stick
    skyline::patching::Patch::in_text(0x17532ac).data(0x2A1F03FA);
    skyline::patching::Patch::in_text(0x17532b0).nop();
    skyline::patching::Patch::in_text(0x17532b4).nop();
    skyline::patching::Patch::in_text(0x17532b8).nop();

    // Custom buffer-state handling
    // Always uses the hitlag handling that cat4 uses
    skyline::patching::Patch::in_text(0x6bd448).nop();
    skyline::patching::Patch::in_text(0x6bd4a4).nop();
    // Stubs the check if the buffer value is 1 and the button is held
    skyline::patching::Patch::in_text(0x6bd5b0).nop();
    // Stubs setting the buffer lifetime to 2 if held
    skyline::patching::Patch::in_text(0x6bd53c).nop();
    skyline::patching::Patch::in_text(0x6bd5b4).nop();

    skyline::install_hooks!(
        // get_command_flag_cat_replace,
        exec_command_hook,
        set_attack_air_stick_hook,
        // exec_command_reset_attack_air_kind_hook,
        set_hold_buffer_value,
        set_release_value_in_hitlag,
        set_release_value,
        // get_buffer_value
    );
}
