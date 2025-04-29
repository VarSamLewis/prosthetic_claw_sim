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

- `main.rs` – Main simulation loop. Handles physics, time stepping, and logging.
- `src/utils/`
  - `csv.rs` – Contains `EMGReader` for reading EMG signals and a CSV writer.
  - `control.rs` – Contains the control system logic (proportional + deadzone).
  - `mod.rs` – Re-exports modules for easier use.
- `emg_data/` – Input/output `.csv` files for EMG signals and claw movement.
- `notebooks/` – Python notebooks for generating EMG data and visualizing outputs.

---

## ✅ Features

- Uses **simulated EMG signals**
- Supports **proportional control** with deadzone
- Time-stepped **physics simulation** (via `rapier2d`)
- Exports results for plotting or model training
- Modular architecture — easy to swap or test different control systems
- Has a workflow that auto checks cargo build and cargo test on all pushes and pulls to main

---

## ⚠ Known Issues

- The `create_simulation.ipynb` animation notebook currently raises an error. This is not critical — it is for visualization only and does not impact the simulation logic.

---

## 📝 TODO

- [ ] Fix the animation notebook
- [ ] Convert notebooks into `.py` scripts for reproducibility
- [ ] Add a `Dockerfile` for cross-platform deployment
- [ ] Add GitHub Actions for CI
- [ ] Add damping to the control logic (e.g., PD controller)
- [ ] Investigate additional EMG signal smoothing techniques
- [ ] Add unit testing to the Rust control logic
- [ ] Finalize modularization: separate out `csv` and `control` utils for swapping/testing multiple controllers

---

## 🚀 Getting Started

1. Make sure you have Rust installed:  
   https://www.rust-lang.org/tools/install

2. Clone the repo:

   ```bash
   git clone https://github.com/VarSamLewis/prosthetic_claw_sim.git
   cd prosthetic_claw_sim
