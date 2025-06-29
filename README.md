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

---

# Next Project Aims for Prosthetic Claw Simulator

---

### 1. Obtain and Process More Realistic EMG Signals  
*Category: Data Collection & Signal Processing*  
- Source higher-fidelity real EMG datasets or collect your own using reliable EMG sensors.  
- Experiment with advanced preprocessing (filtering, noise reduction, normalization).  
- Explore signal augmentation or synthesis techniques to expand training data diversity.

---

### 2. Enhance and Expand the Simulation Environment  
*Category: Software Development & Simulation*  
- Upgrade the physics simulation (e.g., move toward 3D or add compliance and damping effects).  
- Incorporate more detailed muscle-to-motor models for realistic actuation dynamics.  
- Integrate with ROS or Gazebo for standardized robotics simulation workflows.

---

### 3. Refine Control Algorithms and Performance Metrics  
*Category: Control Systems & Evaluation*  
- Implement and compare more advanced control methods (PID, adaptive control, ML-based).  
- Define and build an evaluation suite with quantitative metrics for accuracy, responsiveness, stability, and safety.  
- Add safety constraints like maximum force/torque limits and failure handling logic.

---

### 4. Hardware Integration and Embedded Control Implementation  
*Category: Embedded Systems & Prototyping*  
- Port control logic to embedded hardware (e.g., microcontrollers like STM32 or Raspberry Pi).  
- Interface live EMG sensors with hardware ADC inputs.  
- Develop real-time motor control outputs (PWM, DAC) to drive a physical claw.

---

### 5. Develop Physical Claw Design and Manufacturing Plan  
*Category: Mechanical Design & Prototyping*  
- Create CAD models of the claw mechanism with ergonomic and manufacturability considerations.  
- Select appropriate actuators and sensors to match control requirements.  
- Explore manufacturing methods (3D printing, CNC machining) and material selection.

---

## 🚀 Getting Started

1. Make sure you have Rust installed:  
   https://www.rust-lang.org/tools/install

2. Clone the repo:

   ```bash
   git clone https://github.com/VarSamLewis/prosthetic_claw_sim.git
   cd prosthetic_claw_sim
