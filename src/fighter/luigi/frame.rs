use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::vars::*
};

#[inline(always)]
pub unsafe fn luigi_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    // let mut allowed_cancels : Vec<i32> = [].to_vec();
    // set_hp(fighter, 105.0);
    if ![*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END].contains(&status) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HITSTUN);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // What allows Luigi to cancel Super Jump Punch if he lands the sweetspot.

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_drop")
        && (WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL)
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY)) {
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

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
            luigi_fgc(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        luigi_frame
    );
}