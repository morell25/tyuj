use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::collections::VecDeque;
mod wscom {
    pub mod wscom_main;
}


#[derive(Debug, Clone)]
struct Proces {
    name: String,
    estado: Estados,
    tiempo_restante: Duration,
    num_veces: u8
}

#[derive(Debug, Clone)]
enum Estados {
    Inicial,
    EnCpu,
    Finalizado
}

fn main() {
    //canal para enviar
    let (tx, rx) = channel::<Proces>();
    //lista de procesos
    let mut buf_vec: VecDeque<Proces>= VecDeque::new();
    //hilos activos
    let mut h_a: i32 = 0;

    //generamos los procesos
    for x in 0..=100 {
        let pro= Proces {
            estado : Estados::Inicial,
            name: format!("process{}", x),
            tiempo_restante: Duration::from_secs(rand::random_range(0..=2)),
            num_veces: 0, 
        };
        //added
        buf_vec.push_back(pro);
    }

    loop {
        while h_a < 4 {
            if let Some(mut proceso) = buf_vec.pop_front() {
                let tx_clone: std::sync::mpsc::Sender<Proces> = tx.clone();
                proceso.estado = Estados::EnCpu;
                h_a += 1;
                thread::spawn(move||{
                    //round robin del proceso
                    let r = round(proceso);
                    //Lo envio por el canal como tiempo restado
                    let _ = tx_clone.send(r);
                });
            } else {
                break;
            }
        }
        if h_a == 0 {break}

        if let Ok(mut recibido) = rx.recv() {
            h_a -= 1;
            if !recibido.tiempo_restante.is_zero() {
                buf_vec.push_back(recibido);
            } else {
                recibido.estado = Estados::Finalizado;
            }
        }

    }

}

fn round(mut pro: Proces) -> Proces{
        println!("{:?}", pro);
        pro.tiempo_restante = pro.tiempo_restante.saturating_sub( Duration::from_millis(500));
        pro.num_veces+=1;
    pro
}

