"""
Requires a python venv in '.venv'.
Sets up whisper and its dependencies.
Ensures that pytorch recognizes CUDA
"""

import subprocess
import sys
import torch

# Create virtual environment
subprocess.run([sys.executable, "-m", "venv", ".venv"], check=True)

# Define paths
VENV_PYTHON = ".venv/bin/python"
VENV_PIP = ".venv/bin/pip"

# Upgrade pip
subprocess.run([VENV_PYTHON, "-m", "pip", "install", "--upgrade", "pip"], check=True)

# Install packages
subprocess.run([VENV_PIP, "install", "-U", "openai-whisper"], check=True)
subprocess.run(
    [
        VENV_PIP,
        "install",
        "--pre",
        "torch",
        "torchvision",
        "torchaudio",
        "--index-url",
        "https://download.pytorch.org/whl/nightly/cu124",
    ],
    check=True,
)

# Test if torch recognizes CUDA
print("Torch version:", torch.__version__)
print("torch.CUDA version:", torch.version.cuda)
print("CUDA available:", torch.cuda.is_available())
if torch.cuda.is_available():
    print("CUDA device:", torch.cuda.get_device_name(torch.cuda.current_device()))

if torch.cuda.get_device_name(torch.cuda.current_device()) != "NVIDIA GeForce RTX 3060":
    print("Something's wrong with CUDA in Torch!")
    exit(1)
else:
    print("\nAll good to go!")
    exit(0)
