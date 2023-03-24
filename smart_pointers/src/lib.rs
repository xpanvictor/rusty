pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where T: Messenger{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            max,
            value: 0
        }
    }

    pub fn set_value(&mut self, value: usize) {
        // set value of self first, then warn
        self.value = value;

        let percentage_used_of_max = self.value as f64 / self.max as f64;

        if percentage_used_of_max >= 1.0 {
            self.messenger.send("Error: You've exceeded your quota!")
        }else if percentage_used_of_max > 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota")
        } else if percentage_used_of_max > 0.75 {
            self.messenger.send("Warning: You've used up 75% of your bundle!")
        }
    }
}

pub fn mock_int_mutability() {
    
}

#[cfg(test)]
mod tests {

}
