use pest_test_gen::{pest_tests};

#[pest_tests(
  eson::parser::ESonParser,
  eson::parser::Rule,
  "ESon",
  recursive = true,
  lazy_static = true,
)]

#[cfg(test)]
mod eson_tests {}
