fn main() {}

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    limit: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &T, limit: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            limit,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let of_max = self.value as f64 / self.limit as f64;
        if 0.75 <= of_max && of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if 0.9 <= of_max && of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn sends_warning_message() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(77);

        assert_eq!(1, messenger.messages.borrow().len());
        assert_eq!(
            "Warning: You've used up over 75% of your quota!",
            messenger.messages.borrow().get(0).unwrap()
        );
    }

    #[test]
    fn sends_urgent_warning_message() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(91);

        assert_eq!(1, messenger.messages.borrow().len());
        assert_eq!(
            "Urgent warning: You've used up over 90% of your quota!",
            messenger.messages.borrow().get(0).unwrap()
        );
    }

    #[test]
    fn sends_error_message() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(101);

        assert_eq!(1, messenger.messages.borrow().len());
        assert_eq!(
            "Error: You are over your quota!",
            messenger.messages.borrow().get(0).unwrap()
        );
    }

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }
}
