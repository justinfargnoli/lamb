#[cfg(test)]
mod tests {
    #[test]
    fn inkwell() {
        use inkwell::context::Context;
        let _ = Context::create();
    }
}
