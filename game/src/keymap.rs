
//
//
// Currently not in use.
//
//

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    FORWARD,
    BACKWARD,
    LEFTWARD,
    RIGHTWARD,
    SPRINT,
}

pub struct KeyMap {
    map: HashMap<Key, usize>,
    board: [bool;256],
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let map = KeyMap::default_key_mapping();
        let board = [false;256];

        KeyMap {
            map,
            board,
        }
    }

    pub fn default_key_mapping() -> HashMap<Key, usize> {
        let mut map = HashMap::new();

        map.insert(Key::FORWARD, 87);
        map.insert(Key::BACKWARD, 83);
        map.insert(Key::LEFTWARD, 65);
        map.insert(Key::RIGHTWARD, 68);
        map.insert(Key::SPRINT, 16); // shift

        map
    }

    pub fn val(&self, key: Key) -> usize{
        if let Some(v) = self.map.get(&key) { *v }
        else { 0 as usize }
    }

    pub fn get(&self, key: Key) -> bool {
        self.board[self.val(key)]
    }

    pub fn press(&mut self, key_val: i32) {
        self.board[key_val as usize] = true;
    }

    pub fn release(&mut self, key_val: i32) {
        self.board[key_val as usize] = false;
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}