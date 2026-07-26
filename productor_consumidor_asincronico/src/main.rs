use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

const CAPACIDAD: usize = 2; // Buffer acotado a 2 elementos

#[tokio::main]
async fn main() {
    // Canal acotado con capacidad máxima
    let (tx, mut rx) = mpsc::channel(CAPACIDAD);

    // Tarea Productora
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            println!("[Productor] Intentando enviar ítem {}", i);
            
            // tx.send() es async: si el canal está lleno, se suspende el `.await`
            tx.send(i).await.unwrap();
            
            println!("[Productor] Ítem {} enviado con éxito.", i);
        }
    });

    // Tarea Consumidora
    let consumer = tokio::spawn(async move {
        // Simulamos un consumidor más lento para forzar el llenado del canal
        for _ in 1..=5 {
            sleep(Duration::from_millis(1000)).await;
            
            if let Some(item) = rx.recv().await {
                println!("  [Consumidor] Ítem {} consumido.", item);
            }
        }
    });

    let _ = tokio::join!(producer, consumer);
}