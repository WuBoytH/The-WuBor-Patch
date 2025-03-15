use super::*;

// pub static mut EFFECT_COMMON_CONSTRUCTOR_INLINE : usize = 0x3366a0;

#[skyline::hook(replace = L2CFighterAnimcmdEffectCommon_L2CFighterAnimcmdEffectCommon)]
unsafe extern "C" fn effect_common_constructor(
    fighter: &mut smash_rs::lua2cpp::L2CFighterAnimcmdEffectCommon,
    battle_object: &mut BattleObject,
    module_accessor: &mut BattleObjectModuleAccessor,
    lua_state: &mut smash::lua_State
) {
    original!()(fighter, battle_object, module_accessor, lua_state);
    fighter.sv_set_function_hash(std::mem::transmute(bind_hash_call_effect_GuardCancel as *const ()), smash_rs::phx::Hash40::new("effect_guardcancel"));
    fighter.sv_set_function_hash(std::mem::transmute(bind_hash_call_effect_GuardCancel as *const ()), smash_rs::phx::Hash40::new("effect_burnout"));
}

extern "C" {
    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon31bind_hash_call_effect_ChargeMaxEPN3lib8L2CAgentERNS1_7utility8VariadicEPKcSt9__va_list"]
    fn bind_hash_call_effect_ChargeMax();
}

#[skyline::hook(replace = bind_hash_call_effect_ChargeMax)]
unsafe extern "C" fn effect_chargemax() {
    // nothing
}

#[allow(non_snake_case)]
unsafe extern "C" fn bind_hash_call_effect_GuardCancel(
    fighter: &mut L2CFighterAnimcmdEffectCommon,
    _agent: &mut L2CAgent,
    variadic: &mut smash::lib::utility::Variadic,
    _string: *const u8,
    _va_list: u32,
) {
    println!("what's up");
    effect_GuardCancel(fighter);
    let cstr = variadic.get_format();
    if !cstr.is_null() {
        let mut ret = effect_end(fighter);
        ret.push_variadic(0, cstr, variadic);
    }
    else {
        effect_end(fighter);
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn effect_GuardCancel(fighter: &mut L2CFighterAnimcmdEffectCommon) {
    println!("we are in the beam");
    let agent = &mut fighter.agent;
    loop {
        agent.clear_lua_stack();
        if is_excute(agent.lua_state_agent) {
            agent.clear_lua_stack();
            lua_args!(agent, 1, 0, 0, 0.5);
            FLASH_NO_STOP(agent.lua_state_agent);

            agent.clear_lua_stack();
            lua_args!(agent, 4, 1, 0, 0, 0.25);
            FLASH_FRM(agent.lua_state_agent);
        }
        agent.clear_lua_stack();
        wait(agent.lua_state_agent, 4.0);
        agent.clear_lua_stack();
        if is_excute(agent.lua_state_agent) {
            agent.clear_lua_stack();
            COL_NORMAL(agent.lua_state_agent);
        }
        agent.clear_lua_stack();
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn effect_end(_fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        // unsafe {
        //     let base = (*info.module.ModuleObject).module_base as usize;
        //     EFFECT_COMMON_CONSTRUCTOR_INLINE += base;

        //     skyline::hooks::A64InlineHook(
        //         EFFECT_COMMON_CONSTRUCTOR_INLINE as u64 as _,
        //         effect_common_constructor_inline as _
        //     );
        // }
        skyline::install_hooks!(
            effect_common_constructor,
            effect_chargemax
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}