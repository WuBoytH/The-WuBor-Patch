use super::*;
use super::status::special_s::*;
use super::status::special_lw::*;

unsafe extern "C" fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }

    let clone = fighter.global_table[WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&true.into());
    fighter.global_table["move_customizer_original"].assign(&clone);
    fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}

unsafe extern "C" fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } else {
        None
    }
}

unsafe extern "C" fn jack_move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if let Some(original) = get_original_customizer(fighter) {
        original(fighter);
    }
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            &mut *(jack_special_s_main as *const () as *mut libc::c_void)
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            &mut *(jack_special_lw_pre as *const () as *mut libc::c_void)
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            &mut *(jack_special_lw_main as *const () as *mut libc::c_void)
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            &mut *(jack_special_lw2_pre as *const () as *mut libc::c_void)
        );
    }
    0.into()
}

unsafe extern "C" fn jack_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    // if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST)
    // && rebel_gauge < 10.0 { // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    //     return 0.into();
    // }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST)
    && rebel_gauge < 1.0 { // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, jack_move_customizer);
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(jack_special_lw_uniq as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
