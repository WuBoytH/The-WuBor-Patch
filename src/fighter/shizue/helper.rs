use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    }
};

pub unsafe extern "C" fn shizue_check_rocket_fire(fighter: &mut L2CFighterCommon) -> bool {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_CLAYROCKET_IS_READY) {
        if !ArticleModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET,
            0x20000008
        ) {
            if ![
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_SET,
                *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FAILURE,
                *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE
            ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
                return true;
            }
        }
    }
    false
}

pub unsafe extern "C" fn shizue_check_attack_cancel(fighter: &mut L2CFighterCommon) -> bool {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if !(*FIGHTER_STATUS_KIND_WAIT..=*FIGHTER_STATUS_KIND_TURN_RUN_BRAKE).contains(&status)
    && !(*FIGHTER_STATUS_KIND_JUMP..=*FIGHTER_STATUS_KIND_FALL_AERIAL).contains(&status)
    && !(*FIGHTER_STATUS_KIND_SQUAT..=*FIGHTER_STATUS_KIND_SQUAT_RV).contains(&status)
    && !(*FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT..=*FIGHTER_STATUS_KIND_REBOUND_JUMP).contains(&status) {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            return true;
        }
    }
    false
}
