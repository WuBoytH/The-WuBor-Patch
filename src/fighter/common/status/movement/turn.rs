use crate::imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnCommon)]
unsafe extern "C" fn status_pre_turncommon(fighter: &mut L2CFighterCommon) {
    // Vanilla
    let groups = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND
    ];
    for x in groups.iter() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *x);
    }
    let group_ex = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
    ];
    for x in group_ex.iter() {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *x);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("turn"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_turn_uniq_process_main)]
unsafe extern "C" fn sub_turn_uniq_process_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_REVERSE);
    let turn_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_WORK_FLOAT_TURN_FRAME);
    if turn_frame <= 0.0 {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_TURN) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_TURN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_REVERSE);
        }
    }
    else {
        WorkModule::sub_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_TURN_WORK_FLOAT_TURN_FRAME);
    }
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_TURN_DASH {
        let dash_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
        let mut dash_flick_x = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_flick_x"));
        // Additional buffer if coming from dash or turn dash.
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
            dash_flick_x += 2;
        }
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let turn_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_WORK_FLOAT_LR);
        let flick_x = ControlModule::get_flick_x(fighter.module_accessor);
        if dash_stick_x < stick_x * -turn_lr
        && flick_x & 0xff < dash_flick_x {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_DASH);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_turncommon,
            sub_turn_uniq_process_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}