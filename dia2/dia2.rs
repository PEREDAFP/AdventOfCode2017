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
        "2" =>  {
                println!("El resultado de día 2 parte 1 es: {}",config.run_dia2());
                println!("El resultado de día 2 parte 2 es: {}",config.run_dia2_2())
            },
        _ => println!("No has elegido un día correcto"),
    };
}
