use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_TurnRun_Sub)]
unsafe extern "C" fn status_turnrun_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue, _param_2: L2CValue) {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(param_1.get_u64()),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let turn_run_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("turn_run_frame"));
    WorkModule::set_int(fighter.module_accessor, turn_run_frame, *FIGHTER_STATUS_TURN_RUN_WORK_INT_TURN_FRAME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_RUN_TURN);
    let catch_turn_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_turn_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_turn_frame, *FIGHTER_STATUS_TURN_RUN_WORK_INT_CATCH_FRAME);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN_BRAKE {
        let run_brake_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_RUN_BRAKE_FRAME);
        WorkModule::set_int(fighter.module_accessor, run_brake_frame, *FIGHTER_STATUS_TURN_RUN_WORK_INT_RUN_BRAKE_FRAME);
    }
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_turn_run_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_turn_run_uniq as *const () as _));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    WorkModule::set_float(fighter.module_accessor, speed_x, *FIGHTER_STATUS_TURN_RUN_WORK_FLOAT_START_SPEED);
    // if param_2.get_bool() {
    //     let shake_data_brake_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("shake_data_brake_scale"));
    //     ShakeModule::req(
    //         fighter.module_accessor,
    //         Hash40::new("brake"),
    //         10000,
    //         false,
    //         &Vector2f{x: 0.0, y: 0.0},
    //         shake_data_brake_scale,
    //         0.0,
    //         false,
    //         false
    //     );
    // }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_turnrun_sub
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}