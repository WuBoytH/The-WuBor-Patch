// #![allow(improper_ctypes)]
use crate::imports::*;

// extern "C" {
//     #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__get_touch_pos_implEPNS_26BattleObjectModuleAccessorEj"]
//     pub fn get_touch_pos_simd(
//         module_accessor: *mut smash::app::BattleObjectModuleAccessor,
//         arg2: libc::c_uint,
//     ) -> smash_rs::cpp::simd::Vector2;
// }

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyReflect_effect)]
unsafe extern "C" fn sub_damageflyreflect_effect(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let disable_passive = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    }
    original!()(fighter, param_1);
    WorkModule::set_flag(fighter.module_accessor, disable_passive, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
}

#[skyline::hook(replace = L2CFighterCommon_sub_damage_fly_reflect_d_uniq_process_init)]
unsafe extern "C" fn sub_damage_fly_reflect_d_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let disable_passive = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    }
    let ret = original!()(fighter);
    WorkModule::set_flag(fighter.module_accessor, disable_passive, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    ret
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectD)]
unsafe extern "C" fn status_damageflyreflectd(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_MIIFIGHTER_100KICK_SET_REFLECT_ANGLE) {
        let mut some = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x25e72e042c);
        let miifighter_100kick_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MIIFIGHTER_100KICK_LR);
        if miifighter_100kick_lr < 0.0 {
            some = 180.0 - some;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
        let speed_x = speed_length * some.to_radians().cos();
        let speed_y = speed_length * some.to_radians().sin();
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
            speed_x,
            speed_y
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectD_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectLR)]
unsafe extern "C" fn status_damageflyreflectlr(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    SoundModule::play_se(
        fighter.module_accessor,
        Hash40::new("se_common_down_soil_s"),
        true,
        false,
        false,
        false,
        enSEType(0)
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectLR_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectU)]
unsafe extern "C" fn status_damageflyreflectu(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectU_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_damageflyreflect_effect,
            sub_damage_fly_reflect_d_uniq_process_init,
            status_damageflyreflectd,
            status_damageflyreflectlr,
            status_damageflyreflectu
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}