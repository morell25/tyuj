use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

pub fn wsmutex() {
    let cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let tareas: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = vec![];
    let urls = vec![
        "https://google.com",
        "https://rust-lang.org",
        "https://github.com",
        "https://google.com", // Repetida: Debería leer de caché
        "https://stackoverflow.com",
        "https://rust-lang.org", // Repetida: Debería leer de caché
        "https://wikipedia.org",
        "https://reddit.com",
        "https://github.com", // Repetida: Debería leer de caché
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
                let mut cola = tareas_h.lock().unwrap();
                if let Some(tarea) = cola.pop_back() {
                    drop(cola);

                    //para tarea cacheada, es decir, si tengo dos veces github no descargalar dos veces
                    {
                        let cache_l = cache_h.lock().unwrap();
                        if cache_l.contains_key(&tarea) {
                            println!("tarea cacheada {}", tarea);
                            continue;
                        }
                    }

                    let content_web = thread::sleep(Duration::from_millis(500));
                    println!("a descargar la tarea {}", { &tarea });
                    cache_h
                        .lock()
                        .unwrap()
                        .insert(tarea, format!("content html {:?}", content_web));
                } else {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}


pub fn wsrwlock() {
    let cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    let tareas: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = vec![];
    let urls = vec![
        "https://google.com",
        "https://rust-lang.org",
        "https://github.com",
        "https://google.com", // Repetida: Debería leer de caché
        "https://stackoverflow.com",
        "https://rust-lang.org", // Repetida: Debería leer de caché
        "https://wikipedia.org",
        "https://reddit.com",
        "https://github.com", // Repetida: Debería leer de caché
        "https://amazon.com",
    ];

    {
        let mut tareas_u = tareas.lock().unwrap();
        let mut url_unicas = HashSet::new();
        for x in urls {
            if url_unicas.insert(x) {
                tareas_u.push_back(x.to_string());
            }
        }
    }

    for _ in 0..4 {
        let tareas_h = tareas.clone();
        let cache_h = cache.clone();
        let handle = thread::spawn(move || {
            loop {
                let mut cola = tareas_h.lock().unwrap();
                if let Some(tarea) = cola.pop_back() {
                    drop(cola);

                    //para tarea cacheada, es decir, si tengo dos veces github no descargalar dos veces
                    {
                        let cache_l = cache_h.read().unwrap();
                        if cache_l.contains_key(&tarea) {
                            println!("tarea cacheada {}", tarea);
                            continue;
                        }
                    }

                    let content_web = thread::sleep(Duration::from_millis(500));
                    println!("a descargar la tarea {}", { &tarea });
                    cache_h
                        .write()
                        .unwrap()
                        .insert(tarea, format!("content html {:?}", content_web));
                } else {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
