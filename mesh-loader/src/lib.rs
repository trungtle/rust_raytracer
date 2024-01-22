use std::fmt::Error;
use std::{fs::File, default};
use std::io;
use std::io::BufRead;
use std::path::Path;
use ply_rs::{parser, ply::{self, KeyMap, Property}};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn parse_ply(path: &std::path::Path) -> Result<ply::Ply<ply::DefaultElement>, Error> {
    let _ = match File::open(path) {
        Ok(mut file) => {
            let ply_parser = parser::Parser::<ply::DefaultElement>::new();
            Ok(ply_parser.read_ply(&mut file))
        }
        Err(e) => Err(e)
    };
    Err(Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_open_ply() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        let filename = std::path::Path::new("../assets/pbrt4/pbrt-book/geometry/mesh_00001.ply");
        let result = parse_ply(filename);
    }
}
