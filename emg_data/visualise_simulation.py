import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation


# --- Utilities to find your repo/script directory ---
# This will be the directory containing this .py file
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))


# --- Data I/O & validation ---
def load_data(file_path: str) -> pd.DataFrame:
    if not file_path.endswith('.csv'):
        raise ValueError("The file must be a CSV file.")
    return pd.read_csv(file_path)


def validate_data(df: pd.DataFrame) -> None:
    if df.isnull().values.any():
        raise ValueError("Data contains null values.")
    if not all(df.dtypes == 'float64'):
        raise ValueError("All columns must be of float type.")
    if df.shape[1] != 4:
        raise ValueError("Data must contain exactly 4 columns.")
    required = {'time', 'emg', 'left', 'right'}
    if not required.issubset(df.columns):
        raise ValueError(f"DataFrame must contain columns: {required}")


# --- Static plots ---
def visualise_data(df: pd.DataFrame) -> None:
    fig, axs = plt.subplots(2, 1, figsize=(12, 10), sharex=True)
    fig.subplots_adjust(hspace=0.4)

    axs[0].plot(df['time'], np.degrees(df['left']), color='blue')
    axs[0].set_title('Left Claw Angle')
    axs[0].set_ylabel('Angle (°)')
    axs[0].grid(True)

    axs[1].plot(df['time'], np.degrees(df['right']), color='red')
    axs[1].set_title('Right Claw Angle')
    axs[1].set_xlabel('Time (s)')
    axs[1].set_ylabel('Angle (°)')
    axs[1].grid(True)

    plt.show()


# --- Animation setup ---
def create_figure():
    fig, ax = plt.subplots(figsize=(6, 6))
    ax.set_xlim(-2, 2)
    ax.set_ylim(-2, 2)
    ax.set_aspect('equal')
    ax.grid(True)

    left_claw, = ax.plot([], [], 'b-', linewidth=4, label="Left Claw")
    right_claw, = ax.plot([], [], 'r-', linewidth=4, label="Right Claw")
    ax.legend()

    base = (0, 0)
    claw_length = 1.0
    return fig, ax, left_claw, right_claw, base, claw_length


def init(left_claw, right_claw):
    left_claw.set_data([], [])
    right_claw.set_data([], [])
    return left_claw, right_claw


def update(frame, df, left_claw, right_claw, base, claw_length):
    bx, by = base
    la = df['left'].iloc[frame]
    ra = df['right'].iloc[frame]

    left_claw.set_data(
        [bx, bx + claw_length * np.cos(la)],
        [by, by + claw_length * np.sin(la)]
    )
    right_claw.set_data(
        [bx, bx + claw_length * np.cos(ra)],
        [by, by + claw_length * np.sin(ra)]
    )
    return left_claw, right_claw


# --- Animation runner ---
def run_animation(
    df: pd.DataFrame,
    out_dir: str = None,
    show: bool = True,
    save_filename: str = "claw_animation.mp4"
):
    if out_dir is None:
        out_dir = SCRIPT_DIR
    os.makedirs(out_dir, exist_ok=True)

    fig, ax, left_claw, right_claw, base, claw_length = create_figure()
    ani = animation.FuncAnimation(
        fig,
        update,
        frames=len(df),
        fargs=(df, left_claw, right_claw, base, claw_length),
        init_func=lambda: init(left_claw, right_claw),
        blit=True,
        interval=20
    )

    # save to disk
    save_path = os.path.join(out_dir, save_filename)
    ani.save(save_path, writer='ffmpeg', fps=50)
    print(f"[INFO] Animation saved to {save_path}")

    if show:
        plt.show()

    return ani


if __name__ == "__main__":
    # 1. build the path to your CSV in this script's folder
    data_path = os.path.join(SCRIPT_DIR, "output.csv")

    # 2. load & validate
    df = load_data(data_path)
    validate_data(df)

    # 3. static plots
    visualise_data(df)

    # 4. animate & save into the same folder
    run_animation(df)