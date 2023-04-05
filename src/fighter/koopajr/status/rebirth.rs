use crate::imports::status_imports::*;

#[status_script(agent = "koopajr", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn koopajr_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_KART) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_KART, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        koopajr_rebirth_main
    );
}