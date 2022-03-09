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
        let status = StatusModule::status_kind(fighter.module_accessor);
        if !MiscModule::is_damage_check(fighter.module_accessor, false) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL) != 1.2 {
                WorkModule::set_float(fighter.module_accessor, 1.2, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL);
            }
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        }
        if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let frame : f32;
            if !MiscModule::is_damage_check(fighter.module_accessor, true) {
                frame = 7.0;
            }
            else {
                frame = 30.0;
            }
            if MotionModule::frame(fighter.module_accessor) >= frame {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
    if fighter.global_table["fgc_func"].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) = std::mem::transmute(fighter.global_table["fgc_func"].get_ptr());
        callable(fighter);
    }
}
