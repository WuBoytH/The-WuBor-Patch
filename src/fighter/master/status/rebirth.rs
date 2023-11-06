use crate::imports::status_imports::*;

#[status_script(agent = "master", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn master_rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_REBIRTH, master_rebirth_end);
}