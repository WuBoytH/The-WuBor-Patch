use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::L2CValue
    },
    crate::fighter::belmont::agent_init::*,
    wubor_utils::table_const::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(belmont_guard_cont_pre as *const () as _));
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}
