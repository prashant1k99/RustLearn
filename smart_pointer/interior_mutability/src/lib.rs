// RefCell checks the borrowing rules at runtime
// Since it checks the borrowing rules at Runtime so the performance takes a slight hit

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messanger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messanger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messanger.send("Error: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messanger
                .send("Urgent warning: You've exhausted your resources");
        } else if percentage_of_max >= 0.75 {
            self.messanger
                .send("Warning: You's consumed 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessanger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessanger {
        fn new() -> MockMessanger {
            MockMessanger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessanger {
        fn send(&self, msg: &str) {
            // This works, now to check what happens when we try to take 2 mutable references at
            // runtime
            // self.sent_messages.borrow_mut().push(String::from(msg));

            // Note there is no compiler error here as RefCell checks it on runtime
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));
            // thread 'tests::it_sends_an_over_75_percentage_warning_msg' panicked at src/lib.rs:67:53:
            // already borrowed: BorrowMutError
        }
    }

    #[test]
    fn it_sends_an_over_75_percentage_warning_msg() {
        let mock_messanger = MockMessanger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
    }
}
