use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "src/templates"]
struct Asset;

fn main() {
        // let seila = Asset::get("teste.j2").map_or_else(
        //     ||{
        //         println!("{}", "File not found");
        //         std::process::exit(1);
        //     },
        //     |d| d
        // );
        let testee = match Asset::get("teste.j2") {
            Some(content) => content,
            None =>   {
            println!("file not found");
            std::process::exit(1);
        } 
         };
        
        let teste =  String::from_utf8_lossy(&testee);
        println!("{}", teste);
        for file in Asset::iter() {
            println!("{}", file.as_ref());
        }
//     let tera = match Tera::new("src/templates/*.j2") {
//         Ok(t) => t,
//         Err(e) => {
//             println!("Parsing error(s): {}", e);
//             ::std::process::exit(1);
//         }
//     };
//     let mut context = Context::new();
//     context.insert("author", "Vinicius Espindola");
//     context.insert("year", "2019");
//    let result = match tera.render("teste.j2", &context){
//         Ok(t) => t,
//         Err(why) => panic!("{}", why)
//    };
//    create_file_license(result);
}


// fn create_file_license( content: String) {
//     let mut file =  match File::create("license"){
//         Err(why) => panic!("couldn't create  {}",  why),
//         Ok(file) => file,
//     };
//     let _result = file.write_all(content.as_bytes());
// }