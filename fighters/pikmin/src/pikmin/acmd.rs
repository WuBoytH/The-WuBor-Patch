use super::*;

#[repr(C)]
pub struct PikminGameParams {
    pub damage: f32,
    pub hitbox_size: f32,
    pub hit_effect: Hash40,
    pub sound_attr: i32
}

pub unsafe extern "C" fn pikmin_game_helper(agent: &mut L2CAgentBase, base_damage: f32, base_size: f32) -> PikminGameParams {
    let variation = WorkModule::get_int(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    match variation {
        // Red
        0 => PikminGameParams{
            damage: base_damage * 1.4,
            hitbox_size: base_size,
            hit_effect: Hash40::new("collision_attr_fire"),
            sound_attr: *COLLISION_SOUND_ATTR_FIRE
        },
        // Yellow
        1 => PikminGameParams{
            damage: base_damage,
            hitbox_size: base_size * 1.5,
            hit_effect: Hash40::new("collision_attr_elec"),
            sound_attr: *COLLISION_SOUND_ATTR_ELEC
        },
        // Blue
        2 => PikminGameParams{
            damage: base_damage,
            hitbox_size: base_size,
            hit_effect: Hash40::new("collision_attr_normal"),
            sound_attr: *COLLISION_SOUND_ATTR_PUNCH
        },
        // White
        3 => PikminGameParams{
            damage: base_damage * 0.8,
            hitbox_size: base_size,
            hit_effect: Hash40::new("collision_attr_purple"),
            sound_attr: *COLLISION_SOUND_ATTR_PUNCH
        },
        // Purple
        4 => PikminGameParams{
            damage: base_damage * 1.6,
            hitbox_size: base_size,
            hit_effect: Hash40::new("collision_attr_normal"),
            sound_attr: *COLLISION_SOUND_ATTR_KICK
        },
        _ => unreachable!()
    }
}

#[repr(C)]
pub struct PikminEffectParams {
    pub trail: Option<PikminEffectTrailColor>,
    pub flash: f32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PikminEffectTrailColor {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn pikmin_effect_helper(agent: &mut L2CAgentBase) -> PikminEffectParams {
    let variation = WorkModule::get_int(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    match variation {
        // Red
        0 => PikminEffectParams{
            trail: Some(PikminEffectTrailColor{r: 1.0, g: 0.05, b: 0.0}),
            flash: 0.3
        },
        // Yellow
        1 => PikminEffectParams{
            trail: Some(PikminEffectTrailColor{r: 1.0, g: 1.0, b: 0.14}),
            flash: 0.45
        },
        // Blue
        2 => PikminEffectParams{
            trail: Some(PikminEffectTrailColor{r: 0.1, g: 0.3, b: 1.0}),
            flash: 0.3
        },
        // White
        3 => PikminEffectParams{
            trail: None,
            flash: 0.45
        },
        // Purple
        4 => PikminEffectParams{
            trail: Some(PikminEffectTrailColor{r: 0.36, g: 0.0, b: 1.0}),
            flash: 0.3
        },
        _ => unreachable!()
    }
}

pub fn pikmin_acmd(agent: &mut Agent, name: &str, function: unsafe extern "C" fn(&mut L2CAgentBase)) {
    agent.acmd(name, function, Priority::Low);

    let pikmin = ["_y", "_b", "_w", "_v"];

    for color in pikmin {
        let mut new_name = name.to_owned();
        new_name.push_str(color);
        agent.acmd(new_name.as_str(), function, Priority::Low);
    }
}

mod guard;

mod catch;

pub fn install(agent: &mut Agent) {
    guard::install(agent);

    catch::install(agent);
}