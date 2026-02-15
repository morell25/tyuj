/*
El Escenario: "El Craqueador de Hashes"
Para no complicarnos con criptografía real, vamos a simular que "romper" un hash es simplemente encontrar qué número, al ser multiplicado por un secreto, da un resultado específico. Algunos números tardarán más en procesarse que otros (simulando carga variable).

Estructura del Reto:
La Tarea:
Rust
struct Tarea {
    id: u32,
    hash_objetivo: u64,
    dificultad: u32, // Cuántas iteraciones debe hacer el hilo
}
El Canal de Trabajo: El main crea un canal (tx_tareas, rx_tareas).
Los Trabajadores: Lanzas 4 hilos. Los 4 comparten el mismo rx_tareas (necesitarás Arc<Mutex<Receiver<Tarea>>>).

Dinámica:

-El main lanza 20 tareas al canal.
-Cada hilo hace un loop y dice: "Dame la siguiente tarea".
-El hilo procesa la tarea (un simple bucle de dificultad iteraciones).
-Al terminar, el hilo envía el resultado por otro canal de salida hacia el main.

*/

use std::{sync::{Arc, Mutex, mpsc::{Receiver, channel}}, thread::{self, JoinHandle}, time::{Duration, Instant}};

#[derive(Debug)]
struct Tarea {
    id: u32,
    hash_objetivo: u64,
    dificultad: u32, // Cuántas iteraciones debe hacer el hilo
}

pub fn main(){
    let inicio = Instant::now();
    // diezmil tareas con 25 hilos virtuales
    // 20% de uso mas o menos en promedio
    // 120.54
    // Medicion haciendo dormir al hilo entre 100 y 500 milisegundos

let (tx_tareas, rx_tareas) = channel();
let (tx_resultados, rx_resultados) = channel::<Tarea>(); 

//Cinta hacer referencia a lo que seria una cinta de trabajo
//donde el hilo principal envia tarea
//y los n hilos hijos las van recibiendo

/*
    Esto lo hacemos asi porque con un .chunk tenemos el "problema" de que si un chunk es muy pesado de procesar
    el resto de hilos quedarian esperando

*/

let cinta:Arc<Mutex<Receiver<Tarea>>> = Arc::new(Mutex::new(rx_tareas));
let mut vec_hilos:Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..=24 {
        let cinta_c = cinta.clone();
        let enviador = tx_resultados.clone();
        let join_hanlder = thread::spawn(move||{
            loop {
                let tarea = {cinta_c.lock().unwrap().recv()};

                match tarea {
                    Ok(tarea_filtrada) => {
                        thread::sleep(Duration::from_millis(tarea_filtrada.dificultad as u64));
                        let _ = enviador.send(tarea_filtrada);
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });
        vec_hilos.push(join_hanlder);
    }


    for i in 0..10000 {
        let dificultad = (i % 5 + 1) * 100; // Unas tardan más que otras
        tx_tareas.send(Tarea { id: i, hash_objetivo: i as u64 * 100, dificultad }).unwrap();
    }

    drop(tx_tareas);
    drop(tx_resultados);

    while let Ok(tarea) = rx_resultados.recv() {
        println!("{:?}", tarea)
    }

    for x in vec_hilos {
        x.join().unwrap();
    }
let duracion = inicio.elapsed();
    println!("El programa tardó: {:?}", duracion);

}

