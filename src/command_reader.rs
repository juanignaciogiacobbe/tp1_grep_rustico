use std::env;

#[derive(Debug, PartialEq)]
pub struct CommandReader {
    regular_expression: String,
    ruta_archivo: String,
}

impl CommandReader {
    pub fn build() -> Result<CommandReader, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("Error. No hay suficientes argumentos");
        }

        let regular_expression = args[1].clone();
        let ruta_archivo = args[2].clone();

        Ok(CommandReader {regular_expression, ruta_archivo})
    }
}

//TESTING: PREGUNTAR EN DS

//#[cfg(test)]
//mod tests {
  //  use super::*;

    //#[test]
    //fn test_01_no_le_paso_argumentos_al_reader_arroja_error() {
      //  let command_reader = CommandReader::build(&[]);
    
        //assert!(command_reader.is_err());
    //}

  //  #[test]
    //fn test_02_le_paso_un_argumento_al_reader_arroja_error() {
      //  let command_reader = CommandReader::build(&["./dummy_path".to_string(), "juancito".to_string()]);
    
        //assert!(command_reader.is_err());
   // }

   // #[test]
    //fn test_03_le_paso_correctamente_los_argumentos_y_el_reader_se_crea_correctamente() {
      //  let command_reader = CommandReader::build(&["./dummy_path".to_string(), "juancito".to_string(), "data.txt".to_string()]);

//        assert_eq!(command_reader, Ok(CommandReader{regular_expression: "juancito".to_string(), ruta_archivo: "data.txt".to_string()}));
  //  }

//}