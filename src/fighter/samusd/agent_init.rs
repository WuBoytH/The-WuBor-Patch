use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::L2CValue
    },
    crate::vars::*
};

pub unsafe extern "C" fn samusd_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cshot_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID);
    if sv_battle_object::is_active(cshot_id as u32) {
        return 0.into();
    }
    1.into()
}
