#!/bin/bash

# Chemin vers le script Perl (ajuster si nécessaire)
PERL_SCRIPT="./script.pl"
# PERL_SCRIPT="./script_gpt.pl"

# Fichier à analyser (passé en argument)
# INPUT_FILE="./lib.rs"
INPUT_FILE="./.env"

# Pattern regex avec groupe de capture (passé en argument)
# PATTERN="impl(?:\s*)ServicesName.*as_str.*match(?:\s*)self(?:\s*)\{(?:\s*)\n(.*?)(?:\s*)\}"
PATTERN="impl(?:\s*)ServicesName.*as_str.*match(?:\s*)self(?:\s*)\{(?:\s*)\n(.*?)(?:\s*)\}"

if [ ! -f "$INPUT_FILE" ]; then
    echo "Erreur: Fichier $INPUT_FILE introuvable" >&2
    exit 1
fi

# Appel du script Perl (version optimisée qui lit directement le fichier)
"$PERL_SCRIPT" "$INPUT_FILE" "$PATTERN"