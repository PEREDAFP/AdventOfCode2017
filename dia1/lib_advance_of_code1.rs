use std::fs;

pub struct Config {
    dia: String,
    filename:String,
    contents:Vec<char>,

}
static CERO: u32 = 48;

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
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No hay tercer argumento"),
        };

        let contents = match fs::read_to_string(&filename) {
            Ok(text) =>text.trim().to_string().chars().collect(),
            Err(_) => return Err("Error con el fichero"),
        };

        Ok(Config {dia, filename, contents})
    }

    pub fn get_day(&self) -> &str {
        &self.dia
    }

    pub fn run_dia1(&self) -> u32{
        //Esta a lo tradicional
        let limite = self.contents.len();
        let mut captcha = 0;
        for i in 0..limite{
            if self.contents[i] == self.contents[(i+1)%limite] {
                captcha += (self.contents[i] as u32) - CERO;
            }
        }
        captcha
    }

    pub fn run_dia1_bis(&self) -> u32 {
        //Esta utilizando únicamente iteradores
        let mut v2 = self.contents[1..].to_vec(); //creo un segundo vector desde la posición 1
        v2.push(self.contents[0]); //Añado en la última posición el valor del primer elemento del vector original
                                    //Con esto he conseguido dos vectores desplazados por una posición
        let v3:Vec<(_,_)> = v2.iter()
            .zip(self.contents.iter())
            .collect();     //Realizamos un zip para tener una tupla con el elemento de la posición i y el de i+1
        let captcha:u32 = v3.iter() //Utilizamos sum() para realizar el cálculo aunque antes hemos realizado el map
                                    //para ver si los dos valores son iguales o no
                        .map(|a| if *a.0 == *a.1 { (*a.1 as u32)-CERO  } else {0})
                        .sum();

        captcha

    }

    pub fn run_dia1_segunda(&self) -> u32 {
        //En este caso vamos a volver a realizar las cosas de manera complicadilla
        //Nos creamos un nuevo vector, pero esta vez la mitad del final la ponemos al principio
        //y la mitad del principio la ponemos al final.

        //Realizamos un chain para que se concatenen los dos vectores que hemos creado en uno solo
        //y luego hacemos un zip con el vector original.

        //Esto nos creará una estructura con tuplas del tipo [('1','2'),('2','2)....]

        //Realizaremos al final un sum(), pero ojo, aquí está la virguería:
        //                  Realmente los to_vec y el chain han creado punteros, no valores
        //                  lo que hace que tengamos que comparar cada parte de la tupla con una
        //                  deref diferente.
        //                  Luego tenemos que realizar el cast de punteros

        //Sí, sí. Tanto esta función como la anterior se podrían haber realizado con un bucle y el módulo
        //pero quería probar con estas movidas de los iteradores y demás....

        //Creamos el otro vector pero cambiando la primera por la segunda mitad
        let v2:Vec<_> = self.contents[self.contents.len()/2..].to_vec();
        let v3:Vec<_> = self.contents[..(self.contents.len()/2)].to_vec();
        //Ahora concatenamos, en v2, el vector v3. Ojo, esto llega con nuevos punteros
        let v4:Vec<_> = v2.iter().chain(v3.iter()).collect();
        //El zip ...vamos el [('1','1'),('2','3)...]
        let v5:Vec<(_,_)> = v4.iter()
            .zip(self.contents.iter())
            .collect();
        //Y la función consumidora sum(). Ojo con el follón de la dereferencia de punteros    
        let captcha:u32 = v5.iter()
                        .map(|(a,b)| if **a == *b { ***a as u32 - CERO } else { 0 })
                        .sum();

         captcha
    }
}
