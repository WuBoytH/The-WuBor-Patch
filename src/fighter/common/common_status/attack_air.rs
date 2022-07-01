use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*},
    super::super::common_param
};

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_uniq_process_init)]
unsafe fn sub_attack_air_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI)
    && VarModule::is_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP) {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -common_param::jump::super_jump_gravity
        );
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_common)]
unsafe fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    VarModule::off_flag(fighter.battle_object, attack_air::flag::WHIFF);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_air_uniq as *const () as _));
    if param_1.get_bool() {
        fighter.sub_attack_air_kind();
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP) {
        let super_jump_frame = VarModule::get_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME);
        if fighter.global_table[MOTION_FRAME].get_f32() >= 9.0 - super_jump_frame {
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
            VarModule::set_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME, 0.0);
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        VarModule::off_flag(fighter.battle_object, attack_air::flag::WHIFF);
    }
    else if !VarModule::is_flag(fighter.battle_object, attack_air::flag::WHIFF) {
        let part_size = AttackModule::part_size(fighter.module_accessor) as i32;
        for id in 0..part_size {
            if AttackModule::is_attack(fighter.module_accessor, id, false) {
                VarModule::on_flag(fighter.battle_object, attack_air::flag::WHIFF);
                break;
            }
        }
    }
    if !fighter.attack_air_common_strans().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if !MotionModule::is_end(fighter.module_accessor) {
                return false.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(fighter.module_accessor) {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackAir)]
unsafe fn bind_address_call_status_end_attackair(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
    VarModule::set_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME, 0.0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackAir)]
unsafe fn status_end_attackair(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    VarModule::off_flag(fighter.battle_object, commons::instance::flag::SUPER_JUMP);
    VarModule::set_float(fighter.battle_object, commons::instance::float::SUPER_JUMP_FRAME, 0.0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_landing_attack_air_init)]
unsafe fn sub_landing_attack_air_init(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let mot = param_1.get_int();
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, param_2.get_int(), 0) + param_3.get_f32();
    if VarModule::is_flag(fighter.battle_object, attack_air::flag::WHIFF) {
        landing_lag += 4.0;
    }
    let mut motion_rate = 1.0;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_uniq_process_init,
            sub_attack_air_common,
            status_attackair_main_common,
            bind_address_call_status_end_attackair,
            status_end_attackair,
            sub_landing_attack_air_init
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}