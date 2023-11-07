use crate::imports::status_imports::*;

unsafe extern "C" fn pitb_specialn_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    pitb_specialn_endremove(fighter)
}

unsafe extern "C" fn pitb_specialn_endremove(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, pitb_specialn_shoot_end);
}