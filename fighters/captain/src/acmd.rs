use super::*;

#[no_mangle]
pub unsafe extern "C" fn spend_boost_power(agent: &mut L2CAgentBase) {
    if VarModule::get_float(agent.module_accessor, vars::captain::instance::float::BOOST_POWER) >= vars::captain::BOOST_POWER_THRESHOLD {
        if macros::is_excute(agent) {
            VarModule::add_float(agent.module_accessor, vars::captain::instance::float::BOOST_POWER_ADD, -vars::captain::BOOST_POWER_THRESHOLD);
            VarModule::on_flag(agent.module_accessor, vars::captain::status::flag::USED_BOOST_POWER);
        }
    }
}

mod dash;

mod guard;

mod normals;
mod aerials;
mod specials;

mod catch;
mod throws;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);

    catch::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);
}