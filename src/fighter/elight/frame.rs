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

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT, main )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_jump") {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            if stick_x * lr < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_line(smashline::Main, elight_frame);
}