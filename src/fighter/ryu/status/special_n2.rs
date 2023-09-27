use crate::imports::status_imports::*;

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_N2, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn ryu_special_n2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);

    0.into()
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_N2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    0.into()
}

unsafe fn ryu_special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_special_n2_main
    );
}