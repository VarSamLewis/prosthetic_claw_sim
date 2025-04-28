import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import poisson
import os 
import pandas as pd

def generate_force_profile(t, period, duty):
     force = ((t % period) < (period * duty)).astype(float)
     return force


def generate_biphasic_muap(muap_params, fs):
    A1, A2, T1, T2, std = muap_params
    # build a short time‐vector around zero
    tm = np.arange(-0.01, 0.02, 1/fs)  
    muap = A1 * np.exp(-((tm - T1)**2)/(2*std**2)) \
         - A2 * np.exp(-((tm - T2)**2)/(2*std**2))
    return tm, muap

def export_to_csv(emg, out_dir=None):
    if out_dir is None:
        out_dir = os.path.dirname(os.path.abspath(__file__))
        os.makedirs(out_dir, exist_ok=True)
    df = pd.DataFrame({'time': t, 'emg': emg})
    save_path = os.path.join(out_dir, "emg.csv")
    df.to_csv(save_path, index=False)
    print(f"[INFO] EMG data saved to {save_path}")


def main():
    # 1. PARAMETERS
    fs = 1000            # sampling rate, Hz
    duration = 10        # seconds
    t = np.arange(0, duration, 1/fs)

    # MUAP shape parameters: (A1, A2, T1, T2, std)
    muap_params = (1.0, 0.5, 0.002, 0.005, 0.001)

    # max firing rate during contraction, Hz
    max_rate = 50.0  

    # opening/closing cycle: period = 2s, 50% duty (1s on, 1s off)
    period = 2.0
    duty = 0.5    

    # 2. FORCE PROFILE (0 when open, 1 when closed)
    force = generate_force_profile(t, period, duty)

    # 3. MUAP TEMPLATE
    tm, muap = generate_biphasic_muap(muap_params, fs)

    # 4. FIRING RATES & SPIKE TRAIN
    # rate at each t = max_rate * force(t)
    rates = max_rate * force

    # spikes per sample = rate[Hz] / fs
    lam = rates / fs
    spikes = poisson.rvs(mu=lam)

    # 5. CONVOLVE SPIKES WITH MUAP
    emg = np.convolve(spikes, muap, mode='full')[:t.size]

    # 6. ADD BASELINE GAUSSIAN NOISE
    noise = 0.1 * np.random.randn(t.size)
    emg_noisy = emg + noise

    # 7. EXPORT TO CSV
    export_to_csv(emg)

    # 7. PLOT
    # plt.figure(figsize=(12, 4))
    # plt.plot(t, emg_noisy, label='Synthetic EMG')
    # plt.xlabel('Time (s)')
    # plt.ylabel('Amplitude (a.u.)')
    # plt.title('Fake EMG: hand closing (bursts) / opening (baseline)')
    # plt.xlim(0, duration)
    # plt.grid(True)
    # plt.tight_layout()
    # plt.savefig('sim_emg.png', dpi=300)
    # plt.show()

