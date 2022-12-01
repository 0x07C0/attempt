use std::cmp::Ordering;

pub enum Value {
    Any,
    Number(i32),
}

fn reduce_by_allowed(original: &Vec<i32>, allowed: &Vec<Value>) -> Vec<i32> {
    let mut o_pointer = 0;
    let mut a_pointer = 0;
    let mut vec: Vec<i32> = vec![];
    while o_pointer < original.len() && a_pointer < allowed.len() {
        if let Value::Number(num) = &allowed[a_pointer] {
            match original[o_pointer].cmp(num) {
                Ordering::Less => o_pointer += 1,
                Ordering::Greater => a_pointer += 1,
                Ordering::Equal => {
                    vec.push(original[o_pointer]);
                    o_pointer += 1;
                    a_pointer += 1;
                }
            }
        }
    }
    vec
}

fn reduce_by_preferred(original: &Vec<i32>, preferred: &Vec<Value>) -> Vec<i32> {
    todo!()
}

pub fn attempt(available: &Vec<i32>, allowed: &Vec<Value>, preferred: &Vec<Value>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reducing_by_allowed() {
        assert_eq!(
            reduce_by_allowed(
                &vec![240, 360, 720],
                &vec![Value::Number(360), Value::Number(720)]
            ),
            vec![360, 720]
        );

        assert_eq!(
            reduce_by_allowed(
                &vec![240, 720],
                &vec![Value::Number(360), Value::Number(720)]
            ),
            vec![720]
        );

        assert_eq!(
            reduce_by_allowed(&vec![240], &vec![Value::Number(360), Value::Number(720)]),
            vec![]
        );
    }

    #[test]
    fn reducing_by_preferred() {
        assert_eq!(
            reduce_by_preferred(&vec![240, 360, 720], &vec![Value::Number(360)]),
            vec![360]
        );

        assert_eq!(
            reduce_by_preferred(&vec![360, 720], &vec![Value::Number(1080)]),
            vec![720]
        );

        assert_eq!(
            reduce_by_preferred(
                &vec![240, 360, 720],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![240, 360]
        );
        assert_eq!(
            reduce_by_preferred(
                &vec![240, 360, 720],
                &vec![Value::Number(240), Value::Number(1080)]
            ),
            vec![240, 720]
        );
        assert_eq!(
            reduce_by_preferred(
                &vec![240, 720],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![240, 720]
        );
    }

    #[test]
    fn test_attempt() {
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![Value::Number(360), Value::Number(720)],
                &vec![Value::Number(1080)]
            ),
            vec![720]
        );
        assert_eq!(
            attempt(
                &vec![240, 720],
                &vec![Value::Number(360), Value::Number(720)],
                &vec![Value::Number(1080)]
            ),
            vec![720]
        );
        assert_eq!(
            attempt(
                &vec![240],
                &vec![Value::Number(360), Value::Number(720)],
                &vec![Value::Number(1080)]
            ),
            vec![]
        );
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![
                    Value::Number(240),
                    Value::Number(360),
                    Value::Number(720),
                    Value::Number(1080)
                ],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![240, 360]
        );
        assert_eq!(
            attempt(
                &vec![240, 720],
                &vec![
                    Value::Number(240),
                    Value::Number(360),
                    Value::Number(720),
                    Value::Number(1080)
                ],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![240, 720]
        );
        assert_eq!(
            attempt(
                &vec![240, 720],
                &vec![Value::Number(240), Value::Number(360), Value::Number(1080)],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![240]
        );
        assert_eq!(
            attempt(
                &vec![720],
                &vec![Value::Number(240), Value::Number(360), Value::Number(1080)],
                &vec![Value::Number(240), Value::Number(360)]
            ),
            vec![]
        );
        assert_eq!(
            attempt(
                &vec![240, 360],
                &vec![Value::Number(240), Value::Number(360)],
                &vec![Value::Number(720), Value::Number(1080)]
            ),
            vec![360]
        );
    }

    #[test]
    fn test_attempt_with_any() {
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![Value::Number(360), Value::Any],
                &vec![Value::Number(360), Value::Number(720)]
            ),
            vec![360, 720]
        );
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![Value::Number(240), Value::Number(360), Value::Number(720)],
                &vec![Value::Any, Value::Number(720)]
            ),
            vec![240, 360, 720]
        );
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![Value::Number(360), Value::Number(1080)],
                &vec![Value::Any, Value::Number(720)]
            ),
            vec![360]
        );
        assert_eq!(
            attempt(
                &vec![240, 360, 720],
                &vec![Value::Number(1080)],
                &vec![Value::Any, Value::Number(720)]
            ),
            vec![]
        );
        assert_eq!(
            attempt(&vec![240, 360, 720], &vec![Value::Any], &vec![Value::Any]),
            vec![240, 360, 720]
        );
    }
}
