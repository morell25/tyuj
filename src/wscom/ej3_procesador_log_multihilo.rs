/*
El Escenario: "El Procesador de Logs Inteligente"
Imagina que recibimos líneas de texto (logs) que están "sucias". Queremos procesarlas en 3 etapas:

Etapa 1 (Limpiador): Recibe el texto, lo pone todo en minúsculas y elimina símbolos raros.
Etapa 2 (Analista): Cuenta cuántas palabras tiene y decide si es un log "importante" (ej. si contiene la palabra "error").
Etapa 3 (Escritor): Guarda el resultado final en un "archivo" (nuestra caché o un println!).

*/

use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

#[derive(Debug)]
struct LogProcesado {
    original: String,
    limpio: String,
    num_palabras: usize,
    es_critico: bool,
}

pub fn main() {
    let (tx1, rx1) = mpsc::channel::<LogProcesado>();
    let (tx2, rx2) = mpsc::channel::<LogProcesado>();
    let (tx3, rx3) = mpsc::channel::<LogProcesado>();

    let h1 = et1(rx1, tx2);
    let h2 = et2(rx2, tx3);
    let h3 = et3(rx3);

    let logs_crudos = vec![
        "[INFO] Todo bajo control",
        "[ERROR] Fallo en el motor de renderizado!!",
        "[DEBUG] Revisando punteros -> null",
        "[CRITICAL ERROR] Kernel panic en sector 0x01",
        "[INFO] Todo bajo control",
        "[ERROR] Fallo en el motor de renderizado!!",
        "[DEBUG] Revisando punteros -> null",
        "[CRITICAL ERROR] Kernel panic en sector 0x01",
    ];

    for log in logs_crudos {
        let nuevo_log = LogProcesado {
            original: log.to_string(),
            limpio: String::new(),
            num_palabras: 0,
            es_critico: false,
        };
        tx1.send(nuevo_log).unwrap();
    }

    drop(tx1);

    for x in h1 {
        x.join().unwrap();
    };

    for x in h2 {
        x.join().unwrap();
    };

    for x in h3 {
        x.join().unwrap();
    };


}

fn et1(rx: Receiver<LogProcesado>, sender: Sender<LogProcesado>) -> Vec<thread::JoinHandle<()>> {
    let rx_arc = Arc::new(Mutex::new(rx));
    let mut handles = vec![];
    for _ in 0..=2 {
        //para los hilos
        let rx_arc_cl = rx_arc.clone();
        let sender_clone = sender.clone();

        //spawn de hilo
        let threa_g = thread::spawn(move || {
            loop {
                //bloqueamos el rx hasta recibir el proximo log
                let mensaje = { rx_arc_cl.lock().unwrap().recv() };
                match mensaje {
                    Ok(mut log) => {
                        log.limpio = log.original.to_lowercase();

                        log.limpio = log
                            .limpio
                            .chars()
                            .filter(|a| a.is_alphanumeric() || a.is_whitespace())
                            .collect::<String>();

                        let _ = sender_clone.send(log);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });
        handles.push(threa_g);
    }
    drop(sender);
    handles
}

fn et2(rx: Receiver<LogProcesado>, sx2: Sender<LogProcesado>) -> Vec<thread::JoinHandle<()>> {
    let rx_arc = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    for _ in 0..=2 {
        let rx_arc_cl = rx_arc.clone();
        let sender_clone = sx2.clone();

        let h_guard = thread::spawn(move || {
            loop {
                let mensaje = { rx_arc_cl.lock().unwrap().recv() };

                match mensaje {
                    Ok(mut log) => {
                        if log.limpio.contains("error") {
                            log.es_critico = true;
                        }
                        log.num_palabras = log.limpio.split_whitespace().count();
                        let _ = sender_clone.send(log);
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(h_guard);
    }
    handles
}

fn et3(rx2: Receiver<LogProcesado>) -> Vec<thread::JoinHandle<()>> {
    let rx_arc = Arc::new(Mutex::new(rx2));
    let mut handles = vec![];

    for _ in 0..=2 {
        let rx_arc_cl = rx_arc.clone();

        let h_guard = thread::spawn(move || {
            loop {
                let mensaje = { rx_arc_cl.lock().unwrap().recv() };

                match mensaje {
                    Ok(log) => {
                        println!("{:?}", log)
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(h_guard);
    }
    handles
}
