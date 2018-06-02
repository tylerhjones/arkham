use lifecycled::LifecyclePhase;

struct RuntimeBuilder {
    life_cycle_index: u16,
    life_cycle: Vec<LifecyclePhase>,
}

impl RuntimeBuilder {
    fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            life_cycle_index: 0,
            life_cycle: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use runtime::RuntimeBuilder;

    #[test]
    fn assert_lifecycled_steps_increment() {
        let mut rt_builder = RuntimeBuilder::new();
    }
}
