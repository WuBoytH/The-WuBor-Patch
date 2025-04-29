#![allow(non_snake_case)]

use super::*;

#[repr(C)]
pub struct FighterKineticEnergyDamage {
    parent: KineticEnergy,
    padding: u64,
    damage_target_speed: PaddedVec2,
    reset_type: EnergyStopResetType,
    elapsed_hitstop_frames: f32,
    hitstop_frames: f32,
    _xAC: f32,
    _xB0: f32,
    should_sync_damage_speed: bool,
    needs_to_sync_damage_speed: bool,
    should_start_interpolation: bool,
    interpolation_frames_remaining: u8,
    _xB8: u8,
    is_target_pos: bool,
    _xBA: bool,
    _xBB: bool,
    _xBC: u32,
    _xC0: PaddedVec2
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyStopResetType {
    Ground = 0x0,
    DamageGround,
    DamageAir,
    DamageAirIce,
    DamageOther,
    DamageKnockBack,
    GlideLanding,
    Air,
    AirXNormalMax,
    AirEscape,
    AirBrake,
    AirBrakeAlways,
    GuardDamage,
    Capture,
    CatchCut,
    ItemSwingDash,
    ItemDashThrow,
    SwimBrake,
    Run,
    RunBrake,
    GlideStart,
    CatchDash,
    ShieldRebound,
    Free,
    CaptureBeetle,
    AirLassoHang,
    AirLassoRewind,
    EscapeAirSlide,
    DamageGroundOrbit,
    DamageAirOrbit
}

#[skyline::hook(offset = 0x6d8100)]
unsafe extern "C" fn damage_initialize(energy: &mut FighterKineticEnergyDamage, module_accessor: &mut BattleObjectModuleAccessor) {
    use EnergyStopResetType::*;
    original!()(energy, module_accessor);

    if [
        DamageGround,
        GuardDamage,
        DamageGroundOrbit
    ].contains(&energy.reset_type) {
        energy.parent.speed_brake.x = 0.06;
    }
}

pub fn install() {
    skyline::install_hooks!(
        damage_initialize
    );
}