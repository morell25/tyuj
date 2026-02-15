use crate::wscom::{ej6_crack_hash, ej5_analizador_sensores, ej2_procesador_log, ej3_procesador_log_multihilo, ej1_round_robin, ej4_votos_distrubidos, wscom_main};


mod wscom {
    pub mod wscom_main;
    pub mod ej1_round_robin;
    pub mod ej2_procesador_log;
    pub mod ej3_procesador_log_multihilo;
    pub mod ej4_votos_distrubidos;
    pub mod ej5_analizador_sensores;
    pub mod ej6_crack_hash;
}



fn main() {
    wscom_main::wsmutex();
    wscom_main::wsrwlock();

    ej1_round_robin::main();
    ej2_procesador_log::main();
    ej3_procesador_log_multihilo::main();
    ej4_votos_distrubidos::main();
    ej5_analizador_sensores::main();
    ej6_crack_hash::main();


}


