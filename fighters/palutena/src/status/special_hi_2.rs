use super::*;

unsafe extern "C" fn palutena_special_hi_2_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    VarModule::set_float(fighter.module_accessor, vars::palutena::status::float::SPECIAL_HI_2_SPEED_X, speed_x);
    VarModule::set_float(fighter.module_accessor, vars::palutena::status::float::SPECIAL_HI_2_SPEED_Y, speed_y);
    0.into()
}

unsafe extern "C" fn palutena_special_hi_2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = VarModule::get_float(fighter.module_accessor, vars::palutena::status::float::SPECIAL_HI_2_SPEED_X);
    let speed_y = VarModule::get_float(fighter.module_accessor, vars::palutena::status::float::SPECIAL_HI_2_SPEED_Y);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        speed_y
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, palutena_special_hi_2_init);
    agent.status(Exec, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, palutena_special_hi_2_exec);
}