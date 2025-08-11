#[allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    smash_script::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{app::*, cancels::*, wua_bind::*, vars, table_const::*}
};

mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod vl;

// unsafe fn kirby_copy_motion_patcher(mut motions: Vec<u64>, offset: usize) {
//     let ptr = motions.as_mut_ptr();
//     let len = motions.len();
//     let end_ptr = unsafe { ptr.add(len) };

//     println!("start: {:#?}", ptr);
//     println!("end:   {:#?}", end_ptr);

//     let _ = skyline::patching::Patch::in_text(offset).data(ptr);
//     let _ = skyline::patching::Patch::in_text(offset + 0x8).data(end_ptr);
//     let _ = skyline::patching::Patch::in_text(offset + 0x10).data(end_ptr);
// }

pub fn install() {
    let agent = &mut Agent::new("kirby");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    // let ryu_copy_motions = [
    //     hash40("ryu_special_n"), hash40("special_n"),
    //     hash40("ryu_special_air_n"), hash40("special_air_n"),
    //     hash40("ryu_special_n2"), hash40("special_n2"),
    //     hash40("ryu_special_air_n2"), hash40("special_air_n2")
    // ].to_vec();
    // unsafe { kirby_copy_motion_patcher(ryu_copy_motions, 0x52c0950) };
}