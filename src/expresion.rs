pub trait Expresion {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str;
}

//EMPIEZAN LAS ANCHORING EXPRESSIONS

//El filtro caret '^' consiste en filtrar aquellas lineas que inicien
//con un patron definido
pub struct Caret {
    expresion_normal: ExpresionNormal
}

impl Caret {
    pub fn new(expresion: &str) -> Self {
        let filtro : String = expresion.chars().skip(1).collect();

        let expresion_normal = ExpresionNormal::new(&filtro);

        Self { expresion_normal }
    }
}

impl Expresion for Caret {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let largo_expresion_normal = self.expresion_normal.get_largo();
        let primeros_caracteres_linea = &linea[0..largo_expresion_normal];
        
        let resultado_filtro = self.expresion_normal.filtrar_linea(primeros_caracteres_linea);

        if resultado_filtro == primeros_caracteres_linea {
            linea
        } else {
            "hola"
        }
    }
}

//El filtro dollar sign '$' consiste en filtrar aquellas lineas que terminen
//con un patron definido
pub struct DollarSign {
    expresion_normal: ExpresionNormal

}

impl DollarSign {
    pub fn new(expresion: &str) -> Self {
        let filtro : String = expresion.chars().take(expresion.len() - 1).collect();

        let expresion_normal = ExpresionNormal::new(&filtro);

        Self { expresion_normal }
    }
}

impl Expresion for DollarSign {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let largo_expresion_normal = self.expresion_normal.get_largo();
        let ultimos_caracteres_linea = &linea[(linea.len() - largo_expresion_normal)..(linea.len())];

        let resultado_filtro = self.expresion_normal.filtrar_linea(ultimos_caracteres_linea);

        if resultado_filtro == ultimos_caracteres_linea {
            linea
        } else {
            "hola"
        }
    }
}

//FINALIZAN LAS ANCHORING EXPRESSIONS

//EMPIEZAN LAS REPETITION EXPRESSIONS

pub struct Asterisk {
       
}

 

//FINALIZAN LAS REPETITION EXPRESSIONS


//EMPIEZAN LAS EXPRESIONES NORMALES

//basicamente, una expresion con SOLO caracteres normales.
//aqui voy a contemplar el uso del Period '.'
//porque consiste en verlo como un caracter cualquiera a la hora de filtrar.
pub struct ExpresionNormal {
    expresion: String
}

impl ExpresionNormal {
    pub fn new(expresion: &str) -> Self {
        let expresion_final = expresion.to_string();
        
        //busco si dentro de la expresion tengo algun period '.'
        //for caracter in expresion.chars() {
            //  match caracter {
                //    '.' => expresion_final = procesar_period(&expresion),
                //    _ => println!("Soy un caracter normal")
                //};
                //}
                
                //println!("{}", expresion_final);
        Self { expresion: expresion_final }
    }

    pub fn get_largo(&self) -> usize {
        self.expresion.len()
    }
}
        
impl Expresion for ExpresionNormal {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let regex_chars: Vec<char> = self.expresion.chars().collect();
        let linea_chars: Vec<char> = linea.chars().collect();
                
        let mut regex_index: usize = 0;
        let mut linea_index: usize = 0;
                
        while regex_index < regex_chars.len() && linea_index < linea_chars.len() {
            if regex_chars[regex_index] == '.' {
                regex_index += 1;
                linea_index += 1;
                        
            } else if regex_chars[regex_index] == linea_chars[linea_index] {
                regex_index += 1;
                linea_index += 1;
                        
            } else {
                regex_index = 0; //no encontre el patron, por lo que sigo buscando dentro de la linea
                linea_index += 1;
                        
            }
        }
                
        if regex_index == (regex_chars.len()) {
            linea
        } else {
            "hola"
        }
                
    } 
}
        
//FINALIZAN LAS EXPRESIONES NORMALES
        
#[cfg(test)]
mod tests {
    use super::*;
            
    #[test]
    fn test_01_creo_una_expresion_normal_y_filtro_una_linea_dada() {
        let expresion_normal = ExpresionNormal::new("abcd");

        assert_eq!(expresion_normal.filtrar_linea("abcdefghijk"), "abcdefghijk");
    }

    #[test]
    fn test_02_creo_expresion_normal_con_un_period_y_filtro_una_linea_dada() {
        let expresion_normal = ExpresionNormal::new("a.c.e");

        assert_eq!(expresion_normal.filtrar_linea("abcde"), "abcde");
    }


    #[test]
    fn test_03_creo_una_expresion_con_caret_y_filtro_una_linea_dada() {
        let expresion_con_caret = Caret::new("^ab.cd");

        assert_eq!(expresion_con_caret.filtrar_linea("abecdefghijk"), "abecdefghijk");
    }

    #[test]
    fn test_04_creo_una_expresion_con_dollar_sign_y_filtro_una_linea_dada() {
        let expresion_con_dolar_sign = DollarSign::new("abcd$");

        assert_eq!(expresion_con_dolar_sign.filtrar_linea("juan dice el abecedario: abcd"), "juan dice el abecedario: abcd");

    }


}