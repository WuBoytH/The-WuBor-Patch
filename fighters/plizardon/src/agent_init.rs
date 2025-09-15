use super::*;

unsafe extern "C" fn plizardon_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT) {
        return 0.into();
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POKEMON_INSTANCE_WORK_ID_FLAG_SEND_POKEMON_CHANGE);

    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(plizardon_special_lw_uniq as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}