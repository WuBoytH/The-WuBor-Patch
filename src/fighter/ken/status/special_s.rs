use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

unsafe extern "C" fn ken_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_init_main(fighter)
}

unsafe extern "C" fn ken_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

unsafe extern "C" fn ken_specials_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_specials_init_main(fighter)
}

unsafe extern "C" fn ken_specials_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ken_specials_init,
        ken_specials,
        ken_specials_command_init,
        ken_specials_command
    );
}