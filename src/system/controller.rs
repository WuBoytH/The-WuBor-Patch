use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::L2CValue
    },
    wubor_utils::{controls::*, table_const::*},
};

#[skyline::hook(offset = 0x16d948c, inline)]
unsafe fn packed_packet_creation(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[22].x.as_mut() = 0x2;
}

#[skyline::hook(offset = 0x16d94c0, inline)]
unsafe fn write_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let raw = *ctx.registers[19].x.as_ref();

    let mapped_inputs = *((raw + 0x49508) as *const MappedInputs);
    let mut packet = 0u64;

    *(&mut packet as *mut u64 as *mut i8) = mapped_inputs.lstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(1) = mapped_inputs.lstick_y;

    let buttons = (mapped_inputs.buttons.bits() as u64) << 16;
    packet |= buttons;

    *(&mut packet as *mut u64 as *mut i8).add(6) = mapped_inputs.rstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(7) = mapped_inputs.rstick_y;

    *ctx.registers[8].x.as_mut() = packet;
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller,
}

macro_rules! apply_button_mappings {
    ($controller:ident, $mappings:ident, $(($button:ident, $mapped:ident, $kind:ident, $output:expr))*) => {{
        let mut buttons = Buttons::empty();
        $(
                if $controller.current_buttons.$button() && (*$mappings).$mapped == InputKind::$kind {
                    buttons |= $output;
                }
        )*
        buttons
    }}
}

#[skyline::hook(offset = 0x17504a0)]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool,
) {
    let entry_count = (*mappings.add(player_idx as usize))._34[0];
    let _ret = if controller_struct.controller.style == ControllerStyle::GCController {
        // Used for analog shields
        // let is_r_press = if (*mappings.add(player_idx as usize)).gc_r == InputKind::Guard {
        //     let press = Some(controller_struct.controller.current_buttons.r());
        //     controller_struct.controller.current_buttons.set_r(false);
        //     press
        // } else {
        //     None
        // };

        // let is_l_press = if (*mappings.add(player_idx as usize)).gc_l == InputKind::Guard {
        //     let press = Some(controller_struct.controller.current_buttons.l());
        //     controller_struct.controller.current_buttons.set_l(false);
        //     press
        // } else {
        //     None
        // };

        let ab_smash = (*mappings.add(player_idx as usize)).gc_absmash;
        (*mappings.add(player_idx as usize)).gc_absmash &= 1;
        let ret = original!()(mappings, player_idx, out, controller_struct, arg);
        (*mappings.add(player_idx as usize)).gc_absmash = ab_smash;

        // if let Some(press) = is_r_press {
        //     controller_struct.controller.current_buttons.set_r(press);
        // }

        // if let Some(press) = is_l_press {
        //     controller_struct.controller.current_buttons.set_l(press);
        // }
        ret
    } else {
        if controller_struct.controller.style == ControllerStyle::LeftJoycon
            || controller_struct.controller.style == ControllerStyle::RightJoycon
        {
            let ab_smash = (*mappings.add(player_idx as usize)).joy_absmash;
            (*mappings.add(player_idx as usize)).joy_absmash &= 1;
            let ret = original!()(mappings, player_idx, out, controller_struct, arg);
            (*mappings.add(player_idx as usize)).joy_absmash = ab_smash;
            ret
        } else {
            let ab_smash = (*mappings.add(player_idx as usize)).pro_absmash;
            (*mappings.add(player_idx as usize)).pro_absmash &= 1;
            let ret = original!()(mappings, player_idx, out, controller_struct, arg);
            (*mappings.add(player_idx as usize)).pro_absmash = ab_smash;
            ret
        }
    };
    let controller = &mut controller_struct.controller;

    //println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn)
        && (*mappings.add(player_idx as usize))._34[0] != entry_count
    {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
        (*out).buttons |= Buttons::CStickOverride;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }
}

#[repr(C)]
struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

static mut LAST_ALT_STICK: [f32; 2] = [0.0, 0.0];
static mut LAST_ANALOG: f32 = 0.0;

pub unsafe fn get_mapped_controller_inputs_from_id(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
        .add(0x52c30f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}

#[skyline::hook(offset = 0x3f7220)]
unsafe fn parse_inputs(this: &mut ControlModuleInternal) {
    const NEUTRAL: f32 = 0.2;
    const CLAMP_MAX: f32 = 120.0;

    // println!("this: {:#x}", this as *mut ControlModuleInternal as u64);

    if this.controller_index == -1 {
        return call_original!(this);
    }

    //println!("this.controller_index: {}", this.controller_index);
    // assert!(this.controller_index <= 7);

    let inputs = get_mapped_controller_inputs_from_id(this.controller_index as usize);

    let clamp_mul = 1.0 / CLAMP_MAX;

    // let raw_lstick_x = ((inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    // let raw_lstick_y = ((inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    // let raw_lstick_x = if raw_lstick_x.abs() >= NEUTRAL { raw_lstick_x } else { 0.0 };
    // let raw_lstick_y = if raw_lstick_y.abs() >= NEUTRAL { raw_lstick_y } else { 0.0 };

    let raw_rstick_x = ((inputs.rstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    let raw_rstick_y = ((inputs.rstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    LAST_ALT_STICK[0] = if raw_rstick_x.abs() >= NEUTRAL {
        raw_rstick_x
    } else {
        0.0
    };
    LAST_ALT_STICK[1] = if raw_rstick_y.abs() >= NEUTRAL {
        raw_rstick_y
    } else {
        0.0
    };

    LAST_ANALOG = ((inputs.buttons.bits() >> 22) & 1023) as f32 / 1023.0;

    call_original!(this)
}

#[skyline::hook(offset = 0x6b9c5c, inline)]
unsafe fn after_exec(ctx: &skyline::hooks::InlineCtx) {
    let module = *ctx.registers[19].x.as_ref();
    let internal_class = *(module as *const u64).add(0x110 / 0x8);
    *(internal_class as *mut f32).add(0x40 / 0x4) = LAST_ALT_STICK[0];
    *(internal_class as *mut f32).add(0x44 / 0x4) = LAST_ALT_STICK[1];
    *(internal_class as *mut f32).add(0x48 / 0x4) = LAST_ANALOG;
}

#[skyline::hook(offset = 0x16d7ee4, inline)]
unsafe fn handle_incoming_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let packet = *ctx.registers[15].x.as_ref();

    let mut inputs = MappedInputs {
        buttons: Buttons::empty(),
        lstick_x: 0,
        lstick_y: 0,
        rstick_x: 0,
        rstick_y: 0,
    };

    let raw_buttons = ((packet >> 16) & 0xFFFF_FFFF) as u32;
    let lstick_x = (packet & 0xFF) as i8;
    let lstick_y = ((packet & 0xFF00) >> 8) as i8;
    let rstick_x = ((packet >> 0x30) & 0xFF) as i8;
    let rstick_y = ((packet >> 0x38) & 0xFF) as i8;

    inputs.buttons = Buttons::from_bits_unchecked(raw_buttons as _);
    inputs.lstick_x = lstick_x;
    inputs.lstick_y = lstick_y;
    inputs.rstick_x = rstick_x;
    inputs.rstick_y = rstick_y;

    *ctx.registers[13].x.as_mut() = std::mem::transmute(inputs);
}

/// fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn isthrowstick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut out = fighter.local_func__fighter_status_catch_1();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x;
    let stick_y;
    if Buttons::from_bits_unchecked(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::CStickOverride) {
        stick_x = ControlModule::get_sub_stick_x(fighter.module_accessor);
        stick_y = ControlModule::get_sub_stick_y(fighter.module_accessor);
    }
    else {
        stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    }
    let stick_x = stick_x * lr;
    let throw_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_x"));
    let throw_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi4_stick_y"));
    if stick_x > throw_stick_x {
        out["f"].assign(&true.into());
    } else if stick_x < -throw_stick_x {
        out["b"].assign(&true.into());
    }
    if stick_y > throw_stick_y {
        out["hi"].assign(&true.into());
    } else if stick_y < -throw_stick_y {
        out["lw"].assign(&true.into());
    }
    out
}

static mut GC_TRIGGERS: [f32; 2] = [0.0, 0.0];

#[skyline::hook(offset = 0x3665e2c, inline)]
unsafe fn post_gamecube_process(ctx: &skyline::hooks::InlineCtx) {
    let state: *mut skyline::nn::hid::NpadGcState =
        (ctx as *const _ as *mut u8).add(0x100) as *mut _;
    let _controller: *mut Controller = *ctx.registers[19].x.as_ref() as _;

    GC_TRIGGERS[0] = (*state).LTrigger as f32 / i16::MAX as f32;
    GC_TRIGGERS[1] = (*state).RTrigger as f32 / i16::MAX as f32;
}

#[skyline::hook(offset = 0x3665c8c, inline)]
unsafe fn apply_triggers(ctx: &skyline::hooks::InlineCtx) {
    let controller: *mut Controller = *ctx.registers[19].x.as_ref() as _;
    (*controller).left_trigger = GC_TRIGGERS[0];
    (*controller).right_trigger = GC_TRIGGERS[1];
    GC_TRIGGERS = [0.0, 0.0];
}

#[skyline::hook(offset = 0x3665e60, inline)]
unsafe fn analog_trigger_l(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[9].x.as_ref() & 0x40 != 0 {
        let controller: *mut Controller = *ctx.registers[19].x.as_ref() as _;
        (*controller).current_buttons.set_real_digital_l(true);
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = 0x3665e74, inline)]
unsafe fn analog_trigger_r(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() & 0x80 != 0 {
        let controller: *mut Controller = *ctx.registers[19].x.as_ref() as _;
        (*controller).current_buttons.set_real_digital_r(true);
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(isthrowstick);
    }
}

pub fn install() {
    skyline::patching::Patch::in_text(0x3665e5c).data(0xAA0903EAu32);
    skyline::patching::Patch::in_text(0x3665e70).data(0xAA0803EAu32);
    skyline::install_hooks!(
        map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec,
        post_gamecube_process,
        apply_triggers
    );
    skyline::nro::add_hook(nro_hook);
}
