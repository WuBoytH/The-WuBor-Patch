use crate::imports::*;

#[skyline::hook(offset = 0x33e1a70)]
unsafe extern "C" fn wave_init(vtable: u64, weapon: *mut app::Weapon, something: u64, something_2: f32) {
    original!()(vtable, weapon, something, something_2);
    let module_accessor = (*weapon).battle_object.module_accessor;
    if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR)
    && WorkModule::get_int(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
        *(weapon as *mut bool).add(0x90) = true;
    }
}

#[skyline::hook(offset = 0x33e1fa4, inline)]
unsafe extern "C" fn wave_on_hit(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[24].x.as_ref() as *mut BattleObjectModuleAccessor;
    if WorkModule::get_int(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
        let effect = if VarModule::is_flag(module_accessor, dolly_wave::status::flag::FINAL_HIT) {
            Hash40::new("effect_hitstrong")
        }
        else {
            Hash40::new("effect_hitstrong_last")
        };
        MotionAnimcmdModule::call_script_single(
            module_accessor,
            *WEAPON_ANIMCMD_EFFECT,
            effect,
            -1
        );
        *ctx.registers[19].w.as_mut() = 0;
    }
    else {
        StatusModule::change_status_request(module_accessor, 1, false);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x33e1fa4).nop();
    skyline::install_hooks!(
        wave_init,
        wave_on_hit
    );
}