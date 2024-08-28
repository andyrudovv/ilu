use std::path::PathBuf;


pub enum Licence {
    Nil,
    MIT
}

pub fn check_licence(s: String) -> anyhow::Result<()> {
    let licence = match s.to_lowercase().as_str() {
        "mit" => Licence::MIT,
        _ => Licence::Nil
    };

    match licence {
        Licence::Nil => panic!(),
        _ => {}
    }

    Ok(())
}

pub fn get_content(l: String) -> String {
    let path = "licence_content/".to_string() + &l;

    println!("{}", path);

    std::fs::read_to_string(PathBuf::from(path)).unwrap()
}
