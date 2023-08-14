use crate::imports::status_imports::*;

#[status("szerosuit", FIGHTER_STATUS_KIND_REBIRTH)]
unsafe fn szerosuit_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    szerosuit_rebirth_main::install();
}