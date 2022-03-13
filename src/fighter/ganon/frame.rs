use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N)
        && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW
        && CatchModule::is_catch(fighter.module_accessor) == false
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ganon_frame
    );
}