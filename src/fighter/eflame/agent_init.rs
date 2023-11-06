use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_EFLAME {
            return;
        }
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[FALL_BRAKE_UNIQ].assign(&L2CValue::Bool(false));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_start(on_start);
}
