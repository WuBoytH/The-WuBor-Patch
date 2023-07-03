use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_cancel::*,
};

unsafe extern "C" fn jack_cancel_post(fighter: &mut L2CFighterCommon) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, FighterEntryID(entry_id), -10.0);
    false
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_jack");
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_specials([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
                ].to_vec())
                .post_function(jack_cancel_post)
        );
    }
}
