pub struct Effectful<T> {
    replay: bool,
    history: Vec<T>,
}

impl<T> Default for Effectful<T> {
    fn default() -> Self {
        Self {
            replay: false,
            history: Vec::new(),
        }
    }
}

impl<'a, T: Clone> Effectful<T> {
    pub fn replay(history: Vec<T>) -> Self {
        Self {
            replay: true,
            history,
        }
    }

    pub fn run(&mut self, run: impl FnOnce() -> T) -> T {
        if self.replay {
            self.history.pop().unwrap()
        } else {
            let result = run();
            self.history.insert(0, result.clone());
            result
        }
    }

    pub fn outcomes(self) -> Vec<T> {
        self.history
    }
}
