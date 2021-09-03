use std::fs;
use std::thread;
use std::sync::mpsc;

fn add_veces(cadena: String) -> String {
    let mut result = "".to_string();
    for caracter in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(){
        if cadena.contains(caracter) {
            let mut veces = 0;
            for i in cadena.chars(){
                if i == caracter {
                    veces += 1;
                }
             }
             result.push_str(&format!("{}-{}",caracter, veces));
         }

    }
 result
}

fn is_anagram(a: String, b: String) ->  bool {
    if a.len() != b.len(){
        return false;
    }
    let (txa, rxa) = mpsc::channel();
    let (txb, rxb) = mpsc::channel();
    let hilo1 = thread::spawn(move || txa.send(add_veces(a)).unwrap());
    let hilo2 = thread::spawn(move || txb.send(add_veces(b)).unwrap());
    let result_a = rxa.recv().unwrap();
    let result_b = rxb.recv().unwrap();
    hilo1.join().unwrap();
    hilo2.join().unwrap();
    result_a == result_b

    }


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

    pub fn run_dia4(&self) -> i32 {
        let mut duplicados:i32 = 0; //Contador de duplicados
        let mut lineas:i32 = 0; //Contador de líneas
        for linea in self.contents.lines(){
            lineas += 1;
            let cadena: Vec::<_> = linea
                    .split_whitespace()
                    .collect(); //Convertimos cada una de las líneas en un vector de string
            
            for (pos,valor) in cadena.iter().enumerate() { //Recorremos la cadena  y obtenemos el índice donde nos encontramos
                let inicio = pos + 1; //vamos a recorrer la cadena desde la posición +1 hasta el final
                let mut _encontrado = false; //si antes no nos hemos encontrado con el resultado
                for i in inicio..cadena.len(){
                    if cadena[i] ==  *valor {
                        _encontrado = true;
                        duplicados += 1;
                        break;
                    };
                }
                if _encontrado {
                    break;
                }
                }
            }

        lineas - duplicados
        }

        pub fn run_dia4_2(&self) -> i32 {
            let mut duplicados:i32 = 0; //Contador de duplicados
            let mut lineas:i32 = 0; //Contador de líneas
            for linea in self.contents.lines(){
                lineas += 1;
                let cadena: Vec::<_> = linea
                        .split_whitespace()
                        .collect(); //Convertimos cada una de las líneas en un vector de string

                for (pos,valor) in cadena.iter().enumerate() { //Recorremos la cadena  y obtenemos el índice donde nos encontramos
                    let inicio = pos + 1; //vamos a recorrer la cadena desde la posición +1 hasta el final
                    let mut _encontrado = false; //si antes no nos hemos encontrado con el resultado
                    for i in inicio..cadena.len(){
                        if is_anagram(cadena[i].to_string(),valor.to_string()) {
                            _encontrado = true;
                            duplicados += 1;
                            break;
                        };
                    }
                    if _encontrado {
                        break;
                    }
                    }
                }

            lineas - duplicados
            }


}
