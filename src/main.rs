use std::path::PathBuf;

mod types;

fn main() -> anyhow::Result<()> {

    let type_of_licence_arg = std::env::args().nth(1);
    let path_arg = std::env::args().nth(2);

    let mut type_of_licence = String::new();
    let mut path = String::new();

    match type_of_licence_arg {
        Some(s) => type_of_licence = s.trim().to_string(),
        None => {}
    }

    match path_arg {
        Some(s) => path = s.trim().to_string(),
        None => {} 
    }

    types::check_licence(type_of_licence.clone())?;
    let licence_content = types::get_content(type_of_licence);
    
    std::fs::write(PathBuf::from(path + "/LICENCE"), licence_content).unwrap();

    Ok(())
}
