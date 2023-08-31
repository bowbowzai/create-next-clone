use std::error::Error;
use std::fs::{ self as fs, create_dir_all, File };
use std::io::Write;
use super::Configuration;

pub type MyResult = Result<(), Box<dyn Error>>;

pub fn generate_project_structure(configuration: &Configuration) -> MyResult {
    let folder_name = format!("./{}", configuration.project_name.trim()).to_string();
    let folder_name = folder_name.as_str();

    // create a folder with user input project name
    if let Err(error) = create_dir_all(folder_name) {
        eprint!("Create project folder failed: {}", error);
    }

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ package.json setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let package_json_path = format!("{}/package.json", folder_name).to_string();
    let package_json_template: String = std::fs
        ::read_to_string("src/configs/templates/package.json.template")
        .expect("Unable to read package.json.template");
    let package_json_content = package_json_template.replace(
        "{{project_name}}",
        configuration.project_name.trim()
    );
    create_and_write_file(&package_json_path, &package_json_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ README.md setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let readme_path = format!("{}/README.md", folder_name).to_string();
    let readme_content: String = std::fs
        ::read_to_string("src/configs/templates/readme.md.template")
        .expect("Unable to read readme.md.template");
    create_and_write_file(&readme_path, &readme_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ .eslintrc.json setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let eslintrc_path = format!("{}/.eslintrc.json", folder_name).to_string();
    let eslintrc_content: String = std::fs
        ::read_to_string("src/configs/templates/.eslintrc.json.template")
        .expect("Unable to read .eslintrc.json.template");
    create_and_write_file(&eslintrc_path, &eslintrc_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ .gitignore setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let gitignore_path = format!("{}/.gitignore", folder_name).to_string();
    let gitignore_content: String = std::fs
        ::read_to_string("src/configs/templates/.gitignore.template")
        .expect("Unable to read .gitignore.template");
    create_and_write_file(&gitignore_path, &gitignore_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ next-env.d.ts setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let next_env_path = format!("{}/next-env.d.ts", folder_name).to_string();
    let next_env_content: String = std::fs
        ::read_to_string("src/configs/templates/next-env.d.ts.template")
        .expect("Unable to read next-env.d.ts.template");
    create_and_write_file(&next_env_path, &next_env_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ next.config.js setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let next_config_path = format!("{}/next.config.js", folder_name).to_string();
    let next_config_content: String = std::fs
        ::read_to_string("src/configs/templates/next.config.js.template")
        .expect("Unable to read next.config.js.template");
    create_and_write_file(&next_config_path, &next_config_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ postcss.config.js setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let postcss_config_path = format!("{}/postcss.config.js", folder_name).to_string();
    let postcss_config_content: String = std::fs
        ::read_to_string("src/configs/templates/postcss.config.js.template")
        .expect("Unable to read postcss.config.js.template");
    create_and_write_file(&postcss_config_path, &postcss_config_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ tailwind.config.js setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let tailwind_config_path = format!("{}/tailwind.config.js", folder_name).to_string();
    let tailwind_config_content: String = std::fs
        ::read_to_string("src/configs/templates/tailwind.config.js.template")
        .expect("Unable to read tailwind.config.js.template");
    create_and_write_file(&tailwind_config_path, &tailwind_config_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ tsconfig.json setup ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let ts_config_path = format!("{}/tsconfig.json", folder_name).to_string();
    let ts_config_content: String = std::fs
        ::read_to_string("src/configs/templates/tsconfig.json.template")
        .expect("Unable to read tsconfig.json.template");
    create_and_write_file(&ts_config_path, &ts_config_content)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ create the public folder ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let public_folder_path = format!("./{}/public", configuration.project_name.trim()).to_string();
    if let Err(error) = create_dir_all(public_folder_path) {
        eprint!("Create public folder failed: {}", error);
    }
    // copy next&vercel svg to public folder
    let next_path = format!("./{}/public/next.svg", configuration.project_name.trim()).to_string();
    fs::copy("src/configs/templates/assets/next.svg", next_path)?;
    let vercel_path = format!(
        "./{}/public/vercel.svg",
        configuration.project_name.trim()
    ).to_string();
    fs::copy("src/configs/templates/assets/vercel.svg", vercel_path)?;

    /* ╭━━━━━━━━━━━━━━━━━━━━━━━━ create the app folder ━━━━━━━━━━━━━━━━━━━━━━━━━╮ */
    let app_folder_path = format!("./{}/app", configuration.project_name.trim()).to_string();
    if let Err(error) = create_dir_all(app_folder_path) {
        eprint!("Create app folder failed: {}", error);
    }
    // copy the favicon file to app folder
    let favicon_path = format!(
        "./{}/app/favicon.ico",
        configuration.project_name.trim()
    ).to_string();
    fs::copy("src/configs/templates/assets/favicon.ico", favicon_path)?;

    // create global.css file
    let global_css_path = format!("{}/app/globals.css", folder_name).to_string();
    let global_css_content: String = std::fs
        ::read_to_string("src/configs/templates/globals.css.template")
        .expect("Unable to read globals.css.template");
    create_and_write_file(&global_css_path, &global_css_content)?;

    // create layout.tsx
    let layout_path = format!("{}/app/layout.tsx", folder_name).to_string();
    let layout_content: String = std::fs
        ::read_to_string("src/configs/templates/layout.tsx.template")
        .expect("Unable to read layout.tsx.template");
    create_and_write_file(&layout_path, &layout_content)?;

    // create page.tsx
    let page_path = format!("{}/app/page.tsx", folder_name).to_string();
    let page_content: String = std::fs
        ::read_to_string("src/configs/templates/page.tsx.template")
        .expect("Unable to read lpage.tsx.template");
    create_and_write_file(&page_path, &page_content)?;

    Ok(())
}

fn create_and_write_file(path: &str, content: &str) -> MyResult {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes())?;
    Ok(())
}
