use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::space0,
    combinator::{map_res, recognize},
    multi::{count, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct PlyData {
    vertices: Vec<Vertex>,
    // Add other properties as needed
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    map_res(recognize(pair(opt(tag("-")), take_while1(|c: char| c.is_digit(10)))), str::parse)(input)
}

fn parse_vertex(input: &str) -> IResult<&str, Vertex> {
    let (input, _) = space0(input)?;
    let (input, x) = parse_float(input)?;
    let (input, _) = space0(input)?;
    let (input, y) = parse_float(input)?;
    let (input, _) = space0(input)?;
    let (input, z) = parse_float(input)?;

    Ok((input, Vertex { x, y, z }))
}

fn parse_header(input: &str) -> IResult<&str, ()> {
    let (input, _) = terminated(preceded(tag("ply"), take_while(|c: char| c.is_whitespace())), tag("format ascii 1.0\n"))(input)?;
    let (input, _) = terminated(
        preceded(tag("element vertex "), map_res(take_while(|c: char| c.is_digit(10)), str::parse)),
        tag("\n"),
    )(input)?;
    let (input, _) = terminated(
        preceded(tag("property"), take_while(|c: char| !c.is_whitespace())),
        tag("\n"),
    )(input)?;
    let (input, _) = terminated(preceded(tag("end_header\n"), space0), tag("\n"))(input)?;

    Ok((input, ()))
}

fn parse_data(input: &str) -> IResult<&str, PlyData> {
    let (input, _) = parse_header(input)?;
    let (input, vertices) = separated_list1(tag("\n"), parse_vertex)(input)?;

    Ok((input, PlyData { vertices }))
}

mod test {
    #[test]
    fn test_load_ply() {
        let input_data = r#"
            ply
            format ascii 1.0
            element vertex 3
            property float x
            property float y
            property float z
            end_header
            1.0 2.0 3.0
            4.0 5.0 6.0
            7.0 8.0 9.0
        "#;
    
        match parse_data(input_data) {
            Ok((_, ply_data)) => {
                println!("{:?}", ply_data);
            }
            Err(err) => {
                eprintln!("Error parsing PLY data: {:?}", err);
            }
        }
    }
    
}

