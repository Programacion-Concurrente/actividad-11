# 🕸️ Actividad 11: Redes de Petri

Este repositorio contiene la resolución de la **Actividad 11**, enfocada en el modelado formal, análisis y simulación de sistemas concurrentes mediante **Redes de Petri**, su relación con mecanismos de sincronización en **Rust** (Tokio, Mutex, Productor–Consumidor) y la verificación de propiedades como **Safety** y **Liveness**.

---

## 📑 Índice
1. [Integrantes](#-integrantes)
2. [Descripción del Proyecto](#-descripción-del-proyecto)
3. [Conceptos Teóricos Analizados](#-conceptos-teóricos-analizados)
4. [Estructura del Repositorio](#-estructura-del-repositorio)
5. [Instalación y Ejecución](#-instalación-y-ejecución)

---

## 👥 Integrantes
| Alumno | Legajo | Mail |
| :--- | :--- | :--- |
| Sebastian Brizuela | 105288 | sbrizuela@fi.uba.ar |
| Raquel Ana Dávila | 112002 | radavila@fi.uba.ar |
| Lucas Facundo Couttulenc | 109726 | lcouttulenc@fi.uba.ar |
| Joel Isaac Fernandez Fox | 104424 | jfernandezf@fi.uba.ar |
| Luciano Costa | 102104 | luccosta@fi.uba.ar |

---

## 📋 Descripción del Proyecto
El objetivo de esta actividad es estudiar el modelado formal de sistemas concurrentes a través de **Redes de Petri** y su correspondencia con patrones de sincronización reales. El trabajo práctico se divide en dos grandes enfoques:

1. **Resolución Teórica y Análisis Formal:** Estudio sobre la necesidad de modelos matemáticos formales frente al testing empírico, grafos de alcanzabilidad $R(M_0)$, ecuaciones de estado, propiedades de *Safety* / *Liveness* y la selección de herramientas de verificación adecuada según el escenario.
2. **Implementación y Simulación en Rust:** Desarrollo de tres proyectos en Rust que modelan la simulación de una Red de Petri con análisis de exclusión mutua (`red_petri`), la suspensión y reactivación de tareas en un buffer acotado (`productor_consumidor_asincronico`) y el comportamiento de un participante bloqueado por la caída del coordinador en un protocolo 2PC (`dadlock_por_timeout_2pc`).

---

## 💡 Conceptos Teóricos Analizados

### ¿Por qué Redes de Petri?
A diferencia del testing empírico (el cual no puede garantizar la ausencia de errores debido al no-determinismo en el *interleaving* de hilos), las Redes de Petri proveen un marco matemático para explorar formalmente todo el espacio de estados alcanzables $R(M_0)$ y verificar propiedades críticas del sistema.

### Safety vs. Liveness
* **Safety ("Nada malo ocurre"):** Garantiza que el sistema nunca entrará en un estado inconsistente o no deseado (ej. dos procesos simultáneamente en Sección Crítica).
* **Liveness ("Algo bueno eventualmente ocurre"):** Garantiza que el sistema avanzará hacia su finalización sin quedar atrapado en *deadlocks* o bloqueos indefinidos.

### Modelado de Patrones Clásicos
* **Exclusión Mutua (Mutex):** Control de acceso a la sección crítica a través de tokens que representan permisos disponibles.
* **Productor–Consumidor:** Control de capacidad máxima mediante lugares de fichas libres ($P_{\text{vacíos}}$) y elementos listos para consumir ($P_{\text{llenos}}$).
* **Protocolos Distribuidos (2PC):** Representación formal de estados de incerteza y bloqueos indefinidos (*deadlocks*) ante fallas del coordinador.

---

## 📂 Estructura del Repositorio

El repositorio se organiza dividiendo los proyectos de simulación en Rust y la documentación del informe:

* **`red_petri/`**: Implementación en Rust de una estructura de Red de Petri discreta ($Pre$, $Post$, $M_0$) que explora mediante BFS los marcados alcanzables y verifica la exclusión mutua de dos procesos.
* **`productor_consumidor_asincronico/`**: Simulación en Rust con **Tokio** de un canal acotado MPSC, demostrando la suspensión del productor vía `.await` y su reanudación tras la lectura del consumidor.
* **`dadlock_por_timeout_2pc/`**: Simulación del colapso del coordinador en un protocolo 2PC (mediante `drop` de un canal `oneshot`) y el estado de incerteza en el que queda atascado el participante en `PREPARED`.

---

## 🚀 Instalación y Ejecución

**Clonar el repositorio:**
```bash
git clone git@github.com:Programacion-Concurrente/actividad-11.git
cd actividad-11
```

Ejecutar los diferentes módulos del proyecto:

```bash
# 1. Simulación de Red de Petri & Exclusión Mutua
cargo run --manifest-path red_petri/Cargo.toml

# 2. Simulación de Productor-Consumidor Asincrónico con Tokio
cargo run --manifest-path productor_consumidor_asincronico/Cargo.toml

# 3. Simulación de Bloqueo por Caída del Coordinador en 2PC
cargo run --manifest-path deadlock_por_timeout_2pc/Cargo.toml
```
