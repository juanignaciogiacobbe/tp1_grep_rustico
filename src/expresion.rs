use std::vec;
use core::fmt::Debug;

pub trait Expresion {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str;
}

impl Debug for dyn Expresion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expresion{{{}}}", "Expresion")
    }
}

//EMPIEZAN LAS EXPRESIONES NORMALES

//basicamente, una expresion con SOLO caracteres normales.
//aqui voy a contemplar el uso del Period '.'
//porque consiste en verlo como un caracter cualquiera a la hora de filtrar.
pub struct ExpresionNormal {
    expresion: String
}


impl ExpresionNormal {
    pub fn new(expresion: &str) -> Self {
        Self { expresion: expresion.to_string() }
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


//EMPIEZAN LAS BRACKET EXPRESSIONS

pub struct Brackets {
    caracteres_dentro_del_bracket: Vec<char>,
    indices_brackets: Vec<usize>,
    expresion: String
}

impl Brackets {
    pub fn new(expresion: &str) -> Self {
        let mut indices_brackets: Vec<usize> = vec![];
        let vector_caracteres: Vec<char> = expresion.chars().collect();
        let mut vector_caracteres_dentro_del_bracket: Vec<char> = vec![];

        for i in 0..vector_caracteres.len() {
            if vector_caracteres[i] == '[' {
                indices_brackets.push(i);
            } else if vector_caracteres[i] == ']' {
                indices_brackets.push(i);

            }
        }

        for j in (indices_brackets[0] + 1)..indices_brackets[1] {
            vector_caracteres_dentro_del_bracket.push(vector_caracteres[j]);
        }

        Self { 
            caracteres_dentro_del_bracket: vector_caracteres_dentro_del_bracket,
            expresion: expresion.to_string(),
            indices_brackets: indices_brackets
        }
    }
}

impl Expresion for Brackets {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let regex_chars: Vec<char> = self.expresion.chars().collect();
        let linea_chars: Vec<char> = linea.chars().collect(); 

        let mut regex_index: usize = 0;
        let mut linea_index: usize = 0;
                
        while regex_index < regex_chars.len() && linea_index < linea_chars.len() {
            if regex_index == self.indices_brackets[0] {
                for char in &self.caracteres_dentro_del_bracket {
                    if char == &linea_chars[linea_index] {
                        linea_index += 1;
                        regex_index = regex_index + self.caracteres_dentro_del_bracket.len() + 2;
                    }
                }

                if regex_index != (self.indices_brackets[1] + 1) { //no matchee con ningun item dentro del bracket
                    regex_index = 0; //no encontre el patron, por lo que sigo buscando dentro de la linea
                    linea_index += 1;
                }
            }

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

//FINALIZAN LAS BRACKET EXPRESSIONS


//EMPIEZAN LAS REPETITION EXPRESSIONS

pub struct Asterisk {
    expresion: String
}

impl Asterisk {
    pub fn new(expresion: &str) -> Self {
        Self{ expresion: expresion.to_string() }
    }
}

impl Expresion for Asterisk {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let regex_chars: Vec<char> = self.expresion.chars().collect();
        let linea_chars: Vec<char> = linea.chars().collect(); 

        let mut indices_elementos_a_repetir: Vec<usize> = vec![];

        for i in 0..regex_chars.len() {
            if regex_chars[i] == '*' {
                indices_elementos_a_repetir.push(i - 1);
            }
        }

        let mut regex_index: usize = 0;
        let mut linea_index: usize = 0;
        let mut index_elementos_a_repetir = 0;
                
        while regex_index < regex_chars.len() && linea_index < linea_chars.len() {
            if regex_index == indices_elementos_a_repetir[index_elementos_a_repetir] {

                while regex_chars[regex_index] == linea_chars[linea_index] {
                    linea_index += 1;
                } 
    
                if regex_chars[regex_index] == '.' {
                    while linea_chars[linea_index] != regex_chars[regex_index + 2] {
                        linea_index += 1;
                    }
                }
                
                index_elementos_a_repetir += 1;
                regex_index += 2; //Me salteo el '*' tambien
            }

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
 

pub struct Plus {
    expresion: String
}

impl Plus {
    pub fn new(expresion: &str) -> Self {
        Self{ expresion: expresion.to_string() }
    }
}

impl Expresion for Plus {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let regex_chars: Vec<char> = self.expresion.chars().collect();
        let linea_chars: Vec<char> = linea.chars().collect(); 

        let mut indices_elementos_a_repetir: Vec<usize> = vec![];

        for i in 0..regex_chars.len() {
            if regex_chars[i] == '+' {
                indices_elementos_a_repetir.push(i - 1);
            }
        }
        
        let mut regex_index: usize = 0;
        let mut linea_index: usize = 0;
        let mut index_elementos_a_repetir = 0;
                
        while regex_index < regex_chars.len() && linea_index < linea_chars.len() {
            if regex_index == indices_elementos_a_repetir[index_elementos_a_repetir] {

                if regex_chars[regex_index] != linea_chars[linea_index] {
                    regex_index = 0; //no encontre el patron, por lo que sigo buscando dentro de la linea
                    linea_index += 1;
                    index_elementos_a_repetir = 0;
                }

                while regex_chars[regex_index] == linea_chars[linea_index] {
                    linea_index += 1;
                } 
    
                if regex_chars[regex_index] == '.' {
                    while linea_chars[linea_index] != regex_chars[regex_index + 2] {
                        linea_index += 1;
                    }
                }
                
                index_elementos_a_repetir += 1;
                regex_index += 2; //Me salteo el '+' tambien
            }

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


//para crear este filtro, necesitamos pasarle un vector
//que contenga rangos(el contenido que va dentro de {}).
pub struct CurvyBrackets {
    expresion: String,
    contenido_llaves: String
}

impl CurvyBrackets {
    pub fn new(expresion: &str, contenido_llaves: &str) -> Self {
        Self{ 
            expresion: expresion.to_string(), 
            contenido_llaves: contenido_llaves.to_string()
        }
    }
}

impl Expresion for CurvyBrackets {
    fn filtrar_linea<'a>(&self, linea: &'a str) -> &'a str {
        let regex_chars: Vec<char> = self.expresion.chars().collect();
        let linea_chars: Vec<char> = linea.chars().collect(); 

        let mut indices_elementos_a_repetir: Vec<usize> = vec![];

        for i in 0..regex_chars.len() {
            if regex_chars[i] == '+' {
                indices_elementos_a_repetir.push(i - 1);
            }
        }
        
        let mut regex_index: usize = 0;
        let mut linea_index: usize = 0;
        let mut index_elementos_a_repetir = 0;
                
        while regex_index < regex_chars.len() && linea_index < linea_chars.len() {
            if regex_index == indices_elementos_a_repetir[index_elementos_a_repetir] {

                if regex_chars[regex_index] != linea_chars[linea_index] {
                    regex_index = 0; //no encontre el patron, por lo que sigo buscando dentro de la linea
                    linea_index += 1;
                    index_elementos_a_repetir = 0;
                }

                while regex_chars[regex_index] == linea_chars[linea_index] {
                    linea_index += 1;
                } 
    
                if regex_chars[regex_index] == '.' {
                    while linea_chars[linea_index] != regex_chars[regex_index + 2] {
                        linea_index += 1;
                    }
                }
                
                index_elementos_a_repetir += 1;
                regex_index += 2; //Me salteo el '+' tambien
            }

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


//FINALIZAN LAS REPETITION EXPRESSIONS


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

    #[test]
    fn test_05_creo_una_expresion_con_asterisk_y_filtro_una_linea_dada() {
        let expresion_con_asterisk = Asterisk::new("abc.*ef*g");

        assert_eq!(expresion_con_asterisk.filtrar_linea("juan dice el abecedario: abcefffg"), "juan dice el abecedario: abcefffg");
    }

    #[test]
    fn test_06_creo_una_expresion_con_plus_y_filtro_una_linea_dada() {
        let expresion_con_asterisk = Plus::new("abcd+ef+g");

        assert_eq!(expresion_con_asterisk.filtrar_linea("juan dice el abecedario: abcdefffg"), "juan dice el abecedario: abcdefffg");
        assert_eq!(expresion_con_asterisk.filtrar_linea("juan dice el abecedario: abcefffg"), "hola");
    }

    #[test]
    fn test_07_creo_una_expresion_con_curvy_brackets_y_filtro_una_linea_dada() {
        let expresion_con_asterisk = CurvyBrackets::new("abcd{2, 4}ef", "2, 4");

        assert_eq!(expresion_con_asterisk.filtrar_linea("juan dice el abecedario: abcddef"), "juan dice el abecedario: abcddef");
    }

    #[test]
    fn test_08_creo_una_expresion_con_brackets_y_filtro_una_linea_dada() {
        let expresion_con_brackets = Brackets::new("la [aeiou] es una vocal");

        assert_eq!(expresion_con_brackets.filtrar_linea("juan dice que la f es una vocal, pero la maestra lo corrigio y le dijo que la e es una vocal"),
                                                         "juan dice que la f es una vocal, pero la maestra lo corrigio y le dijo que la e es una vocal");    
    }

}