use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::collections::HashMap;
use test_log::test;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() {
    let mut ply_format: HashMap<String, String> = HashMap::new();
    //ply_format.insert(k, v)

    let path_string = "../assets/pbrt4/pbrt-book/geometry/mesh_00001.ply";
    if let Ok(lines) = read_lines(path_string) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_open_ply() {
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        read();
    }
}
