use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::table_const::*
};

unsafe extern "C" fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("appeal_lw_l"),
            hash40("appeal_lw_r")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor))
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IS_STOP].get_bool() {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_line(smashline::Main, gekkouga_frame);
}