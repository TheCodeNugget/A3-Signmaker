use std::fs;
use std::fs::File;
use std::env;
use std::io::{self, BufRead, Write};
use std::ops::Add;
use std::path::Path;
use unidecode::unidecode;
use signmaker::p3d::P3D;
use signmaker::{io::Input, io::Output, p3d};
use ril::{self, Font, Image, Rgba, TextLayout, TextSegment};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let sign_type: u8 = (&args[2]).parse().expect("Not Valid");
    assert!(path.is_file(), "{} does not exist", &args[1]);
    assert!((1..5).contains(&sign_type), "Sign Type not valid");

    let mapname = path.file_name().unwrap().to_string_lossy().trim_end_matches(".hpp").to_string();
    create_output_folder(&mapname)?;
    let  (mut start_p3d, mut end_p3d) = prep_signs(&sign_type);
    
    let town_names = collect_town_names(&args[1]);

    for town in town_names.iter() {
        create_town_name_png(&mapname, town, &sign_type, 1024, 128);
        create_town_sign(&mapname, town, &mut start_p3d,&mut end_p3d);
    }
    write_config(&mapname, town_names)?;

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn collect_town_names(keypoints: &String) -> Vec<String> {
    let mut town_names: Vec<String> = Vec::new();
    let mut name_buffer: String = String::new();
    
    if let Ok(lines) = read_lines(keypoints) {
        for line in lines.flatten() {
            if line.contains("name=\"") {
                name_buffer = line.trim().trim_start_matches("name=\"").trim_end_matches("\";").to_string();
            };
            if line.contains("type=\"") {
                let type_buffer = line.trim().trim_start_matches("type=\"").trim_end_matches("\";").to_string();
                if (type_buffer.to_lowercase() == "namecity".to_lowercase()) || (type_buffer.to_lowercase() == "namevillage".to_lowercase()) {
                    town_names.push(name_buffer.clone());
                }
            }
        }
    }
    return town_names;
}

fn create_output_folder (map_name: &String) -> std::io::Result<()> {
    let folder_name = map_name.clone().add("_signs");

    if Path::new(&format!("./{}", folder_name)).exists() {
        fs::remove_dir_all(format!("./{}", folder_name))?;
    };

    fs::create_dir(format!("./{}", folder_name))?;
    fs::create_dir(format!("./{}/data", folder_name))?;
    
    Ok(())
}


fn prep_signs(sign_type: &u8) -> (P3D, P3D) {

    let start_file = match sign_type {
        1 => File::open("./data/models/altis_startsign.p3d").unwrap(),
        2 => File::open("./data/models/livonia_startsign.p3d").unwrap(),
        3 => File::open("./data/models/malden_startsign.p3d").unwrap(),
        4 => File::open("./data/models/tanoa_startsign.p3d").unwrap(),
        _ => File::open("./data/models/altis_startsign.p3d").unwrap(),
    };

    let end_file = match sign_type {
        1 => File::open("./data/models/altis_endsign.p3d").unwrap(),
        2 => File::open("./data/models/livonia_endsign.p3d").unwrap(),
        3 => File::open("./data/models/malden_endsign.p3d").unwrap(),
        4 => File::open("./data/models/tanoa_endsign.p3d").unwrap(),
        _ => File::open("./data/models/altis_endsign.p3d").unwrap(),
    };

    let mut start_input: Input = Input::File(start_file);
    let mut end_input: Input = Input::File(end_file);

    let start_p3d = p3d::P3D::read(&mut start_input).unwrap();
    let end_p3d = p3d::P3D::read(&mut end_input).unwrap();
    return (start_p3d, end_p3d);
}

fn create_town_name_png(map_name: &String, town_name: &String, sign_type: &u8, width: u32, height: u32) {
    let folder_name = map_name.clone().add("_signs");
    let mut image = Image::new(width, height, Rgba::transparent());

    let font = Font::open("./data/fonts/din1451alt.ttf", 128.0).unwrap();

    let text_colour = match sign_type {
        1 => Rgba::black(),
        2 => Rgba::black(),
        3 => Rgba::black(),
        4 => Rgba::white(),
        _ => Rgba::black(),
    };

    let (x, y) = image.center();
    let text = TextLayout::new()
        .centered()
        .with_wrap(ril::WrapStyle::Word)
        .with_width(image.width())
        .with_position(x, y)
        .with_segment(&TextSegment::new(&font, town_name, text_colour));

    image.draw(&text);
    image.save_inferred(format!("./{}/data/{}_ca.png", folder_name, unidecode(town_name).replace(" ", "_"))).unwrap();
}

fn create_town_sign(map_name: &String, town_name: &String, start_sign: &mut P3D, end_sign: &mut P3D) {
    let folder_name = map_name.clone().add("_signs");
    start_sign.lods[0].faces[0].texture = format!("{}\\data\\{}_ca.paa", folder_name, unidecode(town_name).replace(" ", "_")).to_string();
    end_sign.lods[0].faces[0].texture = format!("{}\\data\\{}_ca.paa", folder_name, unidecode(town_name).replace(" ", "_")).to_string();

    let start_file = File::create(format!("./{}/rnc_{}_start.p3d", folder_name, unidecode(town_name).replace(" ", "_"))).unwrap();
    let end_file = File::create(format!("./{}/rnc_{}_end.p3d", folder_name, unidecode(town_name).replace(" ", "_"))).unwrap();
    let mut start_output: Output = Output::File(start_file);
    let mut end_output: Output = Output::File(end_file);
    p3d::P3D::write(&start_sign, & mut start_output).unwrap();
    p3d::P3D::write(&end_sign, & mut end_output).unwrap();
}

fn write_config(map_name: &String, town_names: Vec<String>) -> std::io::Result<()> {
    let folder_name = map_name.clone().add("_signs");
    fs::copy("./data/defines.hpp", format!("./{}/defines.hpp", folder_name))?;
    let mut file = File::create(format!("./{}/config.cpp", folder_name))?;
    file.write_all(b"#include \"defines.hpp\"\n")?;
    file.write_all(format!("PREAMBLE({});\n", map_name).as_bytes())?;
    for town in town_names.iter() {
        file.write_all(format!("SIGN({}, {});\n", unidecode(town).replace(" ", "_"), map_name).as_bytes())?;
    }
    file.write_all(b"};")?;
    Ok(())
}