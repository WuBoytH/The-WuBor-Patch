use smash::{
    hash40,
    phx::*,
    app::*
};

#[skyline::hook(replace = FighterUtil::get_just_shield_se)]
unsafe extern "C" fn get_just_shield_se(fighter_kind: i32) -> u64 {
    match fighter_kind {
        0x3c => hash40("se_ryu_guard_just"),
        0x3d => hash40("se_ken_guard_just"),
        0x55 => hash40("se_dolly_guard_just"),
        _ => hash40("se_common_justshield")
    }
}

#[skyline::hook(replace = FighterUtil::get_cliff_xlu_frame)]
unsafe extern "C" fn get_cliff_xlu_frame(module_accessor: *mut BattleObjectModuleAccessor, motion: Hash40) -> f32 {
    let motion_module = *(module_accessor as *const u64).add(0x88 / 8);
    get_motion_data(motion_module, motion.hash, 0)
}

#[skyline::from_offset(0x6e2440)]
extern "C" fn get_motion_data(motion_module: u64, motion: u64, param_3: u32) -> f32;

pub fn install() {
    // Removed the Diddy fighter kind check from the diddy unlink node FighterUtil function
    let _ = skyline::patching::Patch::in_text(0x6938b4).nop();

    skyline::install_hooks!(
        get_just_shield_se,
        get_cliff_xlu_frame
    );
}