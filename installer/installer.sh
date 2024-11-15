#!/bin/bash

set -e

echo "Instalando Magicsh..."

# Clonar el repositorio
REPO_URL="https://github.com/drxcodev/magicsh.git"
INSTALL_DIR="/tmp/magicsh_install"

echo "Clonando el repositorio desde $REPO_URL..."
git clone "$REPO_URL" "$INSTALL_DIR"

# Ir al directorio del proyecto
cd "$INSTALL_DIR"

# Compilar el proyecto con cargo
echo "Compilando el proyecto..."
cargo build --release

# Mover el binario compilado a /usr/local/bin
BIN_PATH="/usr/local/bin/magicsh"
echo "Instalando el ejecutable en $BIN_PATH..."
sudo mv target/release/magicsh "$BIN_PATH"

# Limpiar archivos temporales
echo "Limpiando archivos temporales..."
rm -rf "$INSTALL_DIR"

echo "¡Magicsh se ha instalado correctamente!"
echo "Ejecuta 'magicsh' para iniciar tu shell personalizada."
