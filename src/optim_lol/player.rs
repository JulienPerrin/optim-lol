use std::collections::hash_map::HashMap;

pub struct Player {
    /** to each role is assigned a level of satisfaction between 0 and 1 */
    pub role_preference: HashMap<u8, f64>,
}
