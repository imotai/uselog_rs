# uselog_rs

before using `uselog_rs`, you must use four lines code to use log for outputing log in `test mode` and `not test mode`

```
#[cfg(not(test))]
use log::{debug, info, warn};
#[cfg(test)]
use std::{println as debug, println as info, println as warn};
```
now just one line code 

```
uselog!(debug, info, warn)
```

but you need to add `uselog_rs` to your parent module first like
```
#[macro_use(uselog)]
extern crate uselog_rs;
```


