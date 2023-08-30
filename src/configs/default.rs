use std::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use super::Configuration;

pub type MyResult = Result<(), Box<dyn Error>>;

pub fn generate_project_structure(configuration: &Configuration) -> MyResult {
    let folder_name = format!("./{}", configuration.project_name.trim()).to_string();
    let folder_name = folder_name.as_str();

    // create a folder with user input project name
    if let Err(error) = create_dir_all(folder_name) {
        eprint!("Create folder failed: {}", error);
    }

    // create package json file
    let package_json_path = format!("{}/package.json", folder_name).to_string();
    let package_json_content =
        r#"{
            "name": "my-app",
            "version": "0.1.0",
            "private": true,
            "scripts": {
                "dev": "next dev\",
                "build": "next build",
                "start": "next start",
                "lint": "next lint"
            }
        }"#;
    create_and_write_file(&package_json_path, package_json_content)?;
    // write package json file structure

    Ok(())
}

fn create_and_write_file(path: &str, content: &str) -> MyResult {
    let mut file = File::create(path).expect("Create package.json file failed");
    file.write_all(content.as_bytes())?;
    Ok(())
}
