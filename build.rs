use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("auto_types.rs");

    // Load the template for type definition
    let template = fs::read_to_string("type.hbl.rs").unwrap();

    // Load json data for the template
    let json = fs::read_to_string("message.json").unwrap();
    let data: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();

    // Start handlebars
    let mut handlebars = handlebars::Handlebars::new();
    assert!(handlebars.register_template_string("t1", template).is_ok());

    // Write data to file
    fs::write(&dest_path, handlebars.render("t1", &data).unwrap()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=message.json");
    println!("cargo:rerun-if-changed=type.hbl.rs");
}