#[derive(Debug)]
pub struct BoxVec {
    cells: [Option<u32>; 3],
}

impl BoxVec {
    pub fn new() -> BoxVec {
        BoxVec {
            cells: [None, None, None],
        }
    }

    pub fn peek(&self, box_index: usize) -> u32 {
        match self.cells[box_index] {
            Some(value) => value,
            None => panic!("Empty box!"),
        }
    }

    pub fn replace(&mut self, box_index: usize, new_value: u32) -> u32{
        match self.cells[box_index] {
            Some(value) => value,
            None => {
                self.cells[box_index] = Some(new_value);
                new_value
            }
        }
    }

    pub fn remove(&mut self, box_index: usize) -> u32 {
        match self.cells[box_index] {
            Some(value) => {
                self.cells[box_index] = None;
                value
            },
            None => panic!("Empty box!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BoxVec;

    // |BoxVec tests|------------------------------------------
    #[test]
    fn test_peek_success() {
        let mut box_vec = BoxVec::new();
        let (index, value) = (0, 100);
        box_vec.cells[index] = Some(value);
        assert_eq!(
            box_vec.peek(index),
            100,
            "Peeked item at {} should have value {:?}",
            index,
            Some(value),
        );
    }

    #[test]

    #[should_panic]
    fn test_peek_empty() {
        let box_vec = BoxVec::new();
        box_vec.peek(0);
    }

    #[test]
    #[should_panic]
    fn test_peek_invalid_index() {
        let mut box_vec = BoxVec::new();
        box_vec.cells[0] = Some(100);
        box_vec.peek(3);
    }

    #[test]

    fn test_replace_existing() {
        let mut box_vec = BoxVec::new();
        let (index, old_value, new_value) = (0, 100, 50);
        box_vec.cells[index] = Some(old_value);
        assert_eq!(
            box_vec.cells[index], Some(old_value),
            "Something went wrong during test setup"
        );
        box_vec.replace(index, new_value);
        assert_eq!(
            box_vec.cells[index], Some(old_value),
            "Existing item at {} should still have value {:?}, current value is {:?}",
            index,
            Some(old_value),
            Some(new_value),
        );
    }

    #[test]

    fn test_replace_nonexisting() {
        let mut box_vec = BoxVec::new();
        let (index, value) = (0, 100);
        box_vec.replace(index, value);
        assert_eq!(
            box_vec.cells[index],
            Some(value),
            "Nonexisting item at {} should be updated to value {}",
            index,
            value,
        );
    }

    #[test]
    fn test_remove_existing() {
        let mut box_vec = BoxVec::new();
        let (index, value) = (0, 100);
        box_vec.cells[index] = Some(value);
        assert_eq!(
            box_vec.cells[index],
            Some(value),
            "Something went wrong during test setup"
        );
        box_vec.remove(index);
        assert_eq!(
            box_vec.cells[index],
            None,
            "Existing item at {} should be removed, currently {:?}",
            index,
            box_vec.cells[index],
        )
    }

    #[test]
    #[should_panic]
    fn test_remove_nonexisting() {
        let mut box_vec = BoxVec::new();
        box_vec.remove(0);
    }

    // |Option tests|------------------------------------------
    #[test]
    fn test_option() {
        let some: Option<u32> = Some(1);
        let some2: Option<u32> = Some(2);
        let somestr: Option<&str> = Some("hello");
        let none: Option<u32> = None;

        // and
        assert_eq!(some.and(none), None);
        assert_eq!(none.and(some), None);
        assert_eq!(some.and(somestr), somestr);

        // or
        assert_eq!(some.or(none), some);
        assert_eq!(none.or(some), some);
        assert_eq!(some.or(some2), some);

        // flatten
        let nested_some = Some(Some(1));
        let nested_none: Option<Option<u32>> = Some(None);
        let nested_none2: Option<Option<u32>> = None;
        assert_eq!(nested_some.flatten(), Some(1));
        assert_eq!(nested_none.flatten(), None);
        assert_eq!(nested_none2.flatten(), None);

        // map
        assert_eq!(some.map(|i| i.to_string()), Some(String::from("1")));
        assert_eq!(none.map(|i| i.to_string()), None);

        // map_or
        assert_eq!(some.map_or(String::from("10000"), |i| i.to_string()), String::from("1"));
        assert_eq!(none.map_or(String::from("10000"), |i| i.to_string()), String::from("10000"));

        // map_or_else
        assert_eq!(some.map_or_else(|| String::from("10000"), |i| i.to_string()), String::from("1"));
        assert_eq!(none.map_or_else(|| String::from("10000"), |i| i.to_string()), String::from("10000"));

        // ok_or
        assert_eq!(some.ok_or(0), Ok(1));
        assert_eq!(none.ok_or(0), Err(0));

        // transponse
        let result: Result<Option<u32>, u32> = Ok(Some(1));
        let option: Option<Result<u32, u32>> = Some(Ok(1));
        assert_eq!(option.transpose(), result);

        // unwrap
        assert_eq!(Some(1).unwrap(), 1);

        // unwrap_or
        assert_eq!(Some(1).unwrap_or(0), 1);
        assert_eq!(None.unwrap_or(0), 0);

        // zip
        assert_eq!(Some(1).zip(Some("one")), Some((1, "one")));
        assert_eq!(Some(1).zip(none), None);
    }

    // |Result tests|------------------------------------------
    #[test]
    fn test_result() {
        let ok: Result<u32, &str> = Ok(1);
        let ok2: Result<u32, &str> = Ok(2);
        let okstr: Result<&str, &str> = Ok("ok");
        let err: Result<u32, &str> = Err("error");
        let err2: Result<u32, &str> = Err("error2");

        // ok
        assert_eq!(ok.ok(), Some(1));
        assert_eq!(err.ok(), None);

        // err
        assert_eq!(ok.err(), None);
        assert_eq!(err.err(), Some("error"));

        // and
        assert_eq!(ok.and(err), err);
        assert_eq!(err.and(ok), err);
        assert_eq!(ok.and(okstr), okstr);
        assert_eq!(err.and(err2), err);

        // or
        assert_eq!(ok.or(err), ok);
        assert_eq!(err.or(ok), ok);
        assert_eq!(ok.or(ok2), ok);
        assert_eq!(err.or(err2), err2);

        // expect
        assert_eq!(ok.expect("failed"), 1);

        // unwrap
        assert_eq!(ok.unwrap(), 1);

        // unwrap_or
        assert_eq!(ok.unwrap_or(0), 1);
        assert_eq!(err.unwrap_or(0), 0);

        // unwrap_err
        assert_eq!(err.unwrap_err(), "error");

        // map_err
        assert_eq!(ok.map_err(|i| i.to_string()), Ok(1));
        assert_eq!(err.map_err(|s| format!("{} mapped", s)), Err(String::from("error mapped")));

        // transpose
        let result: Result<Option<u32>, &str> = Ok(Some(1));
        let option: Option<Result<u32, &str>> = Some(Ok(1));
        assert_eq!(result.transpose(), option);
    }
}
