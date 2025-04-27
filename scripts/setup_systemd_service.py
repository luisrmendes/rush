#!/usr/bin/python3.13

import os
import subprocess
import sys


def format_std(text):
    return f"  \033[34m-> \033[0m\033[1m{text}\033[0m"


if os.geteuid() != 0:
    print("This script must be run as sudo!", file=sys.stderr)
    sys.exit(1)

PROGRAM_NAME = "rush"
# BIN_DIR = os.getenv("HOME") + "/bin"
BIN_DIR = "/usr/bin"
PROGRAM_PATH = BIN_DIR + "/" + PROGRAM_NAME
SERVICE_PATH = "/etc/systemd/system/" + PROGRAM_NAME + ".service"

# Build release
print(format_std("Build Release"))
subprocess.run(["cargo", "build", "--release"], check=True)

# Kill PROGRAM_NAME process
print(format_std("Kill " + PROGRAM_NAME))
result = subprocess.run(["pgrep", PROGRAM_NAME], stdout=subprocess.DEVNULL)
if result.returncode == 0:
    subprocess.run(["pkill", PROGRAM_NAME], check=False)

# mkdir if not already present
subprocess.run(["mkdir", "-p", BIN_DIR], check=True)

# Copy binary to user bin
print(format_std("Copy binary to " + BIN_DIR))
subprocess.run(["cp", "../target/release/" + PROGRAM_NAME, BIN_DIR], check=True)

# Add executable permissions
print(format_std("Add executable permissions"))
subprocess.run(["chmod", "+x", PROGRAM_PATH], check=True)

# Check if Systemd service was already installed
if os.path.exists(SERVICE_PATH) is True:
    print("rush.service already exists")
    subprocess.run(["systemctl", "stop", "rush"], check=True)

# Create systemd file
print(format_std("Create systemd file"))

SYSTEMD_FILE_CONTENT = (
    "\
[Unit]\n\
Description=Rush\n\
After=network.target\n\
# Requires=network-online.target  # More robust network check (optional)\n\
\n\
[Service]\n\
User=lrm \n\
WorkingDirectory="
    + BIN_DIR
    + "\n\
EnvironmentFile=/home/lrm/projs/rush/.env\n\
ExecStart="
    + PROGRAM_PATH
    + "\n\
Restart=on-failure\n\
# Type=simple  # Typically 'simple' is fine, but 'forking' is needed if your program forks.\n\
\n\
[Install]\n\
WantedBy=multi-user.target\n"
)

if os.path.exists(SERVICE_PATH):
    os.remove(SERVICE_PATH)

with open(SERVICE_PATH, "w") as file:
    file.write(SYSTEMD_FILE_CONTENT)

# Setup systemd service
print(format_std("Setup Systemd service"))
subprocess.run(["systemctl", "daemon-reload"], check=True)
subprocess.run(["systemctl", "enable", PROGRAM_NAME], check=True)
subprocess.run(["systemctl", "start", PROGRAM_NAME], check=True)

exit(0)
