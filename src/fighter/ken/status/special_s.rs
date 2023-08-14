use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

#[status("ken", FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn ken_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_init_main(fighter)
}

#[status("ken", FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn ken_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

#[status("ken", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)]
unsafe fn ken_specials_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_specials_init_main(fighter)
}

#[status("ken", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)]
unsafe fn ken_specials_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

pub fn install() {
    ken_specials_init::install();
    ken_specials_main::install();
    ken_specials_command_init::install();
    ken_specials_command_main::install();
}