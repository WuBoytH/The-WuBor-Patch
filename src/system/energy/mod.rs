use crate::imports::*;
use wubor_utils::controls::*;
use std::arch::asm;

#[skyline::hook(offset = 0x6ce6d8, inline)]
unsafe extern "C" fn jump1_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d19c4, inline)]
unsafe extern "C" fn jump2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d1b10, inline)]
unsafe extern "C" fn jump3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d0454, inline)]
unsafe extern "C" fn jump4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce7d0, inline)]
unsafe extern "C" fn jump_aerial_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d05cc, inline)]
unsafe extern "C" fn jump_aerial_2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d117c, inline)]
unsafe extern "C" fn jump_aerial_3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce28c, inline)]
unsafe extern "C" fn jump_aerial_4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);    
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_x(boma)
    }
    else {
        ControlModule::get_stick_x(boma)
    };
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d253c, inline)]
unsafe extern "C" fn jump_speed_y_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let module_accessor = &mut *(*((work_module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let mul = if VarModule::is_flag(module_accessor, fighter::instance::flag::SUPER_JUMP) {
        1.2
    }
    else {
        1.0
    };
    let jump_y = callable(work_module, hash40("jump_speed_y"), 0) * mul;
    asm!("fmov s0, w8", in("w8") jump_y)
}


mod control;

pub fn install() {
    // Stubs ControlModule::get_stick_x calls when calculating horizontal jump velocity
    skyline::patching::Patch::in_text(0x6ce6d8).nop();
    skyline::patching::Patch::in_text(0x6d19c4).nop();
    skyline::patching::Patch::in_text(0x6d1b10).nop();
    skyline::patching::Patch::in_text(0x6d0454).nop();

    // Same as above but for double jumps
    skyline::patching::Patch::in_text(0x6ce7d0).nop();
    skyline::patching::Patch::in_text(0x6d05cc).nop();
    skyline::patching::Patch::in_text(0x6d117c).nop();
    skyline::patching::Patch::in_text(0x6ce28c).nop();

    // Super Jump Speed Multiplier
    skyline::patching::Patch::in_text(0x6d253c).nop();

    // Always use Jump Speed Y
    skyline::patching::Patch::in_text(0x6d217c).data(0x140000EBu32);

    skyline::install_hooks!(
        jump1_stick_x_hook,
        jump2_stick_x_hook,
        jump3_stick_x_hook,
        jump4_stick_x_hook,
        jump_aerial_stick_x_hook,
        jump_aerial_2_stick_x_hook,
        jump_aerial_3_stick_x_hook,
        jump_aerial_4_stick_x_hook,
        jump_speed_y_hook
    );

    control::install();
}