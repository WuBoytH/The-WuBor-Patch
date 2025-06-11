use super::*;

unsafe extern "C" fn demon_attack_step_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_step"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    FighterControlModuleImpl::delete_command(fighter.module_accessor, 3, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB);
    FighterControlModuleImpl::delete_command(fighter.module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_DASH);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1 & *FIGHTER_PAD_CMD_CAT1_DASH));
    ControlModule::reset_flick_x(fighter.module_accessor);

    let cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_attack_step"), hash40("cancel_frame"));
    WorkModule::set_int(fighter.module_accessor, cancel_frame, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME);

    if !StopModule::is_stop(fighter.module_accessor) {
        demon_attack_step_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(demon_attack_step_substatus as *const () as _));

    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);

    let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    FighterSpecializer_Demon::add_attack_log(&mut *fighta, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_11, false);

    let move_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_attack_step"), hash40("move_mul"));
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        move_mul
    );
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_step_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME, 0);
    }
    0.into()
}

unsafe extern "C" fn demon_attack_step_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
        return 0.into();
    }

    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
        return 0.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
        let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
            FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE
        }
        else {
            FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L
        };
        fighter.change_status(status.into(), true.into());
        return 0.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
        return 0.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_CATCH_COMMAND.into(), true.into());
        return 0.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
        let frame = fighter.global_table[STATUS_FRAME].get_f32();
        let step2f_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_attack_step"), hash40("step2f_frame"));
        if frame <= step2f_frame as f32 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F.into(), true.into());
            return 0.into();
        }
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6 != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into());
            return 0.into();
        }
    }

    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6 != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into());
            return 0.into();
        }
    }

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let vec = fighter.Vector2__create(stick_x.into(), stick_y.into());
        let len = fighter.Vector2__length(vec).get_f32();
        println!("len: {}", len);
        let special_command_neutral_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_command_neutral_threshold"));
        if len <= special_command_neutral_threshold {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL);
        }
    }

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_WORK_INT_CANCEL_FRAME) > 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_NEUTRAL) {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return 0.into();
            }

            let cat1 = fighter.global_table[CMD_CAT1].get_i32();

            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
                demon_attack_step_command_input_helper(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
                return 0.into();
            }

            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
                demon_attack_step_command_input_helper(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                return 0.into();
            }

            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
                demon_attack_step_command_input_helper(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
                return 0.into();
            }

            if fighter.sub_check_command_walk().get_bool()
            && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
                demon_attack_step_command_input_helper(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
                return 0.into();
            }
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn demon_attack_step_command_input_helper(fighter: &mut L2CFighterCommon) {
    let step = FighterControlModuleImpl::special_command_623_step(fighter.module_accessor);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_SPECIAL_HI_COMMAND);
    }
    let step = FighterControlModuleImpl::special_command_step(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB);
    }
    let step = FighterControlModuleImpl::special_command_step(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623STRICT);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623STRICT);
    }
    let step = FighterControlModuleImpl::special_command_step(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623ALONG);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623ALONG);
    }
    let step = FighterControlModuleImpl::special_command_step(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623BLONG);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623BLONG);
    }
    let step = FighterControlModuleImpl::special_command_step(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623A);
    if 3 <= step {
        FighterControlModuleImpl::reset_special_command_individual(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623A);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, demon_attack_step_main);
}