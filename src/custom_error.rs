use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    ArchivoNoEncontrado,
    ArgumentosInsuficientes,
    ExcesoDeArgumentos,
    ErrorEnLecturaDelArchivo,
    CaracterInesperado,
    UsoErroneoDelAsterisk,
    UsoErroneoDelCurvyBracket,
    LineaNoAscii,
    UsoErroneoDelQuestionSign,
    UsoErroneoDelPlus,
    UsoErroneoDelCaret,
    UsoErroneoDelDollarSign

}

pub struct ArgumentosInsuficientes;
pub struct ArchivoNoEncontrado;


impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::ArgumentosInsuficientes => write!(f, "Error. No has ingresado argumentos suficientes para iniciar el programa."),
            CustomError::ExcesoDeArgumentos => write!(f, "Error. Has ingresado mas de dos argumentos. PISTA: Las regular expressions deberian estar encerradas entre comillas dobles"),
            CustomError::ArchivoNoEncontrado => write!(f, "Error. El path ingresado no es correcto o el archivo no existe."),
            CustomError::ErrorEnLecturaDelArchivo => write!(f, "Error durante la lectura del archivo."),
            CustomError::CaracterInesperado => write!(f, "Error: se ha encontrado un caracter inesperado en la regular expression."),
            CustomError::UsoErroneoDelAsterisk => write!(f, "Error: No haz ingresado correctamente el '*'. Asegurate de antecederlo con un caracter"),
            CustomError::UsoErroneoDelQuestionSign => write!(f, "Error: No haz ingresado correctamente el '?'. Asegurate de antecederlo con un caracter"),
            CustomError::UsoErroneoDelPlus => write!(f, "Error: No haz ingresado correctamente el '+'. Asegurate de antecederlo con un caracter"),
            CustomError::UsoErroneoDelCaret => write!(f, "Error: No haz ingresado correctamente el '^'. Asegurate de que sea el primer caracter en tu regex."),
            CustomError::UsoErroneoDelDollarSign => write!(f, "Error: No haz ingresado correctamente el '$'. Asegurate de que sea el ultimo caracter en tu regex."),
            CustomError::UsoErroneoDelCurvyBracket => write!(f, "Error: No haz ingresado correctamente el '{{'. Asegurate de que cierre correctamente con un }}."),
            
            CustomError::LineaNoAscii => write!(f, "Error: la linea ingresada no tiene el formato ascii"),
        }
    }
}