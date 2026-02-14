use crate::wscom::{life_t, t1, wscom_main};


mod wscom {
    pub mod wscom_main;
    pub mod t1;
    pub mod life_t;
}



fn main() {
    //t1::ma();
    //wscom_main::wsmutex();
    //wscom_main::wsrwlock();
    life_t::main_t()
}


