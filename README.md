# moecs

![Build Status](https://github.com/mccloskeybr/moecs/workflows/CI/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mccloskeybr/moecs/blob/master/LICENSE)

`moecs` (_micro ECS_) is a small
[ECS](https://en.wikipedia.org/wiki/Entity_component_system)
library written in Rust.

Built to be used with lightweight Rust-based game engines, like
[ggez](https://github.com/ggez/ggez).

See example implementations [here](./example).

## Features

*   Simple user-facing API.
*   Entity query caching for efficient repeat lookups.
*   Configurable parallelism (powered by
    [rayon](https://github.com/rayon-rs/rayon)):
    *   Entity queries are run in parallel (when not cached).
    *   System execution can be configured to run in parallel.
    *   System parameters are wrapped in `Arc<RwLock>`, so parallelism can
        easily be achieved within a `System` as well.

## Documentation

### Components

#### Component

`Component`s are highly configurable bundles of data that can be arbitrarily
grouped together to form an Entity. For example, a Dog Entity may be
comprised of a position component (where it is in the world), and a state
component (what it's getting up to). In `moecs`, this would be implemented like
so:

```rust
#[derive(Component)]
struct PositionComponent {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct DogStateComponent {
    state: DogState,
}
enum DogState {
    Sleeping,
    Playing,
    Barking,
}
```

Note that the `#[derive(Component)]` attribute must be defined for each
`Component`.

#### ComponentBundle

Components can be easily bundled together using a `ComponentBundle`
(this is most useful when creating a new Entity, as explored later). Using our
Dog example, we can group the `PositionComponent` and `DogStateComponent`s
using the following:

```rust
let bundle = ComponentBundle::new()
    .add_component(PositionComponent {
        x: 0,
        y: 0,
    })
    .add_component(DogStateComponent {
        state: Sleeping,
    });
```

### Entities

#### EntityManager

As discussed in the Component section, Entities are simply bundles of relevant
Components. Operations on Entities are done via the `EntityManager`.

Note: The `EntityManager` is passed directly to defined `System`s. Therefore,
all Entity-related mutations can only occur in a `System`.

Responsibilities of the `EntityManager` include:

*   Creating new Entities.

Entities are created by providing a `ComponentBundle` of initial `Component`s
to be associated with that Entity (Note: an empty `ComponentBundle` may be
provided). A `u32` will be returned denoting that Entity's unique identifier,
which can be used to retrieve that Entity's `Component`s later.

Note: A strict requirement is that an Entity can only have one Component of
a given type registered at a given time. `moecs` will `panic` if this rule is
broken.

```rust
entity_manager.create_entity(
    ComponentBundle::new()
        .add_component(PositionComponent { x: 0, y: 0 })
);
```

*   Removing existing Entities.

Given some Entity id, that Entity can be removed wholesale (incl. deleting all
relevant `Component`s) via:

```rust
entity_manager.delete_entity(entity_id);
```

*   Adding Components to existing Entities.

Additional `Component`s can be added to an existing Entity via:

```rust
entity_manager.add_components_to_entity(
    &entity_id,
    ComponentBundle::new()
        .add_component(VelocityComponent { x_vel: 0, y_vel: 0 })
);
```

Note: The above stated rule that an Entity may only have one Component of a
given type still applies here. `moecs` will `panic` if this rule is broken.

*   Removing Components from existing Entities.

Similarly, Components can be removed (deleted) from an existing Entity via:

```rust
entity_manager.remove_component_from_entity::<PositionComponent>(&entity_id);
```

*   Querying for Entities that have (or don't have) specified Components.

Querying is done using the `Query` struct, which has 2 mechanisms of specifying
filter criteria: `with`, `without`. These are used to iterate over the list of
all registered Entities in order to filter out Entities *with* a certain
`Component`, or similarly *without* other components as applicable.

Query results are returned via a `QueryResult` struct, which includes the
Entity id of the filtered Entity, as well as the relevant `Component`s.

Note: Query results are automatically cached. Additionally, query processing is
performed in parallel (across registered Entities) to improve efficiency.

Example (simplified) flow:

```rust
entity_manager
    .filter(
        Query::new()
            .with::<SomeComponent>()
            .without::<SomeOtherComponent>(),
    )
    .iter()
    .for_each(|result: QueryResult| {
        let component = result.get_component::<SomeComponent>();
        println!(
            "Entity: {} has component {:?}.",
            result.entity_id(),
            component
        );
    });
```

### Systems

#### System

`System`s are where `Component`s belonging to certain Entities change and
interact with each other. In other words, `System`s contain the logic of your
program. For example, a rudimentary `PhysicsSystem` could be implemented like
so:

```rust
#[derive(System)]
struct PhysicsSystem;
impl System for PhysicsSystem {
    fn execute(entity_manager: Arc<RwLock<EntityManager>>, params: Arc<SystemParamAccessor>) {
        entity_manager
            .read()
            .unwrap()
            .filter(
                Query::new()
                    .with::<PositionComponent>()
                    .with::<VelocityComponent>(),
            )
            .iter()
            .for_each(|result| {
                let entity_id = result.entity_id();
                let position = result.get_component::<PositionComponent>().unwrap();
                let velocity = result.get_component::<VelocityComponent>().unwrap();

                position.write().unwrap().x += velocity.read().unwrap().x_vel;
                position.write().unwrap().y += velocity.read().unwrap().y_vel;
            });
    }
}
```

A couple things to note:

*   All `System`s must use the `#[derive(System)]` attribute.
*   All `System`s must similarly implement the `System` trait, which
    essentially means implementing the `execute` `fn`. This gives you access to
    the `EntityManager` (discussed above), as well as the `SystemParamAccessor`
    (discussed below).

#### System Parameters

It's often adventageous to pass data from outside the `moecs` ecosystem in (for
example, an input handler, or a rendering canvas). These can be passed via.
a `SystemParam`. An example definition:

```rust
#[derive(SystemParam)]
struct CanvasParam<'a> {
    canvas: &'a mut Canvas,
}
```

`SystemParam`s are collected and accessed by a `SystemParamAccessor`, which
essentially just provides a convenient means of looking up a `SystemParam`
from within the `System`. For example:

```rust
#[derive(System)]
struct RenderSystem;
impl System for RenderSystem {
    fn execute(entity_manager: Arc<RwLock<EntityManager>>, params: Arc<SystemParamAccessor>) {
        let canvas_param = params.get_param::<CanvasParam>().unwrap();
        let canvas_param = &mut canvas_param.write().unwrap();
        let canvas = &mut canvas_param.canvas;

        // etc.
    }
}
```

#### System Groups

A `SystemGroup` is a user-defined grouping of like-`System`s. Practically,
a `System` can only be interacted with via. a `SystemGroup`.

`System`s in a `SystemGroup` can be registered to execute in sequence or in
parallel depending on configuration by the user. For example:

```rust
let group_1 = SystemGroup::new_sequential_group()
    .register::<PhysicsSystem>()
    .register::<CollisionSystem>();
let group_2 = SystemGroup::new_parallel_group().register::<RenderSystem>();
```

Parallelism here is horizontal. That is, the `System`s themselves are run in
parallel with each other. Parallelism *within* a `System` is done separatetely
(manually).

### Engine

The `Engine` is how `moecs` is interacted with by some outside process (i.e.
some central game loop). It has the following responsibilities:

*   Register / deregister `SystemGroup`s.
*   Execute a registered `SystemGroup`.

The general flow is as follows:

*   Upon program start up, define and group `System`s of similar type using
    `SystemGroup`s (discussed above).
*   Then, in the main loop, execute the `SystemGroup`s in some sequential
    manner.

A basic example:

```rust
fn main() {
    let mut engine = Engine::new();

    let update_systems = engine.register_system_group(
        SystemGroup::new_sequential_group()
            .register::<PhysicsSystem>(),
    );
    let render_systems = engine.register_system_group(
        SystemGroup::new_sequential_group()
            .register::<DrawShapeSystem>(),
    );

    loop {
        engine.execute_group(update_systems, SystemParamAccessor::new());
        engine.execute_group(render_systems, SystemParamAccessor::new());
    }
}
```
