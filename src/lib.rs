pub mod batch;
pub mod error;
pub mod incr;
pub mod vec;

#[cfg(test)]
mod tests {
    mod array_test;
    mod batch_test;
    pub mod check;
    mod equivalence_test;
    mod incr_test;
    mod vec_test;
}
