use crate::imports::*;

#[repr(C)]
struct SnakeC4Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

unsafe extern "C" fn set_tracker_eff(weapon: &mut L2CWeaponCommon, hash: Hash40, color: &Option<SnakeC4Color>, num: i32) {
    let eff_handle = EffectModule::req_follow(
        weapon.module_accessor,
        hash,
        Hash40::new("top"),
        &Vector3f{x: 0.0, y: 1.0, z: 0.0},
        &ZERO_VECTOR,
        0.7,
        false,
        0,
        0,
        0,
        0,
        0,
        false,
        false
    );
    if let Some(colors) = color {
        EffectModule::set_rgb(weapon.module_accessor, eff_handle as u32, colors.r, colors.g, colors.b);
    }
    VarModule::set_int(weapon.module_accessor, snake_c4::instance::int::TRACKER_EFF + num, eff_handle as i32);
}

unsafe extern "C" fn c4_marker(weapon: &mut L2CWeaponCommon) {
    let status = weapon.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if status == *WEAPON_SNAKE_C4_STATUS_KIND_START
    && !VarModule::is_flag(weapon.module_accessor, snake_c4::instance::flag::TRACKER_ENABLED) {
        VarModule::on_flag(weapon.module_accessor, snake_c4::instance::flag::TRACKER_ENABLED);
        VarModule::on_flag(weapon.module_accessor, snake_c4::instance::flag::TRACKER_VISIBLE);
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let color = if sv_battle_object::is_active(owner_id) {
            let module_accessor = sv_battle_object::module_accessor(owner_id);
            let team = FighterUtil::get_team_color(module_accessor);
            let colors = FighterUtil::get_effect_team_color(EColorKind(team as i32), Hash40::new("shield_effect_color"));
            Some(SnakeC4Color{r: colors.x, g: colors.y, b: colors.z})
        }
        else {
            None
        };
        set_tracker_eff(weapon, Hash40::new("snake_final_lockon"), &color, 0);
        set_tracker_eff(weapon, Hash40::new("snake_final_lockon2"), &color, 1);
        set_tracker_eff(weapon, Hash40::new("snake_final_lockon_ready"), &color, 2);
        set_tracker_eff(weapon, Hash40::new("snake_final_lockon_ready2"), &color, 3);
        SoundModule::play_se(
            weapon.module_accessor,
            Hash40::new("se_snake_final02"),
            true,
            false,
            false,
            false,
            enSEType(0)
        );
    }
}

unsafe extern "C" fn tracker_toggle(weapon: &mut L2CWeaponCommon) {
    let visible = !VarModule::is_flag(weapon.module_accessor, snake_c4::instance::flag::TRACKER_VISIBLE);
    for num in 0..2 {
        let eff_handle = VarModule::get_int(weapon.module_accessor, snake_c4::instance::int::TRACKER_EFF + num) as u32;
        EffectModule::set_visible(weapon.module_accessor, eff_handle, visible);
    }
    if !visible {
        SoundModule::play_se(
            weapon.module_accessor,
            Hash40::new("se_snake_final02"),
            true,
            false,
            false,
            false,
            enSEType(0)
        );
    }
    VarModule::set_flag(weapon.module_accessor, snake_c4::instance::flag::TRACKER_VISIBLE, visible);
}

unsafe extern "C" fn tracker_timer_flash(weapon: &mut L2CWeaponCommon) {
    let status = weapon.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if status != *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION {
        let life = WorkModule::get_int(weapon.module_accessor, 0x1000000A); //WEAPON_SNAKE_C4_INSTANCE_WORK_ID_INT_LIFE_COUNT
        if life <= 180 {
            if life % 30 == 0 {
                tracker_toggle(weapon);
            }
        }
    }
}

unsafe extern "C" fn on_main(weapon: &mut L2CWeaponCommon) {
    c4_marker(weapon);
    tracker_timer_flash(weapon);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, on_main);
}