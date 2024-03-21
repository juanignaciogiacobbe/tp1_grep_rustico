use core::fmt::Debug;


#[derive(Debug, PartialEq)]
struct CaracterNormal {
    caracter: char,
}

#[derive(Debug, PartialEq)]
struct Metacaracter {
    caracter: char,
}

pub trait Caracter {
    fn get_caracter(&self) -> char;
}

impl Debug for dyn Caracter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Caracter{{{}}}", self.get_caracter())
    }
}

impl Caracter for CaracterNormal {
    fn get_caracter(&self) -> char { 
        self.caracter.clone()
    }
}

impl CaracterNormal {
    pub fn new(caracter: char) -> Self {
        Self {
            caracter: caracter
        }
    }
}

impl Caracter for Metacaracter {
    fn get_caracter(&self) -> char { 
        self.caracter.clone()
    }
}

impl Metacaracter {
    pub fn new(caracter: char) -> Self {
        Self {
            caracter: caracter
        }
    }
}

pub fn definir_caracter(caracter: char) -> Box<dyn Caracter> {
    match caracter {
        '.' => Box::new(Metacaracter::new(caracter)),
        '^' => Box::new(Metacaracter::new(caracter)),
        '$' => Box::new(Metacaracter::new(caracter)),
        '*' => Box::new(Metacaracter::new(caracter)),
        _ => Box::new(CaracterNormal::new(caracter)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_creo_correctamente_un_caracter_del_tipo_normal_y_un_metacaracter() {
        let caracter_normal: CaracterNormal = CaracterNormal::new('h');
        let metacaracter: Metacaracter = Metacaracter::new('*');


        assert_eq!(caracter_normal.get_caracter(), 'h');
        assert_eq!(metacaracter.get_caracter(), '*');
    }

    #[test]
    fn test_02_ingreso_un_caracter_y_se_define_como_caracter_normal() {
        let caracter: Box<dyn Caracter> = definir_caracter('h');

        assert_eq!(caracter.get_caracter(), 'h');
    }

    #[test]
    fn test_03_ingreso_un_metacaracter_y_lo_clasifica_correctamente() {
        let caracter: Box<dyn Caracter> = definir_caracter('*');

        assert_eq!(caracter.get_caracter(), '*');
    }
}