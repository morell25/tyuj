use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
enum TypeLog {
    INFO,
    WARN,
    CRITICAL,
}

// 300.000.000 -> tiempo: 168.13s -> 14.236.112
// 30.000.000 -> tiempo: 1.2897451s
// 100.000 -> tiempo: 5.8427ms

// Usamos referencias (&str) para no copiar texto, solo apuntamos a la memoria
#[derive(Debug)]
struct Log<'a> {
    numero_log: u64,
    tipo_log: TypeLog,
    mensaje: &'a str,
}

#[derive(Debug)]
struct ParseLogError;

impl FromStr for TypeLog {
    type Err = ParseLogError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INFO" => Ok(TypeLog::INFO),
            "CRITICAL" => Ok(TypeLog::CRITICAL),
            "WARN" => Ok(TypeLog::WARN),
            _ => Err(ParseLogError),
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let ruta = "./src/yiiiha/hola2.txt";
    let inicio = Instant::now();

    // 1. Mapeo de memoria: El SO carga el archivo de forma eficiente
    let file = File::open(ruta)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // Convertimos el buffer de bytes a un string slice (asumiendo UTF-8)
    let content = std::str::from_utf8(&mmap).expect("Archivo no es UTF-8");

    // 2. Procesamiento paralelo con Rayon
    // par_lines() divide el trabajo automáticamente entre tus núcleos
    let total_logs: usize = content
        .par_lines()
        .map(|linea| {
            // Procesamiento ultra rápido de la línea
            if let Some(log) = parse_line(linea) {
                1
            } else {
                0
            }
        })
        .sum();

    let duracion = inicio.elapsed();
    println!("Procesados {} logs", total_logs);
    println!("El programa tardó: {:?}", duracion);

    Ok(())
}

// Función auxiliar de parseo sin asignaciones
fn parse_line(linea: &str) -> Option<Log> {
    let mut partes = linea.split('|');

    let num_str = partes.next()?;
    let tipo_str = partes.next()?;
    let mensaje = partes.next()?;

    // Parseo numérico y de enum (muy rápidos)
    let numero_log = num_str.parse::<u64>().ok()?;
    let tipo_log = tipo_str.parse::<TypeLog>().ok()?;

    Some(Log {
        numero_log,
        tipo_log,
        mensaje,
    })
}
