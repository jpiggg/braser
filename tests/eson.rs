use pest_test_gen::{pest_tests};

#[pest_tests(
  eson::parser::ESONParser,
  eson::parser::Rule,
  "ESon",
  recursive = true,
  lazy_static = true,
)]

#[cfg(test)]
mod object_tests {}