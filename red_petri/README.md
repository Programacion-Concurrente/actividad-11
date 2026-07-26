# 🕸️ Red de Petri & Exclusión Mutua (Mutex) en Rust

Este proyecto contiene una implementación genérica en **Rust** de una **Red de Petri** discreta y la utiliza para simular y verificar formalmente la propiedad de **Exclusión Mutua (Mutex)** entre dos procesos concurrentes.

---

## 🛠️ Estructura del Proyecto

```text
.
├── Cargo.toml
└── src/
    ├── main.rs       # Modelado del Mutex y verificación
    └── petri_net.rs  # Estructura e implementación de la Red de Petri
```

## ⚙️ Descripción de Componentes

### 1. Librería (`src/petri_net.rs`)
Implementa la estructura `PetriNet` con las siguientes operaciones clave:

* **`new(initial_marking, pre, post)`**: Inicializa la red con el marcado inicial y las matrices de incidencia de entrada ($Pre$) y salida ($Post$).
* **`is_enabled(transition)`**: Evalúa si la transición $t$ está habilitada verificando la condición de disparo:
  $$M(p) \ge Pre(t, p) \quad \forall p$$
* **`fire(transition)`**: Ejecuta el disparo atómico actualizando el marcado según la ecuación de estado:
  $$M'(p) = M(p) - Pre(t, p) + Post(t, p)$$
* **`reachable_markings()`**: Explora todo el espacio de estados alcanzables $R(M_0)$ mediante un algoritmo de búsqueda en anchura (**BFS**), reutilizando `is_enabled` y `fire`.

### 2. Simulación y Verificación (`src/main.rs`)
Modelado de exclusión mutua para dos procesos ($P_1$ y $P_2$) que compiten por un semáforo Mutex.

#### Modelado de Lugares y Transiciones
* **Lugares (Places):**
  * `[0]` $P_0$: $P_1$ fuera de Sección Crítica.
  * `[1]` $P_1$: $P_1$ en Sección Crítica (**CS1**).
  * `[2]` $P_2$: Semáforo Mutex disponible (1 token).
  * `[3]` $P_3$: $P_2$ fuera de Sección Crítica.
  * `[4]` $P_4$: $P_2$ en Sección Crítica (**CS2**).
* **Marcado Inicial:** $M_0 = [1, 0, 1, 1, 0]$
* **Transiciones:**
  * $T_0$: $P_1$ ingresa a Sección Crítica.
  * $T_1$: $P_1$ sale de Sección Crítica y libera el Mutex.
  * $T_2$: $P_2$ ingresa a Sección Crítica.
  * $T_3$: $P_2$ sale de Sección Crítica y libera el Mutex.

---

## 🚀 Ejecución

Para compilar y correr el proyecto:

```bash
cargo run
```

## 📊 Salida Esperada

Al ejecutar la aplicación, el programa calcula todos los marcados alcanzables $R(M_0)$ e inspecciona si existe algún estado que viole la regla de exclusión mutua ($M[1] == 1 \land M[4] == 1$):

```text
--- Marcados Alcanzables ---
M0: [1, 0, 1, 1, 0]
M1: [0, 1, 0, 1, 0]
M2: [1, 0, 0, 0, 1]
M3: [1, 0, 1, 1, 0]

--- Verificación de Exclusión Mutua ---
✅ EXCLUSIÓN MUTUA CUMPLIDA: En ningún marcado alcanzable se encuentran ambos procesos simultáneamente en la Sección Crítica.
```
