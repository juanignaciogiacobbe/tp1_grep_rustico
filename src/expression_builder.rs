//use crate::expresion::{Asterisk, Brackets, CurvyBrackets, Expresion, ExpresionNormal, Plus};

use crate::expresion::{Asterisk, Expresion, ExpresionNormal};

//const CARET: char = '^';
const ASTERISK: char = '*';
const PLUS: char = '+';
//const QUESTION_SIGN: char = '?';
const OPEN_CURVY_BRACKET: char = '{';
//const CLOSE_CURVY_BRACKET: char = '}';
const OPEN_BRACKET: char = '[';
//const CLOSE_BRACKET: char = ']';

pub struct ExpressionBuilder {}


impl ExpressionBuilder {
    pub fn construir_expresiones(expresiones: Vec<String>) -> Vec<Box<dyn Expresion>>{
        let mut vector_expresiones:Vec<Box<dyn Expresion>> = vec![];

        for expresion in expresiones {
            // println!("{}", expresion);
            // if expresion.contains(OPEN_BRACKET) {
            //     vector_expresiones.push(Box::new(Brackets::new(&expresion)));

            if expresion.contains(ASTERISK) {
                println!("Tengo un asterisk! y soy {}", expresion);
                vector_expresiones.push(Box::new(Asterisk::new(&expresion)));

            // } else if expresion.contains(PLUS) {
            //     println!("Tengo un plus!");
            //     vector_expresiones.push(Box::new(Plus::new(&expresion)));

            // } else if expresion.contains(OPEN_CURVY_BRACKET) {
            //     println!("Tengo un curvy bracket!");
            //     vector_expresiones.push(Box::new(CurvyBrackets::new(&expresion)));
             } else {
                println!("Soy una expresion normal!");
                vector_expresiones.push(Box::new(ExpresionNormal::new(&expresion)));
            }

        }

        vector_expresiones
    }
}