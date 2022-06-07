use {
    smash::{
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*},
    super::vl
};

pub unsafe fn add_sp(object: *mut BattleObject, module_accessor: *mut BattleObjectModuleAccessor, mut amount: f32) {
    let meter_max = VarModule::get_float(object, yu::instance::float::SP_GAUGE_MAX);
    let meter_const = yu::instance::float::SP_GAUGE;
    if !VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
        if !VarModule::is_flag(object, yu::status::flag::IS_EX) {
            if shadow_id(module_accessor) == false {
                amount *= 0.75;
            }
            if VarModule::get_float(object, yu::instance::float::SP_GAIN_PENALTY) > 0.0 {
                amount *= 0.1;
            }
            FGCModule::update_meter(object, amount, meter_max, meter_const);
        }
    }
}

pub unsafe fn spent_meter(object: *mut BattleObject, onemore: bool) -> bool {
    let mut spent = false;
    let sp = VarModule::get_float(object, yu::instance::float::SP_GAUGE);
    if sp > 0.0 {
        if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
            if onemore {
                VarModule::set_float(
                    object,
                    yu::instance::float::SPENT_SP,
                    vl::param_private::sp_onemore_shadow,
                );
                spent = true;
            }
            else {
                VarModule::set_float(
                    object,
                    yu::instance::float::SPENT_SP,
                    vl::param_private::sp_ex_shadow
                );
                spent = true;
            }
            if spent {
                VarModule::set_int(
                    object,
                    yu::instance::int::SP_EFFECT_TIMER,
                    vl::param_private::sp_effect_timer_shadow
                );
            }
        }
        else if sp >= vl::param_private::sp_single {
            VarModule::set_float(
                object,
                yu::instance::float::SPENT_SP,
                vl::param_private::sp_single
            );
            spent = true;
            VarModule::set_int(
                object,
                yu::instance::int::SP_EFFECT_TIMER,
                vl::param_private::sp_effect_timer
            );
        }
    }
    if spent {
        VarModule::set_float(
            object,
            yu::instance::float::SP_GAIN_PENALTY,
            vl::param_private::sp_gain_penalty
        );
    }
    return spent;
}

pub unsafe fn spent_meter_super(object: *mut BattleObject) -> bool {
    let mut spent = false;
    let sp = VarModule::get_float(object, yu::instance::float::SP_GAUGE);
    if sp > 0.0 {
        if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
            VarModule::set_float(
                object,
                yu::instance::float::SPENT_SP,
                vl::param_private::sp_super_shadow
            );
            VarModule::set_int(
                object,
                yu::instance::int::SP_EFFECT_TIMER,
                vl::param_private::sp_effect_timer_shadow
            );
            spent = true;
        }
        else if sp >= vl::param_private::sp_super {
            VarModule::set_float(
                object,
                yu::instance::float::SPENT_SP,
                vl::param_private::sp_super
            );
            VarModule::set_int(
                object,
                yu::instance::int::SP_EFFECT_TIMER,
                vl::param_private::sp_effect_timer
            );
            spent = true;
        }
    }
    if spent {
        VarModule::set_float(
            object,
            yu::instance::float::SP_GAIN_PENALTY,
            vl::param_private::sp_gain_penalty
        );
    }
    return spent;
}

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

pub unsafe fn full_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub unsafe fn shadow_id(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6
    || color == 7 {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn sp_glow_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    let onemoreeff: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handr"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    let onemoreeff2: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handl"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rate(module_accessor, onemoreeff, 2.0);
    EffectModule::set_rate(module_accessor, onemoreeff2, 2.0);
    let object_id = (*module_accessor).battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
        EffectModule::set_rgb(module_accessor, onemoreeff, 0.6, 0.0, 1.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 0.6, 0.0, 1.0);
    }
    else {
        EffectModule::set_rgb(module_accessor, onemoreeff, 1.0, 0.8, 0.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 1.0, 0.8, 0.0);
    }
}

pub unsafe fn sp_gauge_handler(module_accessor: *mut BattleObjectModuleAccessor, remove: bool) {
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_starrod_bullet"), false, true);
    if !remove {
        let object_id = (*module_accessor).battle_object_id;
        let object = sv_system::battle_object(object_id as u64);
        let mut level = VarModule::get_int(object, yu::instance::int::SP_LEVEL);
        if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
            level += 1;
        }
        while level > 0 {
            let pos;
            match level {
                2 => pos = yu::SP_2,
                3 => pos = yu::SP_3,
                4 => pos = yu::SP_4,
                5 => pos = yu::SP_5,
                6 => pos = yu::SP_6,
                _ => pos = yu::SP_1,
            }
            EffectModule::req_follow(module_accessor, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), &pos, &ZERO_VECTOR, 0.3, false, 0, 0, 0, 0, 0, false, false);
            level -= 1;
        }
    }
}

pub unsafe fn sp_diff_checker(module_accessor: *mut BattleObjectModuleAccessor) {
    let object_id = (*module_accessor).battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    let sp = VarModule::get_float(object, yu::instance::float::SP_GAUGE);
    let level = sp / vl::param_private::sp_single;
    VarModule::set_int(object, yu::instance::int::SP_LEVEL, level as i32);
    if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY) {
        VarModule::set_int(object, yu::instance::int::SP_EFFECT_TIMER, vl::param_private::sp_effect_timer_shadow);
    }
    else {
        VarModule::set_int(object, yu::instance::int::SP_EFFECT_TIMER, vl::param_private::sp_effect_timer);
    }
    let mut level = VarModule::get_int(object, yu::instance::int::SP_LEVEL);
    if VarModule::is_flag(object, yu::instance::flag::SHADOW_FRENZY)
    && sp > 0.0 {
        level += 1;
    }
    if level == 0 {
        sp_gauge_handler(module_accessor, true);
    }
    else {
        sp_gauge_handler(module_accessor, false);
    }
}
