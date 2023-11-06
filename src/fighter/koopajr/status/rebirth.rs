use crate::imports::status_imports::*;

unsafe extern "C" fn koopajr_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_KART) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_KART, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_REBIRTH, koopa_rebirth_main);
}