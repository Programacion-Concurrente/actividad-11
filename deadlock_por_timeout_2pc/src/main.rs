use tokio::sync::oneshot;
use tokio::time::{timeout, Duration};

#[derive(Debug, PartialEq)]
enum Vote {
    Commit,
    Abort,
}

#[derive(Debug, PartialEq)]
enum Decision {
    GlobalCommit,
    GlobalAbort,
}

#[tokio::main]
async fn main() {
    // Canal para que el Participante envíe su voto al Coordinador
    let (tx_voto, _rx_voto) = oneshot::channel::<Vote>();

    // Canal para que el Coordinador envíe la decisión final al Participante
    let (tx_decision, rx_decision) = oneshot::channel::<Decision>();

    // ------------------------------------------------------------------
    // Tarea: Participante
    // ------------------------------------------------------------------
    let participant_task = tokio::spawn(async move {
        println!("[Participante]: Votando COMMIT y esperando decisión ...");

        // El participante vota Commit
        let _ = tx_voto.send(Vote::Commit);

        // El participante se queda esperando la decisión final del Coordinador.
        // Se aplica un timeout o se espera a que el canal reciba datos.
        match timeout(Duration::from_secs(3), rx_decision).await {
            Ok(Ok(decision)) => {
                println!("[Participante] Decisión recibida: {:?}", decision);
            }
            Ok(Err(_)) => {
                // El tx_decision fue droppeado (el Coordinador se cayó)
                println!(
                    "[Participante] ⚠️ ERROR: El canal del Coordinador se cerró abruptamente."
                );
                println!(
                    "[Participante] Quedo BLOQUEADO en estado de incerteza."
                );
            }
            Err(_) => {
                println!("[Participante] ⏱️ TIMEOUT: No se recibió respuesta del Coordinador.");
            }
        }
    });

    // ------------------------------------------------------------------
    // Tarea: Coordinador (Simulación de Caída)
    // ------------------------------------------------------------------
    let coordinator_task = tokio::spawn(async move {
        println!("[Coordinador] Recibiendo votos de los participantes...");
        
        // SIMULACIÓN DE CAÍDA:
        // El Coordinador se cae. Forzamos el droppeo de tx_decision sin enviar la decisión.
        drop(tx_decision); 
        println!("[Coordinador] 💥 ¡EL COORDINADOR HA COLAPSADO!");
    });

    let _ = tokio::join!(participant_task, coordinator_task);
}