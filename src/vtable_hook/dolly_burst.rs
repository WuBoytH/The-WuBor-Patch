use crate::imports::*;
use std::arch::asm;

static mut BURST_BOMA_PTR : u64 = 0;

#[skyline::hook(offset = 0x975878, inline)]
unsafe extern "C" fn burst_set_pos(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    BURST_BOMA_PTR = boma as u64;
    let mut offset = WorkModule::get_param_float(boma, hash40("param_super_special"), hash40("burst_offset_z"));
    let num = VarModule::get_int(boma, dolly::status::int::SUPER_SPECIAL_TRIPLE_COUNT).max(1);
    offset *= num as f32;
    // println!("offset: {}", offset as f32);
    asm!("fmov s0, w8", in("w8") offset);
}

#[skyline::hook(offset = 0x9758bc, inline)]
unsafe extern "C" fn burst_set_motion(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = BURST_BOMA_PTR as *mut BattleObjectModuleAccessor;
    let num = VarModule::get_int(boma, dolly::status::int::SUPER_SPECIAL_TRIPLE_COUNT);
    let motion = match num {
        1 => hash40("super_special_triple_1"),
        2 => hash40("super_special_triple_2"),
        3 => hash40("super_special_triple_3"),
        _ => hash40("super_special")
    };
    *ctx.registers[8].x.as_mut() = motion;
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x975878).nop();
    skyline::install_hooks!(
        burst_set_pos,
        burst_set_motion
    );
}