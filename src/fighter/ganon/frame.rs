use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N)
        && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ganon_frame
    );
}