## TimeLite

**TimeLite** is a simple library to convert days, weeks, months and years into seconds.

This library is not necessary if you are using something like `chrono` crate but its purpose is to be very ergonomic for users to write seconds on a human level understanding of minutes, hours, days, weeks, months and years.

### Examples

#### 1. Handling Nanoseconds

```rust
use timelite::LiteDuration;

let timer = LiteDuration::nanos(100);
```

#### 2. Handling Microseconds

```rust
use timelite::LiteDuration;

let timer = LiteDuration::micros(1);
```

#### 3. Handling Milliseconds

```rust
use timelite::LiteDuration;

let timer = LiteDuration::millis(1);
```

#### 4. Handling Seconds

```rust
use timelite::LiteDuration;

let timer = LiteDuration::seconds(1);
```

#### 5. Handling Minutes

```rust
use timelite::LiteDuration;

let timer = LiteDuration::minutes(1);
```

#### 6. Handling Hours

```rust
use timelite::LiteDuration;

let timer = LiteDuration::hours(1);
```

#### 7. Handling Days

```rust
use timelite::LiteDuration;

let timer = LiteDuration::days(1);
```

#### 8. Handling Weeks

```rust
use timelite::LiteDuration;

let timer = LiteDuration::weeks(1);
```

#### 9. Handling Months

```rust
use timelite::LiteDuration;

let timer = LiteDuration::months(1);
```

#### 10. Handling Years

```rust
use timelite::LiteDuration;

let timer = LiteDuration::years(1);
```

