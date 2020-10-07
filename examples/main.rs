use std::fs::File;
use std::io::prelude::*;

use pdf_shape::*;
use quick_xml::de::from_str;

fn main() {
    let mut file = File::open("./examples/xml_sample/sample_1.xml").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let document: Document = from_str(&contents).unwrap();
    let tokens = document.get_fsm_tokens();

    println!("Tokens width : {:?} pt", tokens.width());
    println!("Token heigths : {:?} pt", tokens.height());
    println!(
        "All horizontal spacing (pt) : {:#?}",
        tokens.horizontal_spacing()
    );
    println!(
        "All vertical spacing (pt) : {:#?}",
        tokens.vertical_spacing()
    );

    // Words horizontal spacing (mode)
    if let Some(horiztontal_spacing_mode) = tokens.mode_horizontal_spacing() {
        println!(
            "Horizontal spacing (mode) : {} pt",
            horiztontal_spacing_mode
        );
    }

    // Lines vertical spacing (mode)
    if let Some(vertical_spacing_mode) = tokens.mode_vertical_spacing() {
        println!("Vertical spacing (mode) : {} pt", vertical_spacing_mode);
    }
}
