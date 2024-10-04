use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;
#[allow(dead_code)] // Marca este campo como permitido aunque no se use
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    // Crea un nuevo ThreadPool.
    // El tamaño es la cantidad de hilos en el pool.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Envia un trabajo al ThreadPool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("Sending a job to the thread pool"); // Depuración
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
#[allow(dead_code)] // Marca este campo como permitido aunque no se use
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id); // Depuración
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4); // Verifica que se crean 4 trabajadores
    }

    #[test]
    fn test_thread_pool_execute_single_task() {
        let pool = ThreadPool::new(2);

        // Creamos un canal para verificar que el trabajo se haya ejecutado
        let (tx, rx) = mpsc::channel();
        pool.execute(move || {
            tx.send(42).unwrap(); // Enviamos un valor para verificar la ejecución
        });

        // Verificamos que el trabajo fue ejecutado y recibimos el valor
        let result = rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thread_pool_execute_multiple_tasks() {
        let pool = ThreadPool::new(3);

        let counter = Arc::new(Mutex::new(0));

        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }

        // Esperamos un momento para asegurarnos de que los trabajos terminen
        thread::sleep(Duration::from_millis(100));

        // Verificamos que el contador haya incrementado 5 veces
        let result = *counter.lock().unwrap();
        assert_eq!(result, 5);
    }
}