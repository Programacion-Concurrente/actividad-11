mod petri_net;
use petri_net::PetriNet;

fn main() {
    let initial_marking = vec![1, 0, 1, 1, 0];

    let pre_matrix = vec![
        vec![1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 1],
    ];

    let post_matrix = vec![
        vec![0, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1],
        vec![0, 0, 1, 1, 0],
    ];

    let net = PetriNet::new(initial_marking, pre_matrix, post_matrix);

    // Obtener marcados alcanzables R(M0)
    let reachables = net.reachable_markings();

    println!("--- Marcados Alcanzables ---");
    for (idx, marking) in reachables.iter().enumerate() {
        println!("M{}: {:?}", idx, marking);
    }

    // Verificación de Exclusión Mutua (P1 en SC es M[1], P2 en SC es M[4])
    let violation = reachables.iter().any(|m| m[1] == 1 && m[4] == 1);

    println!("\n--- Verificación de Exclusión Mutua ---");
    if violation {
        println!("❌ VIOLACIÓN: Se encontró un estado con ambos procesos en la Sección Crítica.");
    } else {
        println!(
            "✅ EXCLUSIÓN MUTUA CUMPLIDA: En ningún marcado alcanzable se encuentran ambos procesos simultáneamente en la Sección Crítica."
        );
    }
}