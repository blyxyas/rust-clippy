{
    use std::ops::ControlFlow;
    store.register_late_pass(|_| Box::new(default_numeric_fallback::DefaultNumericFallback));
    return ();
}
