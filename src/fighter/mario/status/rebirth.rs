use crate::imports::status_imports::*;

#[status("mario", FIGHTER_STATUS_KIND_REBIRTH)]
unsafe fn mario_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    EffectModule::remove_post_effect_line(fighter.module_accessor, 0x1e, true);
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install() {
    mario_rebirth_main::install();
}