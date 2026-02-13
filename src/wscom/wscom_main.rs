use core::time;
use std::{collections::{HashMap, VecDeque}, fmt::format, sync::{Arc, Mutex}, thread, time::Duration};

pub fn wscom(){
    let cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let tareas:Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = vec![];
    let urls = vec![
    "https://google.com",
    "https://rust-lang.org",
    "https://github.com",
    "https://google.com",      // Repetida: Debería leer de caché
    "https://stackoverflow.com",
    "https://rust-lang.org",   // Repetida: Debería leer de caché
    "https://wikipedia.org",
    "https://reddit.com",
    "https://github.com",      // Repetida: Debería leer de caché
    "https://amazon.com",
];

    {
        let mut tareas_u = tareas.lock().unwrap();
        for x in urls {
            tareas_u.push_back(x.to_string());
        }
    }


    for _ in 0..4 {
        let tareas_h = tareas.clone();
        let cache_h = cache.clone();
        let handle = thread::spawn(move || {
            loop {
                if let Some(tarea) = tareas_h.lock().unwrap().pop_back() {
                drop(tarea);
                let content_web = thread::sleep(Duration::from_millis(500));
                cache_h.lock().unwrap().insert("hola".to_string(), format!("content html {:?}", content_web));
            }
        }
        });
    handles.push(handle);
    };


for h in handles {
    h.join().unwrap();
}

}
