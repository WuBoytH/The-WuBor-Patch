use crate::imports::status_imports::*;

#[status("lucas", FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT)]
unsafe fn lucas_special_hi_reflect_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);
    ret
}

pub fn install() {
    lucas_special_hi_reflect_main::install();
}