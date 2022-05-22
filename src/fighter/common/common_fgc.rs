use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    wubor_utils::{
        wua_bind::*,
        vars::*,
    }
};

pub unsafe fn common_fgc(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        if !MiscModule::is_damage_check(fighter.module_accessor, false) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        }
    }
    if fighter.global_table["fgc_func"].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) = std::mem::transmute(fighter.global_table["fgc_func"].get_ptr());
        callable(fighter);
    }
}
