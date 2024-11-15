#!/bin/bash
# Usar ls con colores y resaltar archivos .txt en verde
ls --color=auto "$@" | GREP_COLORS='mt=01;31' grep --color=always '\.txt$' || true