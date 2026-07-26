use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PetriNet {
    marking: Vec<usize>,
    pre_matrix: Vec<Vec<usize>>,  // pre_matrix[transicion][lugar]
    post_matrix: Vec<Vec<usize>>, // post_matrix[transicion][lugar]
}

impl PetriNet {
    /// Crea una nueva instancia de la Red de Petri.
    pub fn new(initial_marking: Vec<usize>, pre: Vec<Vec<usize>>, post: Vec<Vec<usize>>) -> Self {
        Self {
            marking: initial_marking,
            pre_matrix: pre,
            post_matrix: post,
        }
    }

    /// Devuelve el marcado actual.
    pub fn marking(&self) -> &[usize] {
        &self.marking
    }

    /// Verifica si una transición está habilitada: M(p) >= Pre(t, p) para todo p.
    pub fn is_enabled(&self, transition: usize) -> bool {
        if transition >= self.pre_matrix.len() {
            return false;
        }

        self.pre_matrix[transition]
            .iter()
            .zip(self.marking.iter())
            .all(|(&pre, &m)| m >= pre)
    }

    /// Dispara una transición actualizando el marcado de forma atómica: M'(p) = M(p) - Pre(t, p) + Post(t, p)
    pub fn fire(&mut self, transition: usize) -> Result<(), &'static str> {
        if !self.is_enabled(transition) {
            return Err("La transición no está habilitada");
        }

        for p in 0..self.marking.len() {
            self.marking[p] =
                self.marking[p] - self.pre_matrix[transition][p] + self.post_matrix[transition][p];
        }

        Ok(())
    }

    /// Explora y devuelve todos los marcados alcanzables R(M0) utilizando `is_enabled` y `fire`.
    pub fn reachable_markings(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Insertar el marcado inicial
        visited.insert(self.marking.clone());
        queue.push_back(self.marking.clone());

        let num_transitions = self.pre_matrix.len();

        // Instanciamos la red auxiliar una sola vez fuera del bucle principal
        // para reutilizar las matrices de incidencia sin clonarlas constantemente.
        let mut temp_net = self.clone();

        while let Some(current_marking) = queue.pop_front() {
            result.push(current_marking.clone());

            for t in 0..num_transitions {
                // Reasignamos el marcado actual a evaluar en la red auxiliar
                temp_net.marking = current_marking.clone();

                // Verificamos si la transición t está habilitada
                if temp_net.is_enabled(t) {
                    // Disparamos la transición en la red auxiliar
                    if temp_net.fire(t).is_ok() {
                        let next_marking = temp_net.marking().to_vec();

                        // Si es un estado no visitado, lo registramos
                        if !visited.contains(&next_marking) {
                            visited.insert(next_marking.clone());
                            queue.push_back(next_marking);
                        }
                    }
                }
            }
        }

        result
    }
}
