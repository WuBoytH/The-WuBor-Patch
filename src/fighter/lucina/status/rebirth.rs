use crate::imports::status_imports::*;

#[status("lucina", FIGHTER_STATUS_KIND_REBIRTH)]
unsafe fn lucina_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    lucina_rebirth_main::install();
}