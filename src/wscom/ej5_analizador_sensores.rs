/*
El Escenario: "El Analizador de Calidad de Sensores"
Imagina una fábrica con miles de sensores de temperatura. Los sensores envían lecturas constantemente, pero los datos vienen "sucios" y desordenados. Necesitamos un sistema que procese estas lecturas en tiempo real.
El Objetivo:
Tienes que crear un sistema que reciba un Vec<Lectura> y nos diga la temperatura media por tipo de sensor.
Las 3 Fases del Sistema:
Fase 1: El Validador (Paralelo con thread::scope):
Recibe el Vec inicial.
Divide el trabajo en chunks.
Cada hilo debe filtrar las lecturas: si la temperatura es menor a -50°C o mayor a 150°C, es un error del sensor y se descarta.
Pasa las lecturas válidas a la siguiente fase mediante un canal.
Fase 2: El Clasificador (Hilo Único):
Recibe las lecturas válidas del canal.
Su trabajo es agruparlas. Pero para no saturar al de la fase 3, solo envía paquetes de 10 en 10 (un Vec de 10 lecturas cada vez).
Fase 3: El Estadístico (Hilo Único):
Recibe los paquetes de 10 lecturas.
Mantiene un HashMap global donde guarda: (TipoDeSensor, (SumaTotalTemps, CantidadDeLecturas)).
Al final, imprime la media de cada tipo.
*/
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::thread;

#[derive(Debug, Clone)]
struct Lectura {
    sensor_id: u32,
    tipo: String,
    valor: f64,
}

pub fn main() {
    let inicio = Instant::now();
    let estadisticas = Mutex::new(HashMap::<String, (f64, u32)>::new());
    let lecturas = generar_lecturas(1_00_000);
    //50 millones -> 34 segundos
    
    let (tx1, rx1) = channel(); 
    let (tx2, rx2) = channel::<Vec<&Lectura>>(); 

    thread::scope(|s| {
        for x in lecturas.chunks(250) {
            let t1_clone = tx1.clone();
            s.spawn(move || {
                for i in x {
                    if i.valor >= -50.0 && i.valor <= 150.0 {
                        let _ = t1_clone.send(i);
                    }
                }
            });
        }
        drop(tx1);

        let t2_clone = tx2.clone();
        s.spawn(move || {
            let mut vec_envio = Vec::new();
            while let Ok(filtrado) = rx1.recv() {
                vec_envio.push(filtrado);
                if vec_envio.len() >= 10 {
                    let _ = t2_clone.send(vec_envio);
                    vec_envio = Vec::new();
                }
            }
            if !vec_envio.is_empty() {
                let _ = t2_clone.send(vec_envio);
            }
            drop(t2_clone);
        });
        drop(tx2);

        let estadisticas_ref = &estadisticas; 
        s.spawn(move || {
            while let Ok(sensores_info) = rx2.recv() {
                let mut guard = estadisticas_ref.lock().unwrap();
                for x in sensores_info {
                    guard.entry(x.tipo.clone())
                        .and_modify(|(sum, count)| {
                            *sum += x.valor;
                            *count += 1;
                        })
                        .or_insert((x.valor, 1));
                }
            }
        });
    }); 

    let resultados = estadisticas.into_inner().expect("");
    
    
    
    for (tipo, (suma, cuenta)) in resultados {
        let media = suma / cuenta as f64;
        println!("Sensor: {:<12} | Muestras: {:<4} | Media: {:.2}°C", tipo, cuenta, media);
    }
    let duracion = inicio.elapsed();
    println!("El programa tardó: {:?}", duracion);
}

// Generador de datos de prueba, generado con IA
fn generar_lecturas(cantidad: usize) -> Vec<Lectura> {
    let tipos = vec!["Temperatura", "Humedad", "Presion", "CO2"];
    let mut lecturas = Vec::with_capacity(cantidad);

    let mut semilla = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for i in 0..cantidad {
        semilla = semilla.wrapping_mul(1103515245).wrapping_add(12345);
        let valor_azar = (semilla % 300) as f64 - 70.0; // Rango -70 a 230

        lecturas.push(Lectura {
            sensor_id: i as u32,
            tipo: tipos[i % tipos.len()].to_string(),
            valor: valor_azar,
        });
    }
    lecturas
}