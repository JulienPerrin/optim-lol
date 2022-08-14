from ortools.linear_solver import pywraplp
from enum import Enum

class ROLE(Enum):
    TOP = 1
    JUNGLE = 2
    MID = 3
    BOTTOM = 4
    SUPPORT = 5

def main(): 
    #joueur 1 veut jouer rôle TOP, joueur 2 rôle BOTTOM, joueur 3 rôle MID, joueur 4 rôle JUNGLE et joueur 5 rôle SUPPORT
    preferenceJoueur1 = Preference(1, 0, 0, 0, 0)
    preferenceJoueur2 = Preference(0, 0, 0, 1, 0)
    preferenceJoueur3 = Preference(0, 0, .5, .5, 0)
    preferenceJoueur4 = Preference(0, .5, .5, 0, 0)
    preferenceJoueur5 = Preference(0, 0, 0, 0, 1)
    data = create_data_model(preferenceJoueur1, preferenceJoueur2, preferenceJoueur3, preferenceJoueur4, preferenceJoueur5)

    # Create the mip solver with the SCIP backend.
    solver = pywraplp.Solver.CreateSolver('SCIP')
    if not solver:
        raise Error("pas glop")

    x = {}
    for j in range(data['num_vars']):
        x[j] = solver.IntVar(0, 1, 'x[%i]' % j)
    print('Number of variables =', solver.NumVariables())

    for i in range(data['num_constraints']):
        constraint_expr = [data['constraint_coeffs'][i][j] * x[j] for j in range(data['num_vars'])]
        solver.Add(sum(constraint_expr) <= data['bounds'][i])

    obj_expr = [data['obj_coeffs'][j] * x[j] for j in range(data['num_vars'])]
    solver.Maximize(solver.Sum(obj_expr))

    status = solver.Solve()

    if status == pywraplp.Solver.OPTIMAL:
        print('Objective value =', solver.Objective().Value())
        for j in range(data['num_vars']):
            if (x[j].solution_value() == 1):
                print("Le joueur {} joue le rôle {} ({})".format(j // 5 + 1, ROLE(j % 5 + 1).name, j % 5 + 1))
        print()
        print('Problem solved in %f milliseconds' % solver.wall_time())
        print('Problem solved in %d iterations' % solver.iterations())
        print('Problem solved in %d branch-and-bound nodes' % solver.nodes())
    else:
        print('The problem does not have an optimal solution.')

class Preference:
  def __init__(self, satisfactionTop, satisfactionJungle, satisfactionMid, satisafactionBottom, satisfactionSupport):
    self.satisfactionTop = satisfactionTop
    self.satisfactionJungle = satisfactionJungle
    self.satisfactionMid = satisfactionMid
    self.satisafactionBottom = satisafactionBottom
    self.satisfactionSupport = satisfactionSupport

  def satisfaction_array(self):
    return [self.satisfactionTop, self.satisfactionJungle, self.satisfactionMid, self.satisafactionBottom, self.satisfactionSupport]

def create_data_model(preferenceJoueur1: Preference,
        preferenceJoueur2: Preference,
        preferenceJoueur3: Preference,
        preferenceJoueur4: Preference,
        preferenceJoueur5: Preference):
    """Stores the data for the problem."""
    data = {}
    data['constraint_coeffs'] = [
        # data[i, j] = joueur i rôle j
        # 1 rôle par joueur
        [1, 1, 1, 1, 1,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  ],
        [0, 0, 0, 0, 0,  1, 1, 1, 1, 1,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  ],
        [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  1, 1, 1, 1, 1,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  ],
        [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  1, 1, 1, 1, 1,  0, 0, 0, 0, 0,  ],
        [0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  0, 0, 0, 0, 0,  1, 1, 1, 1, 1,  ],
        
        # 1 joueur par rôle
        [1, 0, 0, 0, 0,  1, 0, 0, 0, 0,  1, 0, 0, 0, 0,  1, 0, 0, 0, 0,  1, 0, 0, 0, 0,  ],
        [0, 1, 0, 0, 0,  0, 1, 0, 0, 0,  0, 1, 0, 0, 0,  0, 1, 0, 0, 0,  0, 1, 0, 0, 0,  ],
        [0, 0, 1, 0, 0,  0, 0, 1, 0, 0,  0, 0, 1, 0, 0,  0, 0, 1, 0, 0,  0, 0, 1, 0, 0,  ],
        [0, 0, 0, 1, 0,  0, 0, 0, 1, 0,  0, 0, 0, 1, 0,  0, 0, 0, 1, 0,  0, 0, 0, 1, 0,  ],
        [0, 0, 0, 0, 1,  0, 0, 0, 0, 1,  0, 0, 0, 0, 1,  0, 0, 0, 0, 1,  0, 0, 0, 0, 1,  ],
    ]
    data['bounds'] = [1, 1, 1, 1, 1,  1, 1, 1, 1, 1]
    data['obj_coeffs'] = preferenceJoueur1.satisfaction_array() + preferenceJoueur2.satisfaction_array() + preferenceJoueur3.satisfaction_array() + preferenceJoueur4.satisfaction_array() + preferenceJoueur5.satisfaction_array() 
    data['num_vars'] = 25
    data['num_constraints'] = 10
    return data


#### Main ####
main()
