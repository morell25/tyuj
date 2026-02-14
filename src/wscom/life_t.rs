/*

El Escenario: "El Procesador de Logs Inteligente"
Imagina que recibimos líneas de texto (logs) que están "sucias". Queremos procesarlas en 3 etapas:

Etapa 1 (Limpiador): Recibe el texto, lo pone todo en minúsculas y elimina símbolos raros.

Etapa 2 (Analista): Cuenta cuántas palabras tiene y decide si es un log "importante" (ej. si contiene la palabra "error").

Etapa 3 (Escritor): Guarda el resultado final en un "archivo" (nuestra caché o un println!).

*/

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

#[derive(Debug)]
struct LogProcesado {
    original: String,
    limpio: String,
    num_palabras: usize,
    es_critico: bool,
}

pub fn main_t() {
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

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

}

fn et1(rx: Receiver<LogProcesado>, sender: Sender<LogProcesado>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(mut texto) = rx.recv() {
            texto.limpio = texto.original.to_lowercase();

            let log_c = texto.limpio;

            texto.limpio = log_c
                .chars()
                .filter(|a| a.is_alphanumeric() || a.is_whitespace())
                .collect::<String>();

            sender.send(texto);
        }
    })
}

fn et2(rx: Receiver<LogProcesado>, sx2: Sender<LogProcesado>)-> thread::JoinHandle<()>{
    thread::spawn(move || {
        while let Ok(mut lol) = rx.recv() {
            let mut word_counter = 0;
            if lol.limpio.contains("error") {
                lol.es_critico = true;
            }
            for x in lol.limpio.chars() {
                if x.is_whitespace() {
                    word_counter += 1;
                } else {
                    //println!("palabra: {x}")
                }
            }

            lol.num_palabras = word_counter;

            sx2.send(lol);
        };
    })
}

fn et3(rx2: Receiver<LogProcesado>)  -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(lol) = rx2.recv() {
            //Escribo en archivo
            print!("{:?}", lol);
        }
    })
}
