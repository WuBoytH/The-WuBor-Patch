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
    let add_gauge = WorkModule::is_flag(fighter.module_accessor, 0x200000E9); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE
    let arsene_exist = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
    WorkModule::on_flag(fighter.module_accessor, 0x200000E9);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
    let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    let cancel_cost = if rebel_gauge - 20.0 <= 0.0 {
        rebel_gauge - 0.1
    }
    else {
        20.0
    };
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, FighterEntryID(entry_id), -cancel_cost);
    WorkModule::set_flag(fighter.module_accessor, add_gauge, 0x200000E9);
    WorkModule::set_flag(fighter.module_accessor, arsene_exist, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
    false
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_jack");
    CustomCancelManager::initialize_agent(agent);
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
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
