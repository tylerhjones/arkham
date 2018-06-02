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

    fn new_lifecycle_step(&mut self) -> &mut Self {
        self.life_cycle_index += 1;
        return self;
    }
}



#[cfg(test)]
mod test {
    use runtime::RuntimeBuilder;

    #[test]
    fn assert_lifecycled_steps_increment() {
        let mut rt_builder = RuntimeBuilder::new();
        assert_eq!(rt_builder.life_cycle_index, 0);

        rt_builder.new_lifecycle_step();
        assert_eq!(rt_builder.life_cycle_index, 1);

        rt_builder
            .new_lifecycle_step()
            .new_lifecycle_step();
        assert_eq!(rt_builder.life_cycle_index, 3);
    }
}
