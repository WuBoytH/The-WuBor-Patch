use crate::imports::status_imports::*;

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucas_special_hi_reflect_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);
    ret
}

pub fn install() {
    install_status_scripts!(
        lucas_special_hi_reflect_main
    );
}