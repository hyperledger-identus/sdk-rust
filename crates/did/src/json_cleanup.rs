use serde_json::Value;

pub(crate) struct RejectionGuard<T, Cleanup>
where
    Cleanup: FnOnce(T),
{
    owner: Option<T>,
    cleanup: Option<Cleanup>,
}

impl<T, Cleanup> RejectionGuard<T, Cleanup>
where
    Cleanup: FnOnce(T),
{
    pub(crate) fn new(owner: T, cleanup: Cleanup) -> Self {
        Self {
            owner: Some(owner),
            cleanup: Some(cleanup),
        }
    }

    pub(crate) fn owner(&self) -> &T {
        self.owner
            .as_ref()
            .expect("an armed rejection guard always owns its candidate")
    }

    pub(crate) fn into_owner(mut self) -> T {
        self.owner
            .take()
            .expect("a rejection guard yields its candidate exactly once")
    }
}

impl<T, Cleanup> Drop for RejectionGuard<T, Cleanup>
where
    Cleanup: FnOnce(T),
{
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            let cleanup = self
                .cleanup
                .take()
                .expect("an armed rejection guard always owns its cleanup callback");
            cleanup(owner);
        }
    }
}

pub(crate) fn drop_json_values_iteratively(roots: impl IntoIterator<Item = Value>) {
    let mut pending: Vec<Value> = roots.into_iter().collect();

    while let Some(value) = pending.pop() {
        match value {
            Value::Array(mut values) => pending.append(&mut values),
            Value::Object(values) => pending.extend(values.into_values()),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use serde_json::{Map, Value};

    use super::{RejectionGuard, drop_json_values_iteratively};

    #[test]
    fn armed_guard_exposes_shared_owner_and_runs_cleanup_on_drop() {
        let cleaned = Cell::new(false);

        {
            let guard = RejectionGuard::new(String::from("candidate"), |owner| {
                assert_eq!(owner, "candidate");
                cleaned.set(true);
            });

            assert_eq!(guard.owner(), "candidate");
            assert!(!cleaned.get());
        }

        assert!(cleaned.get());
    }

    #[test]
    fn success_yields_the_exact_owner_without_running_cleanup() {
        let cleaned = Cell::new(false);
        let owner = Box::new(String::from("candidate"));
        let original = owner.as_ref() as *const String;
        let guard = RejectionGuard::new(owner, |_| cleaned.set(true));

        let owner = guard.into_owner();

        assert!(std::ptr::eq(original, owner.as_ref()));
        assert!(!cleaned.get());
    }

    #[test]
    fn iterative_cleanup_handles_a_32768_level_tree() {
        let mut nested = Value::Null;
        for level in 0..32_768 {
            nested = if level % 2 == 0 {
                Value::Array(vec![nested])
            } else {
                let mut object = Map::new();
                object.insert(String::from("child"), nested);
                Value::Object(object)
            };
        }

        let mut sibling = Map::new();
        sibling.insert(
            String::from("children"),
            Value::Array(vec![Value::Bool(true), Value::String(String::from("leaf"))]),
        );

        drop_json_values_iteratively([nested, Value::Object(sibling)]);
    }
}
