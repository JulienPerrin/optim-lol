# Optimisation du choix des rôles dans LOL

Ce projet contient un algorithme d'optimisation combinatoire qui détermine quel joueur doit jouer quel rôle à LOL. 

Le programme existe en deux versions, une version en Python, et une autre version en RUST. 

## Lancement du programme

### En Python

Cloner le projet. 
Avoir Python3 installé : https://www.python.org/downloads/

Installer ortools : `python -m pip install --upgrade --user ortools`

Lancer la commande : `python3 test.py` à la racine du projet pour exécuter l'exemple

Résultat :
```
$ python3 test.py 
Number of variables = 25
Objective value = 4.0
Le joueur 1 joue le rôle TOP (1)
Le joueur 2 joue le rôle BOTTOM (4)
Le joueur 3 joue le rôle MID (3)
Le joueur 4 joue le rôle JUNGLE (2)
Le joueur 5 joue le rôle SUPPORT (5)

Problem solved in 2.000000 milliseconds
Problem solved in 0 iterations
Problem solved in 0 branch-and-bound nodes
```

### En RUST

1. Avoir RUST installé sur son PC. 
2. Lancer la commande `cargo run`

Résultat : 
```
$ cargo run
...
Result - Optimal solution found

Objective value:                5.00000000
Enumerated nodes:               0
Total iterations:               0
Time (CPU seconds):             0.00
Time (Wallclock seconds):       0.00

Total time (CPU seconds):       0.00   (Wallclock seconds):       0.00

Status Finished
obj_value 5.0
Le joueur 1 joue le rôle TOP (1)
Le joueur 2 joue le rôle JUNGLE (2)
Le joueur 3 joue le rôle MID (3)
Le joueur 4 joue le rôle BOTTOM (4)
Le joueur 5 joue le rôle SUPPORT (5)
```

## Principe

5 joueurs
5 rôles
1 rôle chacun
1 rôle par joueur

préférence de chaque joueur : 2 ou 3 rôles
fonction à écrire qui calcule la préférence du joueur pour chaque  rôle + IHM
```
sij = satisfaction du joueur i pour le rôle j
```


```
xij = le joueur i joue le rôle j
xij appartient à {0;1}
```

1 rôle chacun : 
```
x11 + x21 + x31 + x41 + x51 = 1
x12 + x22 + x32 + x42 + x52 = 1
x13 + x23 + x33 + x43 + x53 = 1
x14 + x24 + x34 + x44 + x54 = 1
x15 + x25 + x35 + x45 + x55 = 1
```

1 rôle par joueur : 
```
x11 + x12 + x13 + x14 + x15 = 1
x21 + x22 + x23 + x24 + x25 = 1
x31 + x32 + x33 + x34 + x35 = 1
x41 + x42 + x43 + x44 + x45 = 1
x51 + x52 + x53 + x54 + x55 = 1
```

calcul de la satisfaction : 
```
z = somme pour i et j des xij*sij 
```
on cherche à maximiser z

