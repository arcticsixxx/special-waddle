pub enum InputMode {
    Normal,
    Editing,
}

pub struct UserInput {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
}

pub enum Direction {
    Left,
    Right,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.character_index > 0 {
                    self.character_index -= 1;
                }
            }
            Direction::Right => {
                if self.character_index < self.input.len() {
                    self.character_index += 1;
                }
            }
        }
    }

    pub fn enter_char(&mut self, c: char) {
        let index = self.byte_index();
        self.input.insert(index, c);
        self.move_cursor(Direction::Right);
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        // nothing to delete
        if self.character_index == 0 {
            return;
        }

        let byte_idx = self
            .input
            .char_indices()
            .nth(self.character_index - 1)
            .map(|(i, _)| i)
            .unwrap();

        self.input.remove(byte_idx);
        self.move_cursor(Direction::Left);
    }

    pub fn submit_input(&mut self) {
        self.input.clear();
        self.character_index = 0;
    }
}
