use std::io::{Error,Write};
use tera::{Tera,Context};
use std::fs::File;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/templates"]
struct Asset;

fn main() {
        let template_name = "license_template";
        let license_template_binary = match Asset::get("teste.j2") {
            Some(content) => content,
            None =>   {
            println!("file not found");
            std::process::exit(1);
        } 
        };
        
        let license_template =  String::from_utf8_lossy(&license_template_binary);
        // for file in Asset::iter() {
        //     println!("{}", file.as_ref());
        // }
    let mut tera = Tera::default();
    let mut context = Context::new();
    context.insert("author", "Vinicius Espindola");
    context.insert("year", "2019");
    tera.add_raw_template("license_template", license_template.as_ref()).unwrap();
    let mut context = Context::new();
    context.insert("author", "Vinicius Espindola");
    context.insert("year", "2021");
    let license =  match tera.render(template_name, &context){
        Ok(t) => t,
        Err(e) => {
            println!("Error {}" , e);
            std::process::exit(1);
        }
    };
    match create_file_license(license) {
    Ok(_t) =>  { 
        println!("File created successfully");
        std::process::exit(0);
    },
    Err(_e) => {
        println!("Error to create file license");
        std::process::exit(1);
    }
   };
}


fn create_file_license( content: String) -> Result<(),Error> {
    let mut file =  match File::create("license"){
        Err(why) => panic!("couldn't create  {}",  why),
        Ok(file) => file,
    };
    file.write_all(content.as_bytes())
}