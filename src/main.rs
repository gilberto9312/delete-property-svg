use std::fs;
use std::path::Path;
use std::io::{self, Write};
use regex::Regex;

fn main() -> io::Result<()> {
    // Obtener la ruta del directorio y la propiedad a eliminar
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <ruta_del_directorio> <propiedad_a_eliminar>", args[0]);
        return Ok(());
    }

    let dir_path = Path::new(&args[1]);
    let property_to_remove = &args[2];

    // Crear una expresión regular para buscar la propiedad y su valor en el archivo SVG
    let re = Regex::new(&format!(r#"\b{}=["'][^"']*["']"#, property_to_remove)).unwrap();

    // Iterar sobre los archivos en el directorio
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;

        if let Some(extension) = entry.path().extension() {
            if extension == "svg" {
                // Leer el archivo SVG
                let svg_content = fs::read_to_string(entry.path())?;

                // Eliminar la propiedad y su valor del contenido SVG
                let new_svg_content = re.replace_all(&svg_content, "");

                // Convertir new_svg_content a String antes de escribir en el archivo
                let new_svg_string = new_svg_content.into_owned();

                // Sobrescribir el archivo con el nuevo contenido
                let mut file = fs::File::create(entry.path())?;
                file.write_all(new_svg_string.as_bytes())?;

                println!("Se ha eliminado la propiedad '{}' del archivo {:?}", property_to_remove, entry.path());
            }
        }
    }

    Ok(())
}
