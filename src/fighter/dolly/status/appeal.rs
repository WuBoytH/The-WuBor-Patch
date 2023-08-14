use crate::imports::status_imports::*;

#[status("dolly", FIGHTER_STATUS_KIND_APPEAL)]
unsafe fn dolly_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    fighter.status_end_Appeal();
    0.into()
}

pub fn install() {
    dolly_appeal_end::install();
}