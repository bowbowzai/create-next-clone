use crate::Selection;

pub mod default;

pub struct Configuration<'a> {
    pub project_name: &'a String,
    pub is_typescript: Selection,
    pub is_eslint: Selection,
    pub is_tailwind: Selection,
    pub is_src: Selection,
    pub is_app_router: Selection,
    pub is_customize_alias: Selection,
}

pub fn generate_project_structure(configuration: &Configuration) {
    match configuration {
        Configuration {
            is_typescript: Selection::Yes,
            is_eslint: Selection::Yes,
            is_tailwind: Selection::Yes,
            is_src: Selection::No,
            is_app_router: Selection::Yes,
            is_customize_alias: Selection::No,
            ..
        } => {
            // this is default project structure
            default::generate_project_structure(&configuration);
        }
        _ => (),
    }
}
