# Deterministic Time Functions

## Sync Example
```
use std::time::Duration;

use dtf::sync::dtf;

let sensitive_method = || {
    if let Some(user) = users.get(user_id) {
        if user.password_hash.validate(password) {
            return Ok(());
        }
    }
    return Err("invalid credentials");
}

let min_duration = Duration::from_millis(1500);
let jitter = Duration::from_millis(100);

let result = dtf(min_duration, jitter, sensitive_method);
```

## Async Example

Requires the `async` feature be enabled.

```
use std::time::Duration;

use dtf::future::dtf;

let sensitive_method = async || {
    if let Some(user) = users.get(user_id) {
        if user.password_hash.validate(password) {
            return Ok(());
        }
    }
    return Err("invalid credentials");
}

let min_duration = Duration::from_millis(1500);
let jitter = Duration::from_millis(100);

let result = dtf(min_duration, jitter, sensitive_method).await;
```