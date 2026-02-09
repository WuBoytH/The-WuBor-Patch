use crate::imports::*;

#[skyline::hook(offset = 0x33e1800)]
unsafe extern "C" fn wave_init(vtable: u64, weapon: *mut app::Weapon, something: u64, something_2: f32) {
    original!()(vtable, weapon, something, something_2);
    let module_accessor = (*weapon).battle_object.module_accessor;
    if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR)
    && WorkModule::get_int(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
        *(weapon as *mut bool).add(0x90) = true;
    }
}

#[skyline::hook(offset = 0x33e1d34, inline)]
unsafe extern "C" fn wave_on_hit(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[24].x() as *mut BattleObjectModuleAccessor;
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
        ctx.registers[19].set_w(0);
    }
    else {
        StatusModule::change_status_request(module_accessor, 1, false);
    }
}

#[skyline::from_offset(0x33bcd10)]
unsafe extern "C" fn weapon_hit_handler2(
    vtable: u64,
    weapon: &mut smash::app::Weapon,
    param_3: u64,
    log: u64
);

extern "C" {
    #[link_name = "add_go"]
    fn add_go(module_accessor: *mut BattleObjectModuleAccessor, amount: f32);
}

unsafe extern "C" fn dolly_wave_on_hit2(
    vtable: u64,
    weapon: &mut smash::app::Weapon,
    param_3: u64,
    log: u64,
    damage: f32
) {
    println!("hi");
    weapon_hit_handler2(vtable, weapon, param_3, log);
    let module_accessor = weapon.battle_object.module_accessor;
    let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let battle_object = MiscModule::get_battle_object_from_id(owner_id);
    if !battle_object.is_null() {
        if (*battle_object).kind == *FIGHTER_KIND_DOLLY as u32
        && TeamModule::team_owner_id(module_accessor) as u32 == owner_id {
            let owner_module_accessor = (*battle_object).module_accessor;
            let collision_log = log as *mut CollisionLogScuffed;
            let collision_kind = (*collision_log).collision_kind;
            if [1, 2].contains(&collision_kind) {
                let mul = if collision_kind == 2 {
                    0.1
                }
                else {
                    1.0
                };
                // println!("meter to add: {}", damage * mul);
                add_go(owner_module_accessor, damage * mul);
            }
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x33e1d34).nop();
    let _ = skyline::patching::Patch::in_text(0x51bbd10).data(dolly_wave_on_hit2 as *const () as u64);
    skyline::install_hooks!(
        wave_init,
        wave_on_hit
    );
}