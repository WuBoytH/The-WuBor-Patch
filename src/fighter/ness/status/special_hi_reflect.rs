use crate::imports::status_imports::*;

#[status("ness", FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT)]
unsafe fn ness_special_hi_reflect_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);
    ret
}

pub fn install() {
    ness_special_hi_reflect_main::install();
}