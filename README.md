# 🦾 Prosthetic Claw Simulator

A lightweight physics-based simulation for testing EMG-driven prosthetic control systems.  
Written in **Rust** using the [`rapier2d`](https://rapier.rs/) physics engine, this project models a simple two-pronged claw controlled by EMG (electromyography) input signals.

---

## 🎯 Project Goal

To explore and prototype control systems for EMG-driven prosthetic devices in a 2D environment.  
This simulation is a stepping stone toward embedding EMG-driven logic into real-time embedded systems or prosthetic limbs.

---

## 🛠 How It Works

- **Input**: EMG data (either simulated or real) is loaded from a CSV file.
- **Processing**: A control system converts EMG signals into target joint angles.
- **Simulation**: A 2D claw responds to torque based on the control logic.
- **Output**: The simulation logs motion and outputs new CSV files for analysis or visualization.

---

## 📁 Structure

- `main.rs` – The main simulation loop in Rust.
- `emg_data/` – Contains input and output `.csv` files for EMG signals and claw motion.
- `notebooks/` – Contains Python notebooks for data simulation and visualization (animation, plotting, etc.)

---

## ⚠ Known Issues

- The `create_simulation` animation notebook currently raises an error. This is not critical — it’s used only for visualization and doesn't impact core simulation logic.

---

## ✅ Features

- Uses **Simulated EMG signals**
- Basic **proportional control** for joint actuation
- CSV logging for integration with plotting or ML
- Supports **time-based playback** to simulate real-time EMG input

---

## 📝 TODO

- [ ] Fix the animation notebook
- [ ] Convert notebooks into `.py` scripts for reproducibility
- [ ] Add a `Dockerfile` for cross-platform deployment
- [ ] Add GitHub Actions for CI
- [ ] Add damping to the control logic (PD controller)
- [ ] Investigate additional EMG signal smoothing techniques
- [ ] Add unit testing to the Rust control logic
- [ ] Refactor Rust into modules for better structure and readability

---

## 🚀 Getting Started

1. Make sure you have Rust installed:  
   https://www.rust-lang.org/tools/install

2. Clone the repo:

   ```bash
   git clone https://github.com/VarSamLewis/prosthetic_claw_sim.git
   cd prosthetic_claw_sim
