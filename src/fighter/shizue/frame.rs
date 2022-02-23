use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    super::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
fn shizue_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_FIRE_ROCKET_ANYTIME) {
            ArticleModule::shoot(
                fighter.module_accessor,
                *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
                false
            );
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_CLAYROCKET_IS_READY);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_FIRE_ROCKET_ANYTIME);
        }

        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && shizue_check_rocket_fire(fighter)
        && !MiscModule::is_damage_check(fighter.module_accessor, false)
        && shizue_check_attack_cancel(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_FIRE_ROCKET_ANYTIME);
            ControlModule::clear_command_one(
                fighter.module_accessor,
                *FIGHTER_PAD_COMMAND_CATEGORY1,
                *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
            );
        }
    }
}

pub fn install() {
    install_agent_frames!(
        shizue_frame
    );
}