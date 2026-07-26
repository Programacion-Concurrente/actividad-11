# 🤝 Simulación de Two-Phase Commit (2PC) y Caída del Coordinador en Rust

Este proyecto implementa la simulación del protocolo de consenso distribuido **Two-Phase Commit (2PC)** utilizando el runtime **Tokio**. Se simula un escenario de falla en el cual el Coordinador colapsa (mediante el cierre/droppeo abrupto de un canal `oneshot`), analizando las implicancias teóricas de bloqueo y las propiedades del sistema que se conservan.

---

## 🛠️ Estructura del Proyecto

```text
.
├── Cargo.toml
└── src/
    └── main.rs       # Simulación de las tareas del Coordinador y del Participante
```

## ⚙️ Descripción del Código

El programa simula dos tareas concurrentes (`tokio::spawn`):
* **Participante**: Envía su voto (`Vote::Commit`) al Coordinador e ingresa a la Fase 2 quedando a la espera de la decisión global (`rx_decision`) envuelta en un `timeout`.
* **Coordinador**: Simula recibir los votos y, antes de tomar/enviar la decisión final, sufre un colapso. Esto se modela invocando `drop(tx_decision)`, cerrando el canal emisor sin transmitir ningún mensaje.

---

## ❓ Análisis y Preguntas Teóricas

### 1. ¿En qué estado del ciclo de vida queda detenido el proceso Participante?

> **Respuesta:** El Participante queda detenido en el estado **`PREPARED`** (o *READY* / En Espera de Decisión).

Tras emitir un voto afirmativo (`VOTE_COMMIT`), el Participante adquiere y bloquea los recursos locales (locks, registros de log) quedando a la espera de la decisión unánime. Al detectarse el cierre del canal (`Ok(Err(_))`), el participante reconoce la falla pero permanece atrapado en ese estado de incerteza.

### 2. ¿Por qué la tarea no puede avanzar ni finalizar libremente?

> **Respuesta:** Porque el protocolo 2PC es **estrictamente dependiente de una decisión global unánime**.

Si el Participante tomara una decisión unilateral:
* Si hiciera **Commit** por su cuenta, violaría la consistencia si otro participante votó *Abort*.
* Si hiciera **Abort** por su cuenta, violaría la atomicidad si el Coordinador logró registrar un *Global Commit* antes de caer.

Al perder comunicación con el Coordinador, carece de la información para resolver su estado y se ve forzado a mantener los recursos bloqueados.

### 3. ¿Cómo se representa este estado de bloqueo indefinido en el Grafo de Alcanzabilidad de una Red de Petri?

> **Respuesta:** Se representa mediante un **Bloqueo Mutuo o Estado Muerto (*Deadlock*)**.

En el Grafo de Alcanzabilidad ($R(M_0)$), este estado corresponde a un **nodo terminal** (un marcado $M_k$) en el cual ninguna transición saliente está habilitada ($M_k \xrightarrow{t} \text{ninguna}$). Las fichas quedan atrapadas en los lugares que representan el estado `PREPARED` y el bloqueo de recursos local.

### 4. ¿Qué propiedad fundamental del sistema se preserva a pesar de que el proceso quedó bloqueado?

> **Respuesta:** Se preserva la **Consistencia** (o **Atomicidad / Seguridad / Safety**).

El protocolo 2PC sacrifica la **Disponibilidad** (*Liveness*) en favor de la **Consistencia** (*Safety*). Se prefiere mantener un proceso bloqueado indefinidamente antes que arriesgar un estado inconsistente en los datos del sistema distribuido.

---

## 🚀 Ejecución

Asegurate de incluir la dependencia de Tokio en tu `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Para compilar y correr el proyecto:

```bash
cargo run
```

## 📊 Salida Esperada

```text
[Participante]: Votando COMMIT y esperando decisión ...
[Coordinador] Recibiendo votos de los participantes...
[Coordinador] 💥 ¡EL COORDINADOR HA COLAPSADO!
[Participante] ⚠️ ERROR: El canal del Coordinador se cerró abruptamente.
[Participante] Quedo BLOQUEADO en estado de incerteza.
