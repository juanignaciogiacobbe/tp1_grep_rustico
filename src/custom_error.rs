use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    ArchivoNoEncontrado,
    ArgumentosInsuficientes,
    ExcesoDeArgumentos,
    ErrorEnLecturaDelArchivo
}

pub struct ArgumentosInsuficientes;
pub struct ArchivoNoEncontrado;


impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::ArgumentosInsuficientes => write!(f, "Error. No has ingresado argumentos suficientes para iniciar el programa."),
            CustomError::ExcesoDeArgumentos => write!(f, "Error. Has ingresado mas de dos argumentos. PISTA: Las regular expressions deberian estar encerradas entre comillas dobles"),
            CustomError::ArchivoNoEncontrado => write!(f, "Error. El path ingresado no es correcto o el archivo no existe."),
            CustomError::ErrorEnLecturaDelArchivo => write!(f, "Error durante la lectura del archivo.")
        }
    }
}