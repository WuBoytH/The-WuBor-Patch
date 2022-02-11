#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_ftStatusUniqProcessDamage_init_common)]
unsafe fn ftstatusuniqprocessdamage_init_common(fighter: &mut L2CFighterCommon) {
    call_original!(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN) {
        let disable_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME) as f32;
        let disable_escape = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME) as f32;
        let hitstun_mul = WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL);
        let new_disable_attack = (disable_attack as f32 * hitstun_mul) as i32;
        let new_disable_escape = (disable_escape as f32 * hitstun_mul) as i32;
        WorkModule::set_int(fighter.module_accessor, new_disable_attack, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
        WorkModule::set_int(fighter.module_accessor, new_disable_escape, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
        if WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL) > 0.5 {
            WarkModule::add_f32(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL, -0.05);
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL) < 0.5 {
                WorkModule::set_float(fighter.module_accessor, 0.5, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL);
            }
        }
    }
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            ftstatusuniqprocessdamage_init_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}