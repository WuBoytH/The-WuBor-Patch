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
    wubor_utils::table_const::*
};

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn buddy_attackdash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32, // originally *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32,
        0
    );
    0.into()
}

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn buddy_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_dash"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log = fighter.get_mini_jump_attack_data_log_info(hash40("attack_dash").into()).get_i64();
    // this looks different from the common status decomp
    // however the devs got tired of writing the following so they made a helper function for it at some point
    // pLVar8 = (L2CValue *)lib::L2CValue::operator[](&this->field_0x118,0x9ceb46930);
    // pLVar8 = (L2CValue *)lib::L2CValue::operator[](pLVar8,0xb6c751ced);
    WorkModule::set_int64(fighter.module_accessor, log, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let info = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, info);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[0x15].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(buddy_attackdash_main_loop as *const () as _))
}

unsafe extern "C" fn buddy_attackdash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() { // addition
            return 0.into();
        }
    }
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    // new stuff
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y * 0.5
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    //back to original stuff kinda?
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if situation == *SITUATION_KIND_GROUND { // this is not original but this is so you can't attack cancel... in the air.
        if 0 < jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor)
            && fighter.sub_check_button_jump().get_bool() {
                let log = fighter.status_attack();
                let info = log[0x10f40d7b92 as u64].get_i64();
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EXPRESSION,
                    Hash40::new_raw(mot),
                    -1
                );
                WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == jump_attack_frame {
            if fighter.global_table[IN_HITLAG].get_bool()
            && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
                let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
            let cont;
            if stick_x * lr < turn_run_stick_x {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    cont = !ItemModule::is_have_item(fighter.module_accessor, 0);
                }
                else { cont = false; }
            }
            else { cont = false; }
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                return 0.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) {
            let cont;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                cont = !ItemModule::is_have_item(fighter.module_accessor, 0);
            }
            else { cont = false; }
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation == *SITUATION_KIND_GROUND {
            let attack_dash_end_type = WorkModule::get_param_int(fighter.module_accessor, 0x17e10662a4, 0);
            if attack_dash_end_type != *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        buddy_attackdash_pre,
        buddy_attackdash_main
    );
}