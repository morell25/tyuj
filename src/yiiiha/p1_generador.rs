use std::fs::File;
use std::io::{Write, BufWriter};

/// Genera un fichero de texto plano con el formato: ID|NIVEL|MENSAJE
pub fn generar_fichero_logs(ruta: &str, cantidad: usize) -> std::io::Result<()> {
    let file = File::create(ruta)?;
    let mut writer = BufWriter::new(file);
    let niveles = vec!["INFO", "CRITICAL", "WARN"];

    for i in 0..cantidad {
        let nivel = niveles[i % niveles.len()];
        // Usamos writeln! para añadir el salto de línea automáticamente
        writeln!(writer, "{}|{}|Mensaje de log número {}", i, nivel, i)?;
    }
    
    // flush asegura que todo lo que está en el buffer se escriba al disco
    writer.flush()?;
    println!("Fichero '{}' generado con {} líneas.", ruta, cantidad);
    Ok(())
}