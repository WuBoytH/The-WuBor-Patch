mod normals;
mod specials;
mod lasso;
mod escape;

pub fn install(agent : &mut smashline::Agent) {
    normals::install(agent);
    specials::install(agent);
    lasso::install(agent);
    escape::install(agent);
}