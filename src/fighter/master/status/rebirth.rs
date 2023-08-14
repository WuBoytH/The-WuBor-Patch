use crate::imports::status_imports::*;

#[status("master", FIGHTER_STATUS_KIND_REBIRTH)]
unsafe fn master_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    master_rebirth_main::install();
}