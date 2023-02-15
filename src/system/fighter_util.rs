use {
    smash::{
        hash40,
        app::*
    }
};

#[skyline::hook(replace = FighterUtil::get_just_shield_se)]
pub unsafe fn get_just_shield_se_replace(fighter_kind: i32) -> u64 {
    match fighter_kind {
        0x3c => hash40("se_ryu_guard_just"),
        0x3d => hash40("se_ken_guard_just"),
        0x55 => hash40("se_dolly_guard_just"),
        _ => hash40("se_common_justshield")
    }
}

pub fn install() {
    skyline::install_hooks!(
        get_just_shield_se_replace
    );
    
}