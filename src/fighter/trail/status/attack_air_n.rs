use {
    crate::imports::status_imports::*,
    crate::fighter::common::status::attack::attack::*
};

unsafe extern "C" fn trail_attack_air_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
    if !StopModule::is_stop(fighter.module_accessor) {
        trail_attack_air_n_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(trail_attack_air_n_substatus as *const () as _));
    trail_attack_air_n_reset(fighter, false.into());
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_ATTACK_AIR_BUTTON_COUNT);
    if 2 <= count {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CONNECT_COMBO);
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_attack_air_n_main_loop as *const () as _))
}

unsafe extern "C" fn trail_attack_air_n_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.attack_air_uniq(param_1).get_bool() {
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

unsafe extern "C" fn trail_attack_air_n_reset(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
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
        trail_attack_air_n_mot_helper(fighter);
    }
}

unsafe extern "C" fn trail_attack_air_n_mot_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn trail_attack_air_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                    VarModule::set_int(fighter.module_accessor, fighter::status::int::ENABLED_AERIALS, flags);
                    if aerial_cancel_common(fighter).get_bool() {
                        return 1.into();
                    }
                }
                else {
                    trail_attack_air_n_reset(fighter, true.into());
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

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, trail_attack_air_n_main);
}