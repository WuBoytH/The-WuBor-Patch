use crate::imports::status_imports::*;

#[status("gekkouga", FIGHTER_STATUS_KIND_SPECIAL_HI)]
unsafe fn gekkouga_special_hi_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status("gekkouga", FIGHTER_STATUS_KIND_SPECIAL_HI)]
unsafe fn gekkouga_special_hi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    gekkouga_special_hi_init::install();
    gekkouga_special_hi_exec::install();
}