use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    super::super::helper::*
};

#[status_script(agent = "murabito", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn murabito_special_n_pocket_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_pocket_set_flag(fighter)
}

pub fn install() {
    install_status_scripts!(
        murabito_special_n_pocket_end
    );
}