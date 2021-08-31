use std::fs;

pub struct Config {
    dia: String,
    _filename:String,
    contents:String,

}
impl Config {
    //La forma de trabajar con este programa ./ejecutable diax ficherodatosdia.txt
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //Nos saltamos el argumento que indica el nombre del ejecutable
        args.next();

        //Nos aprovechamos del iterador que es args para realizar esta virguería...
        let dia = match args.next() {
            Some(arg) => arg,
            None => return Err("No hay segundo argumento"),
        };
        let _filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No hay tercer argumento"),
        };

        let contents = match fs::read_to_string(&_filename) {
            Ok(text) =>text.to_string(),
            Err(_) => return Err("Error con el fichero"),
        };

        Ok(Config {dia, _filename, contents})
    }

    pub fn get_day(&self) -> &str {
        &self.dia
    }

    pub fn run_dia2(&self) -> i32 {
        let mut resultado:i32 = 0;
        for linea in self.contents.lines(){
            let cadena: Vec<_> = linea.to_string().split_whitespace()
                    .map(|d| match d.parse::<i32>(){
                                  Ok(i) => i,
                                  Err(_) => 0,
                              })
                    .collect();
            let maximo = match cadena.iter().max(){
                    Some(i) => *i,
                    _ => -1000000,
            };
            let minimo = match cadena.iter().min(){
                    Some(i) => *i,
                    _ => -1000000,
            };
            resultado += maximo-minimo;
            }
        resultado
        }
    pub fn run_dia2_2(&self) -> i32 {
        let mut resultado:i32 = 0; //Contador inicializado
        for linea in self.contents.lines(){
            let cadena: Vec<_> = linea.to_string().split_whitespace()
                    .map(|d| match d.parse::<i32>(){
                                  Ok(i) => i,
                                  Err(_) => 0,
                              })
                    .collect(); //Convertimos cada una de las líneas, string separado por espacios, en un i32

            for (pos,valor) in cadena.iter().enumerate() { //Recorremos la cadena  y obtenemos el índice donde nos encontramos
                let inicio = pos + 1; //vamos a recorrer la cadena desde la posición +1 hasta el final
                let mut _encontrado = false; //si antes no nos hemos encontrado con el resultado
                for i in inicio..cadena.len(){
                    if cadena[i] % valor == 0 || valor % cadena[i] == 0{ //Así evitamos que el vector esté ordenado
                        _encontrado = true;
                        resultado += match cadena[i] > *valor{
                            true =>  cadena[i] / valor,
                            false => valor / cadena[i],
                        };

                        break;
                    }
                if _encontrado {
                    break;
                }
                }
            }

        }
        resultado

    }
}
