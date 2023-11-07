use crate::imports::status_imports::*;

unsafe extern "C" fn mario_rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    EffectModule::remove_post_effect_line(fighter.module_accessor, 0x1e, true);
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_REBIRTH, mario_rebirth_end);
}