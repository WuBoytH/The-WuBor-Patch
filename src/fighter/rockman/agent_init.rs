use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::table_const::*
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_ROCKMAN {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&false.into());
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&false.into());
        fighter.global_table[DASH_COMMON_UNIQ].assign(&false.into());
        fighter.global_table[RUN_MAIN_UNIQ].assign(&false.into());
        fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].assign(&false.into());
        fighter.global_table[GUARD_CONT_UNIQ].assign(&false.into());
        fighter.global_table[TURN_UNIQ].assign(&false.into());
        fighter.global_table[FALL_BRAKE_UNIQ].assign(&false.into());
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
