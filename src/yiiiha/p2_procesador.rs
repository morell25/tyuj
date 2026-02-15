use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Log {
    numero_log: u64,
    tipo_log: TypeLog,
    mensaje: String,
}

#[derive(Debug, Clone)]
enum TypeLog {
    INFO,
    WARN,
    CRITICAL,
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
    let inicio = Instant::now();
    let f = File::open("./src/yiiiha/hola.txt")?;
    let reader = BufReader::new(f);
    let mut reader_lines = reader.lines();

    let mut vec_hilos: Vec<JoinHandle<()>> = Vec::new();

    const LECTURA_LINEA_FICHERO: usize = 50000;
    const CAPACIDAD_BUFFER: usize = 30;

    let (tx_log, rx_log) = sync_channel::<Vec<Log>>(CAPACIDAD_BUFFER);

    let rx_log_mut = Arc::new(Mutex::new(rx_log));

    for _ in 0..=4 {
        let rx_log_mut_clo = rx_log_mut.clone();

        let hilo = thread::spawn(move || {
            loop {
                let vector_logs_lock = { rx_log_mut_clo.lock().unwrap().recv() };
                match vector_logs_lock {
                    Ok(vec_log) => {
                        for x in vec_log {
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        vec_hilos.push(hilo);
    }


    loop {
        let mut chunks: Vec<Log> = Vec::with_capacity(LECTURA_LINEA_FICHERO);
        for texto in reader_lines.by_ref().take(LECTURA_LINEA_FICHERO) {
            match texto {
                Ok(log) => {
                    let log_sin_struct: Vec<&str> = log.split("|").collect();
                    let struct_log = Log {
                        numero_log: log_sin_struct[0].parse().unwrap(),
                        tipo_log: log_sin_struct[1].parse().unwrap(),
                        mensaje: log_sin_struct[2].to_string(),
                    };
                    chunks.push(struct_log);
                }
                Err(_) => {
                    break;
                }
            }
        }
        if chunks.is_empty() {
            break;
        }
        let _ = tx_log.send(chunks);
        //println!("{:?}", chunks);
    }

    drop(tx_log);

    

    for x in vec_hilos {
        x.join().unwrap();
    }
    let duracion = inicio.elapsed();
    println!("El programa tardó: {:?}", duracion);
    Ok(())
}
