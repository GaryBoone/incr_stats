pub mod batch;
pub mod desc;
pub mod error;
pub mod incr;

#[cfg(test)]
mod tests {
    mod array_test;
    mod batch_test;
    pub mod check;
    mod desc_test;
    mod equivalence_test;
    mod incr_test;
}
