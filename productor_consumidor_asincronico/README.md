# 📦 Productor–Consumidor con Buffer Acotado en Tokio (Rust)

Este proyecto implementa una simulación del patrón **Productor–Consumidor** concurrente utilizando el runtime asincrónico **Tokio** en Rust y un canal acotado (`tokio::sync::mpsc::channel`). Además, analiza el comportamiento del bloqueo asincrónico y su equivalencia teórica en **Redes de Petri**.

---

## 🛠️ Estructura del Proyecto

```text
.
├── Cargo.toml
└── src/
    └── main.rs       # Implementación de las tareas Productora y Consumidora
```

## ⚙️ Descripción del Código

El programa lanza dos tareas asincrónicas (`tokio::spawn`):
* **Productora**: Intenta enviar 5 ítems de manera veloz a través de un canal con capacidad máxima para 2 elementos (`CAPACIDAD = 2`).
* **Consumidora**: Procesa los elementos a un ritmo más lento (con un `sleep` de 1 segundo por cada elemento) para forzar que el buffer se llene y provocar la suspensión de la tarea productora.

---

## ❓ Análisis y Preguntas Teóricas

### 1. ¿En qué momento exacto la tarea Productora queda suspendida asincrónicamente?

> **Respuesta:** La tarea Productora se suspende en el instante exacto en que invoca `tx.send(item).await` y el canal ya contiene un número de elementos igual a `CAPACIDAD` (en este caso, 2 elementos).

En Rust con Tokio, `tx.send()` devuelve una `Future`. Al estar el buffer lleno, no se bloquea el hilo (*thread*) del sistema operativo, sino que se retorna `Poll::Pending`. El punto de await (`.await`) cede el control al *executor* de Tokio, pausando la tarea del productor para que el hilo pueda ejecutar otras tareas.

### 2. ¿Qué acción del Consumidor habilita nuevamente la ejecución del Productor?

> **Respuesta:** La ejecución del Productor se habilita nuevamente cuando el Consumidor invoca la función `rx.recv().await` y remueve exitosamente un ítem del canal.

Al liberar un lugar en el canal, Tokio notifica al *Waker* asociado a la tarea del Productor. Esto cambia su estado de suspendido a **listo para ejecutar** (*ready*), permitiendo que el *executor* reanude el `.await` de `tx.send()`, deposite el nuevo ítem y continúe su ejecución.

### 3. Explicación del comportamiento dinámico mediante el intercambio de tokens en una Red de Petri

> **Respuesta:** En una Red de Petri equivalente para un Buffer Acotado de capacidad $N$, el canal se modela con dos lugares principales:
> * $P_{\text{vacíos}}$: Representa los espacios disponibles en el canal. Marcado inicial: $M_0(P_{\text{vacíos}}) = N$.
> * $P_{\text{llenos}}$: Representa los elementos depositados listos para consumir. Marcado inicial: $M_0(P_{\text{llenos}}) = 0$.

**Dinámica:**
* **Suspensión:** Cada disparo de la transición *Enviar* consume 1 token de $P_{\text{vacíos}}$ y coloca 1 token en $P_{\text{llenos}}$. Cuando se envían $N$ elementos consecutivos, $P_{\text{vacíos}}$ se queda con 0 tokens, deshabilitando la transición *Enviar* ($M(P_{\text{vacíos}}) < 1$) y bloqueando al Productor.
* **Rehabilitación:** Cuando el Consumidor dispara la transición *Recibir*, remueve 1 token de $P_{\text{llenos}}$ y regenera 1 token en $P_{\text{vacíos}}$. Este nuevo token habilita nuevamente la transición *Enviar*, permitiendo que el Productor continúe.

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
[Productor] Intentando enviar ítem 1
[Productor] Ítem 1 enviado con éxito.
[Productor] Intentando enviar ítem 2
[Productor] Ítem 2 enviado con éxito.
[Productor] Intentando enviar ítem 3
  [Consumidor] Ítem 1 consumido.
[Productor] Ítem 3 enviado con éxito.
[Productor] Intentando enviar ítem 4
  [Consumidor] Ítem 2 consumido.
[Productor] Ítem 4 enviado con éxito.
[Productor] Intentando enviar ítem 5
  [Consumidor] Ítem 3 consumido.
[Productor] Ítem 5 enviado con éxito.
  [Consumidor] Ítem 4 consumido.
  [Consumidor] Ítem 5 consumido.
```
