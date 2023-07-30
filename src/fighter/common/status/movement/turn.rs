use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnCommon)]
unsafe fn status_pre_turncommon(fighter: &mut L2CFighterCommon) {
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
    if [
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_DASH
    ].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32())
    && VarModule::is_flag(fighter.battle_object, dash::flag::DISABLE_PIVOT_TURN_DASH) {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
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

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_turncommon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}