use crate::imports::*;

unsafe extern "C" fn gamewatch_special_hi_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_air_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_WORK_INT_ATTACK_AIR_KIND);
    ControlModule::set_attack_air_kind(fighter.module_accessor, attack_air_kind);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    // New
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);

    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE, false, -1);
    }
    ArticleModule::change_motion(
        fighter.module_accessor,
        *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_PARACHUTE,
        Hash40::new("special_hi_open"),
        false,
        -1.0
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi_open"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_special_hi_fall_main_loop as *const () as _))
}

unsafe extern "C" fn gamewatch_special_hi_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_special().get_bool()
    || fighter.sub_transition_group_check_air_attack().get_bool()
    || fighter.sub_transition_group_check_air_escape().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE.into(), false.into());
        return 1.into();
    }
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let prev_stick_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
    if (stick_y <= squat_stick_y && squat_stick_y < prev_stick_y)
    || (jump_stick_y <= stick_y && prev_stick_y < jump_stick_y) {
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL, gamewatch_special_hi_fall_main);
}