use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[inline(always)]
pub unsafe fn luigi_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    // let mut allowed_cancels : Vec<i32> = [].to_vec();
    // set_hp(fighter, 105.0);
    if ![*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END].contains(&status) {
        SPECIAL_HITSTUN[entry_id(fighter.module_accessor)] = false;
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // What allows Luigi to cancel Super Jump Punch if he lands the sweetspot.

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_drop")
        && (CANCEL[entry_id(fighter.module_accessor)]
        || IS_FUNNY[entry_id(fighter.module_accessor)]) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }

        // Kills any residual effects from Thunderhand if Luigi is no longer in the move.

        if (StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_S
        || StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE
        || StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END)
        && (StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END) {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_thunder"), false, true);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("luigi_rocket_hold"), false, true);
        }

        if IS_FGC[entry_id(fighter.module_accessor)] {
            luigi_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        luigi_frame
    );
}