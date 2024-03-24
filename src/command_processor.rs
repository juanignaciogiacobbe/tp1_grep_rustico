use crate::file_processor::FileProcessor;
use crate::custom_error::CustomError;
//use crate::regex_processor::RegexProcessor;

//const REGULAR_EXPRESSION: usize = 1;
const RUTA_ARCHIVO: usize = 2;
const CANTIDAD_ARGUMENTOS_NECESARIOS: usize = 3; //ingresa el ejecutable, la regex, y el file path.


#[derive(Debug)]
pub struct CommandProcessor {
   // file_processor: FileProcessor,
   // regex_processor: RegexProcessor
}

impl CommandProcessor {
    pub fn build(args: Vec<String>) -> Result<CommandProcessor, CustomError> {  
      if args.len() < CANTIDAD_ARGUMENTOS_NECESARIOS {
          return Err(CustomError::ArgumentosInsuficientes);
      }
        
        //let regular_expression = &args[REGULAR_EXPRESSION];
      let ruta_archivo = &args[RUTA_ARCHIVO];

      let file_processor = FileProcessor::build(ruta_archivo.to_string())?;
        //let regex_processor = RegexProcessor::new(regular_expression.to_string());

      Ok(CommandProcessor {})
    }

    pub fn run(&self) {
      println!("Bienvenido al egrep rustico!");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_no_le_paso_argumentos_al_processor_arroja_error() {
      let args: Vec<String> = vec![];
      let command_processor = CommandProcessor::build(args);
    
      assert!(command_processor.is_err());
    }

    //#[test]
    //fn test_02_le_paso_un_argumento_al_processor_arroja_error() {
      //let args = vec!["./dummy_path".to_string(), "juancito".to_string()];

      //let command_processor = CommandProcessor::build(args);
    
      //assert!(command_processor.is_err());
    //}

    //#[test]
    //fn test_03_le_paso_correctamente_los_argumentos_y_el_reader_se_crea_correctamente() {
      //let args = vec!["./dummy_path".to_string(), "juancito".to_string(), "data.txt".to_string()];

      //let command_processor = CommandProcessor::build(args);

      //assert_eq!(command_processor, Ok(CommandProcessor{regular_expression: "juancito".to_string(), ruta_archivo: "data.txt".to_string()}));
    //}

   // #[test]
    //fn test_04_creo_un_command_processor_y_lee_el_archivo_dado() {
      //let args = vec!["./dummy_path".to_string(), "juancito".to_string(), "./data/data.txt".to_string()];
      //let command_processor = CommandProcessor::build(args);

      //assert!(command_processor.unwrap().run().is_ok());
    //}

}