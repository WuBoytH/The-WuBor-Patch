use super::*;

mod acmd;
mod status;

#[no_mangle]
unsafe extern "C" fn shinku_on_hit_inner(
    vtable: u64,
    weapon: &mut smash::app::Weapon,
    something: u32
) -> u64 {
    if !VarModule::is_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT) {
        if something & (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) as u32 != 0 {
            VarModule::on_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT);
            println!("hi");
            KineticModule::unable_energy(weapon.battle_object.module_accessor, 0);
            WorkModule::set_int(weapon.battle_object.module_accessor, 35, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }
    else {
        if something & *COLLISION_KIND_MASK_REFLECTOR as u32 != 0 {
            VarModule::off_flag(weapon.battle_object.module_accessor, vars::ryu_shinkuhadoken::status::instance::HIT);
            KineticModule::enable_energy(weapon.battle_object.module_accessor, 0);
        }
    }
    // else {
    //     KineticModule::resume_energy(weapon.battle_object.module_accessor, 0);
    // }
    *(weapon as *mut smash::app::Weapon as *mut bool).add(0x90) = true;
    MiscModule::normal_weapon_hit_handler(vtable, weapon, something)
}

pub fn install() {
    let agent = &mut Agent::new("ryu_shinkuhadoken");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}