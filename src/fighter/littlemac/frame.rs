use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if IS_FUNNY[entry_id(fighter.module_accessor)] {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == 2
            && FUNNY_JUMPS[entry_id(fighter.module_accessor)] > 0 {
                WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                FUNNY_JUMPS[entry_id(fighter.module_accessor)] -= 1;
            }
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
                FUNNY_JUMPS[entry_id(fighter.module_accessor)] = 10;
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        littlemac_frame
    );
}