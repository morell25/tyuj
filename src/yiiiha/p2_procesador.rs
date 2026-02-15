use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

// 300.000.000 -> tiempo: 98.59s -> 14.236.112
// 30.000.000 -> Tiempo total: 10.1586006s -> 1.365.018
// 100.000 -> Tiempo total: 44.9719ms -> 4.015

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
    let f = File::open("./src/yiiiha/hola2.txt")?;
    let reader = BufReader::new(f);
    let mut reader_lines = reader.lines();

    let mut vec_hilos: Vec<JoinHandle<()>> = Vec::with_capacity(NUM_HILOS);

    const LECTURA_LINEA_FICHERO: usize = 100_000;
    const CAPACIDAD_BUFFER: usize = 20;
    const NUM_HILOS: usize = 8;
    const TOTAL_ESTIMADO: u64 = 300_000_000;

    let (tx_log, rx_log) = sync_channel::<Vec<Log>>(CAPACIDAD_BUFFER);

    let rx_log_mut = Arc::new(Mutex::new(rx_log));

    for _ in 0..=NUM_HILOS {
        let rx_log_mut_clo = rx_log_mut.clone();

        let hilo = thread::spawn(move || {
            loop {
                let vector_logs_lock = { rx_log_mut_clo.lock().unwrap().recv() };
                match vector_logs_lock {
                    Ok(vec_log) => for x in vec_log {},
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        vec_hilos.push(hilo);
    }

    let mut total_leido: u64 = 0;
    let inicio_global = Instant::now();
    let mut inicio_tramo = Instant::now();
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
        let num_logs = chunks.len() as u64;
        let _ = tx_log.send(chunks);

        total_leido += num_logs;

        if total_leido % 1_000_000 == 0 {
            let duracion_tramo = inicio_tramo.elapsed().as_secs_f32();
            let velocidad = 1_000_000.0 / duracion_tramo;
            let progreso = (total_leido as f64 / TOTAL_ESTIMADO as f64) * 100.0;

            println!(
                "📊 Progreso: {:.2}% | Procesados: {}M | Velocidad: {:.0} logs/s",
                progreso,
                total_leido / 1_000_000,
                velocidad
            );

            inicio_tramo = Instant::now(); // Reiniciamos el cronómetro para el siguiente tramo
        }
        //println!("{:?}", chunks);
    }

    drop(tx_log);

    for x in vec_hilos {
        x.join().unwrap();
    }

    println!("--- 🏁 FIN DE LECTURA ---");
    println!("Tiempo total: {:?}", inicio_global.elapsed());
    Ok(())
}
