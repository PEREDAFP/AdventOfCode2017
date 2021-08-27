mod lib_advance_of_code1;

use lib_advance_of_code1::Config;
const USAGE: &str  = "Debes utilizar <ejecutable> <diax> <diax.txt>";

fn main(){
    let config = match Config::new(std::env::args()){
        Ok(c)  => c,
        Err(e) => {
            println!("Se ha producido el error: {}", e);
            println!("{}", USAGE);
            std::process::exit(1);
        }
    };

    match config.get_day() {
        "1" =>  {
                println!("El total del primer tramo (día1) es {}  ", config.run_dia1_bis());
                println!("El total del segundo tramo (día1) es {} ", config.run_dia1_segunda());
            },
        _ => println!("No has elegido un día correcto"),
    };
}
