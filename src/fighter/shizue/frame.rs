use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame,
    super::helper::*,
};

unsafe extern "C" fn shizue_clayrocket_fire_handler(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, shizue::instance::flag::FIRE_ROCKET_ANYTIME) {
        ArticleModule::shoot(
            fighter.module_accessor,
            *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
            false
        );
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_CLAYROCKET_IS_READY);
        VarModule::off_flag(fighter.module_accessor, shizue::instance::flag::FIRE_ROCKET_ANYTIME);
    }

    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
    && shizue_check_rocket_fire(fighter)
    && !MiscModule::is_damage_check(fighter.module_accessor, false)
    && shizue_check_attack_cancel(fighter) {
        VarModule::on_flag(fighter.module_accessor, shizue::instance::flag::FIRE_ROCKET_ANYTIME);
        ControlModule::clear_command_one(
            fighter.module_accessor,
            *FIGHTER_PAD_COMMAND_CATEGORY1,
            *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
        );
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    shizue_clayrocket_fire_handler(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, on_main);
}