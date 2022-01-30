use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::vars::*
};

pub unsafe extern "C" fn samusd_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cshot_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID);
    if sv_battle_object::is_active(cshot_id as u32) {
        // return 0.into();
        let cshot_boma = sv_battle_object::module_accessor(cshot_id as u32);
        WorkModule::set_int(cshot_boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    1.into()
}
