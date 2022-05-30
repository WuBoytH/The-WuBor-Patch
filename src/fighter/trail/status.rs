use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        vars::*,
        cancels::*,
        table_const::*
    },
    super::super::common::common_status::attack::*
};

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn trail_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    let clone = fighter.global_table[SUB_STATUS].clone();
    fighter.global_table["attack_substatus"].assign(&clone);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(trail_attack_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn trail_attack_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        let normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec();
        if normal_cancel_common(fighter, normal_cancels).get_bool() {
            return 0.into();
        }
    }
    let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table["attack_substatus"].get_ptr());
    callable(fighter, param_1.clone());
    0.into()
}

#[status_script(agent = "trail", status = FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn trail_attackairn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
    if !StopModule::is_stop(fighter.module_accessor) {
        trail_attackairn_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(trail_attackairn_substatus as *const () as _));
    trail_attackairn_reset(fighter, false.into());
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_ATTACK_AIR_BUTTON_COUNT);
    if 2 <= count {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CONNECT_COMBO);
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_attackairn_main_loop as *const () as _))
}

unsafe extern "C" fn trail_attackairn_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.attack_air_uniq(param_1.clone()).get_bool() {
        return 0.into();
    }
    let mut cont = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0;
    if !cont {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                cont = true;
            }
        }
    }
    if cont {
        if ComboModule::is_enable_combo_input(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CONNECT_COMBO);
        }
    }
    0.into()
}

unsafe extern "C" fn trail_attackairn_reset(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    ControlModule::reset_trigger(fighter.module_accessor);
    AttackModule::clear_all(fighter.module_accessor);
    AttackModule::clear_inflict_kind_status(fighter.module_accessor);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_CANCEL_UNABLE_CANCEL);
    sv_module_access::cancel(fighter.lua_state_agent);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CONNECT_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_HIT_SPEED_Y);
    if param_1.get_bool() {
        ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_AIR_N_COMBINATION);
        trail_attackairn_mot_helper(fighter);
    }
}

unsafe extern "C" fn trail_attackairn_mot_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let count = ComboModule::count(fighter.module_accessor);
    let mot;
    let kind;
    if count == 1 {
        mot = Hash40::new("attack_air_n");
        kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N;
    }
    else if count == 2 {
        mot = Hash40::new("attack_air_n2");
        kind = FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01;
    }
    else {
        mot = Hash40::new("attack_air_n3");
        kind = FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02;
    }
    if *kind != *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N {
        MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if *kind != *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N {
        MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, kind);
    WorkModule::set_int64(fighter.module_accessor, mot.hash as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}

unsafe extern "C" fn trail_attackairn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_AttackAir_Main_common().get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let count = ComboModule::count(fighter.module_accessor) as i32;
        let combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_air_n_combo_max"));
        let mut cont = count < combo_max;
        if cont {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CONNECT_COMBO) {
                cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO);
            }
            else {
                cont = false;
            }
            if cont {
                if !only_jabs(fighter) {
                    let flags = ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
                    WorkModule::set_int(fighter.module_accessor, flags, FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS);
                    if aerial_cancel_common(fighter).get_bool() {
                        return 1.into();
                    }
                }
                else {
                    trail_attackairn_reset(fighter, true.into());
                }
            }
        }
    }
    fighter.sub_air_check_superleaf_fall_slowly();
    if !fighter.global_table[IS_STOP].get_bool() {
        fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
    }
    0.into()
}

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn trail_landingattackair_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let mut motion_rate: f32 = 1.0;
    let landing_param_type;
    let mut landing_param : u64 = 0;
    let landing_mot;
    if mot == hash40("attack_air_n") {
        landing_param_type = hash40("landing_attack_air_frame_n");
        landing_mot = hash40("landing_air_n");
    }
    else if mot == hash40("attack_air_f") {
        landing_param_type = hash40("landing_attack_air_frame_f");
        landing_mot = hash40("landing_air_f");
    }
    else if mot == hash40("attack_air_b") {
        landing_param_type = hash40("landing_attack_air_frame_b");
        landing_mot = hash40("landing_air_b");
    }
    else if mot == hash40("attack_air_hi") {
        landing_param_type = hash40("landing_attack_air_frame_hi");
        landing_mot = hash40("landing_air_hi");
    }
    else if mot == hash40("attack_air_lw") {
        landing_param_type = hash40("landing_attack_air_frame_lw");
        landing_mot = hash40("landing_air_lw");
    }
    else {
        landing_param_type = hash40("param_private");
        let luaconst;
        let temp1;
        if mot == hash40("attack_air_n2") {
            landing_param = hash40("landing_attack_air_frame_n2");
            landing_mot = hash40("landing_air_n2");
            temp1 = hash40("landing_attack_air_frame_n");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_N;
        }
        else if mot == hash40("attack_air_n3") {
            landing_param = hash40("landing_attack_air_frame_n3");
            landing_mot = hash40("landing_air_n3");
            temp1 = hash40("landing_attack_air_frame_n");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_N;
        }
        else if mot == hash40("attack_air_f2") {
            landing_param = hash40("landing_attack_air_frame_f2");
            landing_mot = hash40("landing_air_f2");
            temp1 = hash40("landing_attack_air_frame_f");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F;
        }
        else  {
            landing_param = hash40("landing_attack_air_frame_f3");
            landing_mot = hash40("landing_air_f3");
            temp1 = hash40("landing_attack_air_frame_f");
            luaconst = *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F;
        }
        let lag = WorkModule::get_param_float(fighter.module_accessor, temp1, 0);
        let other_lag = WorkModule::get_float(fighter.module_accessor, luaconst);
        motion_rate = lag / other_lag;
    }
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, landing_param);
    landing_lag *= motion_rate;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF) {
        landing_lag += 4.0;
    }
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(landing_mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(landing_mot),
        0.0,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_RUN_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        trail_attack_main,
        trail_attackairn_main,
        trail_landingattackair_init
    );
}