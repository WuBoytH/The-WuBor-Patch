use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S)
        && (StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
        || MiscModule::is_damage_check(fighter.module_accessor, false)) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        daisy_frame
    );
}