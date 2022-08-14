# Optimisation du choix des rôles dans LOL

Ce projet contient un algorithme d'optimisation combinatoire qui détermine quel joueur doit jouer quel rôle à LOL. 

## Installation

Cloner le projet. 
Avoir Python3 installé : https://www.python.org/downloads/

Installer oortools : `python -m pip install --upgrade --user ortools`

Lancer la commande : `python3 test.py` à la racine du projet pour exécuter l'exemple

## Principe

5 joueurs
5 rôles
1 rôle chacun
1 rôle par joueur

préférence de chaque joueur : 2 ou 3 rôles
fonction à écrire qui calcule la satisfaction du joueur en fonction d'un des 5 rôles par rapport à la position du joueur dans le triangles
sij = satisfaction du jour i pour le rôle j


xij = le joueur i joue le rôle j
xij appartient à {0;1}

1 rôle chacun : 
x11 + x21 + x31 + x41 + x51 = 1
x12 + x22 + x32 + x42 + x52 = 1
x13 + x23 + x33 + x43 + x53 = 1
x14 + x24 + x34 + x44 + x54 = 1
x15 + x25 + x35 + x45 + x55 = 1

1 rôle par joueur : 
x11 + x12 + x13 + x14 + x15 = 1
x21 + x22 + x23 + x24 + x25 = 1
x31 + x32 + x33 + x34 + x35 = 1
x41 + x42 + x43 + x44 + x45 = 1
x51 + x52 + x53 + x54 + x55 = 1

calcul de la satisfaction : 
z = somme pour i et j des sij 
on cherche à maximiser z

