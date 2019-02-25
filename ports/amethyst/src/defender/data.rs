use amethyst::{
    core::{
        ArcThreadPool,
        bundle::SystemBundle,
    },
    ecs::prelude::*,
    Error,
    prelude::*,
};

pub struct DefenderData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    running_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> DefenderData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world.res);
        }

        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct DefenderDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for DefenderDataBuilder<'a, 'b> {
    fn default() -> Self {
        DefenderDataBuilder::new()
    }
}

impl<'a, 'b> DefenderDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        DefenderDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>, {
        bundle.build(&mut self.core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_run_bundle<B>(mut self, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>, {
        bundle.build(&mut self.running)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_running<S>(
        mut self,
        system: S,
        name: &str,
        dependencies: &[&str]
    ) -> Self where for<'c> S: System<'c> + Send + 'a, {
        self.running.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<DefenderData<'a, 'b>> for DefenderDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> DefenderData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);

        DefenderData {
            core_dispatcher,
            running_dispatcher
        }
    }
}