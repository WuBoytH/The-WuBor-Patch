use {
    smash::{
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    crate::vars::*
};

#[inline(always)]
pub unsafe fn spent_meter(module_accessor: *mut BattleObjectModuleAccessor, onemore: bool) -> bool {
    let mut spent = false;
    if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) > 0.0 {
        if WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            if onemore {
                WorkModule::set_float(module_accessor, 12.5, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
                spent = true;
                WorkModule::set_int(module_accessor, 600, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
            }
            else {
                WorkModule::set_float(module_accessor, 6.25, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
                spent = true;
                WorkModule::set_int(module_accessor, 600, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
            }
        }
        else if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 25.0 {
            WorkModule::set_float(module_accessor, 25.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
            spent = true;
            WorkModule::set_int(module_accessor, 300, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
        }
    }
    if spent {
        WorkModule::set_float(module_accessor, 360.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY);
    }
    return spent;
}

#[inline(always)]
pub unsafe fn spent_meter_super(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let mut spent = false;
    if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) > 0.0 {
        if WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            WorkModule::set_float(module_accessor, 25.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
            spent = true;
        }
        else if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 50.0 {
            WorkModule::set_float(module_accessor, 50.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
            spent = true;
        }
    }
    if spent {
        WorkModule::set_int(module_accessor, 300, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
        WorkModule::set_float(module_accessor, 360.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY);
    }
    return spent;
}

#[inline(always)]
pub unsafe fn upper_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    else {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[inline(always)]
pub unsafe fn full_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[inline(always)]
pub unsafe fn shadow_id(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

#[inline(always)]
pub unsafe fn sp_glow_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    let onemoreeff: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handr"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    let onemoreeff2: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handl"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rate(module_accessor, onemoreeff, 2.0);
    EffectModule::set_rate(module_accessor, onemoreeff2, 2.0);
    if WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
        EffectModule::set_rgb(module_accessor, onemoreeff, 0.6, 0.0, 1.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 0.6, 0.0, 1.0);
    }
    else {
        EffectModule::set_rgb(module_accessor, onemoreeff, 1.0, 0.8, 0.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 1.0, 0.8, 0.0);
    }
}

#[inline(always)]
pub unsafe fn sp_gauge_handler(module_accessor: *mut BattleObjectModuleAccessor, remove: bool) {
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_starrod_bullet"), false, true);
    if !remove {
        let mut level = WorkModule::get_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL);
        if WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            level += 1;
        }
        while level > 0 {
            let pos;
            match level {
                2 => pos = SP_2,
                3 => pos = SP_3,
                4 => pos = SP_4,
                5 => pos = SP_5,
                6 => pos = SP_6,
                _ => pos = SP_1,
            }
            EffectModule::req_follow(module_accessor, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), &pos, &ZERO_VECTOR, 0.3, false, 0, 0, 0, 0, 0, false, false);
            level -= 1;
        }
    }
}

#[inline(always)]
pub unsafe fn sp_diff_checker(module_accessor: *mut BattleObjectModuleAccessor) {
    if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) < 25.0 {
        WorkModule::set_int(module_accessor, 0, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL);
    }
    else {
        while WorkModule::get_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL) < 6 {
            if WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= WorkModule::get_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL) as f32 * 25.0
            && WorkModule::get_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL) as f32 * 25.0 > WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) {
                break;
            }
            WorkModule::inc_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL);
        }
    }
    if WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
        WorkModule::set_int(module_accessor, 600, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
    }
    else {
        WorkModule::set_int(module_accessor, 300, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
    }
    let level = (WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) / 25.0) as i32;
    WorkModule::set_int(module_accessor, level, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL);
    if WorkModule::get_int(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL) == 0
    && !WorkModule::is_flag(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
        sp_gauge_handler(module_accessor, true);
    }
    else {
        sp_gauge_handler(module_accessor, false);
    }
}

#[inline(always)]
pub unsafe fn add_sp(module_accessor: *mut BattleObjectModuleAccessor, amount: f32) {
    let mut sp_gauge = WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
    sp_gauge += amount;
    if sp_gauge < 0.0 {
        sp_gauge = 0.0;
    }
    let sp_gauge_max = WorkModule::get_float(module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
    if sp_gauge > sp_gauge_max {
        sp_gauge = sp_gauge_max;
    }
    WorkModule::set_float(module_accessor, sp_gauge, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
}
