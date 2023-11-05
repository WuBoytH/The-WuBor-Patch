use crate::imports::status_imports::*;

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn ness_special_hi_reflect_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS);
    ret
}

pub fn install() {
    install_status_scripts!(
        ness_special_hi_reflect_main
    );
}