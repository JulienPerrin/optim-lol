extern crate alloc;

mod optim_lol;
mod utils;

use crate::optim_lol::player::Player;
use crate::optim_lol::role::Role;
use crate::utils::pair::Table;

use crate::Role::*;
use alloc::vec::Vec;
use coin_cbc::{raw::Status, Model, Row, Sense};
use std::collections::HashMap;

const NB_ROLES: usize = Role::nombre_roles();

fn main() {
    let mut player1 = Player {
        role_preference: HashMap::new(),
    };
    player1.role_preference.insert(TOP as u8, 1.);

    let mut player2 = Player {
        role_preference: HashMap::new(),
    };
    player2.role_preference.insert(JUNGLE as u8, 1.);

    let mut player3 = Player {
        role_preference: HashMap::new(),
    };
    player3.role_preference.insert(MID as u8, 1.);

    let mut player4 = Player {
        role_preference: HashMap::new(),
    };
    player4.role_preference.insert(BOTTOM as u8, 1.);

    let mut player5 = Player {
        role_preference: HashMap::new(),
    };
    player5.role_preference.insert(SUPPORT as u8, 1.);

    let mut players = Vec::new();
    players.push(player1);
    players.push(player2);
    players.push(player3);
    players.push(player4);
    players.push(player5);

    // Create the problem.
    let mut m = Model::default();

    let mut binaries: Table<u8, u8> = Table::new(); // = HashMap::with_capacity(25);
    for (i, _player) in players.iter().enumerate() {
        for j in 1..NB_ROLES + 1 {
            binaries.set(i as u8, j as u8, m.add_binary());
        }
    }

    // 1 rôle par joueur
    for (i, _player) in players.iter().enumerate() {
        let row: Row = m.add_row();
        for j in 1..NB_ROLES + 1 {
            let binary = binaries.get(&(i as u8), &(j as u8));
            m.set_weight(row, binary, 1.0);
        }
        m.set_row_equal(row, 1.0);
    }

    // 1 joueur par rôle
    for j in 1..NB_ROLES + 1 {
        let row: Row = m.add_row();
        for (i, _player) in players.iter().enumerate() {
            let binary = binaries.get(&(i as u8), &(j as u8));
            m.set_weight(row, binary, 1.0);
        }
        m.set_row_equal(row, 1.0);
    }

    for (i, player) in players.iter().enumerate() {
        for j in 1..NB_ROLES + 1 {
            let binary = binaries.get(&(i as u8), &(j as u8));
            let satisfaction_of_player_for_role = player.role_preference.get(&(j as u8));
            match satisfaction_of_player_for_role {
                Some(x) => m.set_obj_coeff(binary, *x),
                None => m.set_obj_coeff(binary, 0 as f64),
            }
        }
    }

    m.set_obj_sense(Sense::Maximize);

    // Solve the problem. Returns the solution
    let sol = m.solve();

    // Check the result. sol.raw() returns a shared reference to the
    // raw bindings, allowing to use all getters.
    assert_eq!(Status::Finished, sol.raw().status());
    println!("Status {:#?}", sol.raw().status());
    println!("obj_value {:#?}", sol.raw().obj_value());

    for (i, _player) in players.iter().enumerate() {
        for j in 1..NB_ROLES + 1 {
            if sol.col(binaries.get(&(i as u8), &(j as u8))) == 1 as f64 {
                println!(
                    "Le joueur {:#?} joue le rôle {:#?} ({})",
                    &(i as u8) + 1,
                    Role::from_u8(j as u8),
                    j as u8
                );
            }
        }
    }

    println!("\n");
}
