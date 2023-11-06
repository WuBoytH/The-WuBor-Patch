use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    crate::fighter::daisy::agent_init::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_PEACH {
            return;
        }
        fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}