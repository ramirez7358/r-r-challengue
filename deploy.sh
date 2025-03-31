#!/bin/bash

# Exit on error
set -e

echo "Updating system..."
sudo apt update -y

echo "Installing necessary packages..."
sudo apt install -y curl gcc libpq-dev docker.io docker-compose

echo "Installing Rust..."
if ! command -v cargo &> /dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
else
  echo "Rust is already installed."
fi

echo "Cloning the repository..."
if [ ! -d "r-r-challengue" ]; then
  git clone https://github.com/ramirez7358/r-r-challengue.git
else
  echo "Repository already exists. Pulling latest changes..."
  cd r-r-challengue
  git pull origin main
fi

cd r-r-challengue

echo "Copying environment file..."
cp -n env/env_example.json env/env.json || echo "env.json already exists, skipping copy."

echo "Building Docker containers..."
docker-compose build

echo "Starting all services using Docker Compose..."
docker-compose up -d

echo "Application is now running. Access it at http://<your-ec2-public-ip>:8080"
