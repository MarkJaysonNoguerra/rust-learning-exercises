// normal paths
// use std::cmp::Ordering;
// use std::io;

// nested paths
// use std::{cmp::Ordering, io};

// normal paths
// use std::io;
// use std::io::Write;

// merging paths with the use of self
use std::io::{self, Write};

// The Glob Operator
// If we want to bring all public items defined in a path into scope
// we can specify that path followed by the `*` glob operator:
use std::collections::*;
