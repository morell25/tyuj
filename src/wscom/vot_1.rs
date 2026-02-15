use std::{collections::HashMap, sync::mpsc::channel, thread};

/*
El Escenario: "El Sistema de Votación Distribuido"
Imagina que tenemos un sistema de votación para un concurso. Los votos llegan en crudo y necesitamos procesarlos, pero hay un requisito: No podemos anunciar los resultados hasta que todos los hilos hayan terminado de procesar su parte.

Las Reglas del Juego:
Entrada: Un Vec<Voto> donde cada voto tiene un candidato (String) y una region (String).
Reparto: Tienes que dividir ese Vec en 4 trozos (o lanzar 4 hilos que vayan consumiendo de una cola).
Procesamiento Local: Cada hilo debe tener su propio "conteo local" (un HashMap privado). Esto es para evitar bloquear el HashMap global cada vez que llega un voto (lo cual sería lentísimo).
Agregación Final: Cuando un hilo termina de procesar su trozo, debe enviar su HashMap local al hilo principal (o a un hilo recolector).
Resultado: El hilo principal suma todos los conteos locales y da el ganador.
*/

struct Voto {
    candidato: String,
    region: String,
}
pub fn main_t() {
    let vec_total_votos: Vec<Voto> = generar_votos();
    let (tx, rx) = channel();
    thread::scope(|p| {
        for x in vec_total_votos.chunks(25) {
            let tx_cloned = tx.clone();
            p.spawn(move || {
                let mut conteo = HashMap::new();
                for z in x {
                    *conteo.entry(z.candidato.clone()).or_insert(0) += 1;
                }
                tx_cloned.send(conteo).unwrap();
            });
        }
        drop(tx);
    });

    while let Ok(hola) = rx.recv() {
        println!("{:?}", hola)
    }
}

fn generar_votos() -> Vec<Voto> {
    let candidatos = vec!["Rustacean", "Gopher", "Pythonista", "JavaDev"];
    let regiones = vec!["Norte", "Sur", "Este", "Oeste"];

    let mut votos = Vec::new();

    // Generamos 100 votos de prueba
    for i in 0..100 {
        votos.push(Voto {
            // Repartimos los votos usando el índice para no meter una librería de azar externa
            candidato: candidatos[i % candidatos.len()].to_string(),
            region: regiones[(i / 2) % regiones.len()].to_string(),
        });
    }
    votos
}
