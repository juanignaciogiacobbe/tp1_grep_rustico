use std::error::Error;

use crate::file_processor::FileProcessor;
//use crate::regex_processor::RegexProcessor;

//enum Errores {
  //ArgumentosInsuficientes,
//  FalloEnLecturaDeArchivo,
//}

#[derive(Debug)]
pub struct CommandProcessor {
    file_processor: FileProcessor,
   // regex_processor: RegexProcessor
}

impl CommandProcessor {
    pub fn build(args: Vec<String>) -> Result<CommandProcessor, Box<dyn Error>> {
        
        //ver error handling
        //if args.len() < 3 {
            //return Err(ArgumentosInsuficientes);
        //}

        //let regular_expression = &args[1];
        let ruta_archivo = &args[2];

        let file_processor = FileProcessor::build(ruta_archivo.to_string())?;
        //let regex_processor = RegexProcessor::new(regular_expression.to_string());

        Ok(CommandProcessor {file_processor})
    }

    pub fn run(&self) -> Result<Vec<String>, Box<dyn Error>> {
      //file_processor deberia realizar y devolver las lecturas
      let lecturas = self.file_processor.read_file()?;

      for lectura in &lecturas {
        println!("{:?}", lectura);
      }

      Ok(lecturas)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_no_le_paso_argumentos_al_processor_arroja_error() {
      let args = vec![];
      let command_processor = CommandProcessor::build(args);
    
      assert!(command_processor.is_err());
    }

    #[test]
    fn test_02_le_paso_un_argumento_al_processor_arroja_error() {
      let args = vec!["./dummy_path".to_string(), "juancito".to_string()];

      let command_processor = CommandProcessor::build(args);
    
      assert!(command_processor.is_err());
    }

    //#[test]
    //fn test_03_le_paso_correctamente_los_argumentos_y_el_reader_se_crea_correctamente() {
      //let args = vec!["./dummy_path".to_string(), "juancito".to_string(), "data.txt".to_string()];

      //let command_processor = CommandProcessor::build(args);

      //assert_eq!(command_processor, Ok(CommandProcessor{regular_expression: "juancito".to_string(), ruta_archivo: "data.txt".to_string()}));
    //}

    #[test]
    fn test_04_creo_un_command_processor_y_lee_el_archivo_dado() {
      let args = vec!["./dummy_path".to_string(), "juancito".to_string(), "./data/data.txt".to_string()];
      let command_processor = CommandProcessor::build(args);

      assert!(command_processor.unwrap().run().is_ok());
    }

}