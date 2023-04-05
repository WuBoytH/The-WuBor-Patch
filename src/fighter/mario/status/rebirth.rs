use crate::imports::status_imports::*;

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    EffectModule::remove_post_effect_line(fighter.module_accessor, 0x1e, true);
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mario_rebirth_main
    );
}