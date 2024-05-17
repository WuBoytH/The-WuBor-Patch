mod normals;
mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install(acmd: &mut smashline::Agent) {
    normals::install(acmd);
    aerials::install(acmd);
    specials::install(acmd);
    escape::install(acmd);
    cliff::install(acmd);
}