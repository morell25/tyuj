use crate::{wscom::{ej1_round_robin, ej2_procesador_log, ej3_procesador_log_multihilo, ej4_votos_distrubidos, ej5_analizador_sensores, ej6_crack_hash, wscom_main}, yiiiha::{p1_generador, p2_procesador}};


mod wscom {
    pub mod wscom_main;
    pub mod ej1_round_robin;
    pub mod ej2_procesador_log;
    pub mod ej3_procesador_log_multihilo;
    pub mod ej4_votos_distrubidos;
    pub mod ej5_analizador_sensores;
    pub mod ej6_crack_hash;
}

mod yiiiha {
    pub mod p1_generador;
    pub mod p2_procesador;
}


fn main() {
    //wscom_main::wsmutex();
    //wscom_main::wsrwlock();

    //ej1_round_robin::main();
    //ej2_procesador_log::main();
    //ej3_procesador_log_multihilo::main();
    //ej4_votos_distrubidos::main();
    //ej5_analizador_sensores::main();
    //ej6_crack_hash::main();
    
    //parte yiiiiha
    //let _ = p1_generador::generar_fichero_logs("./src/yiiiha/hola.txt", 10_000_000);
    let _ = p2_procesador::main();



}


