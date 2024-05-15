use crate::imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_Pass_common)]
unsafe extern "C" fn status_pass_common(fighter: &mut L2CFighterCommon) {
    fighter.sub_air_check_fall_common_pre();
    let transitions = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON
    ];
    for val in transitions.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("pass"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_Pass_Main_sub)]
unsafe extern "C" fn status_pass_main_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let pass_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_PASS_WORK_INT_FRAME);
    if pass_frame > 0 {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
        || fighter.sub_transition_group_check_ground_jump().get_bool() {
            return 1.into();
        }
    }
    original!()(fighter, param_1)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pass_common,
            status_pass_main_sub
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}