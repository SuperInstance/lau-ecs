# lau-ecs

A bitmask-driven Entity Component System (ECS) written in Rust, purpose-built for simulation-style game worlds. Entities carry up to 13 **component types** tracked by a `u64` bitmask; the `World` stores each component family in its own `HashMap`, and systems query by mask. The result is a tiny, serde-serialisable ECS with zero external dependencies beyond `serde` itself — ideal for embedding in game runtimes, teaching how ECS architectures work, or prototyping agent-based simulations.

**54 tests** · zero unsafe · `no_std`-friendly data types · full serde round-trips.

---

## Table of Contents

1. [What This Does](#what-this-does)
2. [Key Idea](#key-idea)
3. [Install](#install)
4. [Quick Start](#quick-start)
5. [API Reference](#api-reference)
6. [How It Works](#how-it-works)
7. [The Math](#the-math)
8. [License](#license)

---

## What This Does

`lau-ecs` gives you:

| Feature | Detail |
|---|---|
| **Entity management** | Spawn/despawn entities with auto-incrementing `EntityId(u64)` IDs. |
| **13 component types** | Position, Velocity, Vibe, Health, AgentAI, Renderable, Collider, Inventory, Pet, Dialogue, BiomeAffinity, Craftable, QuestTarget. |
| **Bitmask queries** | Build a `ComponentMask`, call `world.query(mask)` → get all matching `EntityId`s. |
| **Built-in systems** | `MovementSystem`, `VibePropagationSystem`, `CollisionSystem`, `ConservationSystem`. |
| **Collision detection** | Circle–circle, AABB–AABB, circle–AABB, point-vs-all. |
| **Serialisation** | Derives `Serialize`/`Deserialize` on every type. Freeze the entire `World` to JSON and thaw it back. |

---

## Key Idea

Each component type is assigned a fixed bit position inside a `u64`:

```
bit 0 → Position     bit 5 → Renderable    bit 10 → BiomeAffinity
bit 1 → Velocity     bit 6 → Collider      bit 11 → Craftable
bit 2 → Vibe         bit 7 → Inventory     bit 12 → QuestTarget
bit 3 → Health       bit 8 → Pet
bit 4 → AgentAI      bit 9 → Dialogue
```

An entity's **component mask** is just the OR of all its set bits. Querying is a simple bitmask subset test: `entity_mask & query_mask == query_mask`. This is O(n) over entities but constant-time per entity — and for the entity counts typical of this crate's use-cases (hundreds to low thousands), that's plenty fast.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-ecs = "0.1"
```

Or via `cargo add`:

```sh
cargo add lau-ecs
```

### Dependencies

| Crate | Why |
|---|---|
| `serde` (with `derive`) | Serialisation of all public types |

---

## Quick Start

```rust
use lau_ecs::*;

// 1. Create a world
let mut world = World::new();

// 2. Spawn entities
let player = world.spawn();
world.add_component(player, Component::Position(0.0, 0.0, 0.0));
world.add_component(player, Component::Velocity(1.0, 0.0, 0.0));
world.add_component(player, Component::Health(100.0));
world.add_component(player, Component::Vibe(50.0));

let npc = world.spawn();
world.add_component(npc, Component::Position(5.0, 0.0, 0.0));
world.add_component(npc, Component::Vibe(0.0));

// 3. Run systems
let mut movement = MovementSystem;
let mut vibes = VibePropagationSystem::default();
let mut collision = CollisionSystem;

movement.run(&mut world, 1);
vibes.run(&mut world, 1);

// 4. Query
let moving = world.query(
    ComponentMask::new()
        .with(ComponentType::Position)
        .with(ComponentType::Velocity)
);
assert_eq!(moving.len(), 1);

// 5. Serialise the entire world
let json = serde_json::to_string(&world).unwrap();
let restored: World = serde_json::from_str(&json).unwrap();
```

---

## API Reference

### Core Types

#### `EntityId`

```rust
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntityId(pub u64);
```

A newtype wrapper around `u64`. Cheap to copy, hash, and compare.

#### `ComponentMask`

```rust
pub struct ComponentMask(u64);
```

| Method | Description |
|---|---|
| `new()` | All bits cleared. |
| `with(ty)` | Builder: set one bit. |
| `insert(ty)` / `remove(ty)` | Mutate in place. |
| `has(ty)` | Check one bit. |
| `matches(&other)` | True if all bits in `other` are set in `self`. |

#### `ComponentType` (enum)

The 13 discriminants, from `Position = 0` through `QuestTarget = 12`.

#### `Component` (enum)

Runtime representation:

```rust
pub enum Component {
    Position(f64, f64, f64),
    Velocity(f64, f64, f64),
    Vibe(f64),
    Health(f64),
    AgentAI(AgentState),
    Renderable(RenderInfo),
    Collider(ColliderShape),
    Inventory(InventoryData),
    Pet(PetData),
    Dialogue(DialogueState),
    BiomeAffinity(String),
    Craftable(CraftRecipe),
    QuestTarget(QuestData),
}
```

Call `.component_type()` to recover the `ComponentType` discriminant.

#### `World`

| Method | Signature | Notes |
|---|---|---|
| `new()` | → `World` | Empty world, next ID = 1. |
| `spawn()` | → `EntityId` | Auto-incrementing. |
| `despawn(id)` | | Removes entity + all its components. |
| `add_component(id, comp)` | | Inserts into the correct sparse map, sets mask bit. |
| `remove_component(id, ty)` | | Removes from sparse map, clears mask bit. |
| `has_component(id, ty)` | → `bool` | |
| `get_component(id, ty)` | → `Option<Component>` | Returns a **clone**. |
| `query(mask)` | → `Vec<EntityId>` | All entities whose mask is a superset. |

#### Component Data Types

| Type | Key fields |
|---|---|
| `AgentState` | `program_counter`, `stack: Vec<f64>`, `state: String` |
| `RenderInfo` | `sprite`, `scale`, `layer` |
| `ColliderShape` | `Box { w, h }` · `Circle { r }` · `Point` |
| `InventoryData` | `items: HashMap<String, u32>`, `capacity` |
| `PetData` | `species`, `happiness`, `evolution_stage` |
| `DialogueState` | `current_node`, `mood`, `history: Vec<String>` |
| `CraftRecipe` | `inputs: HashMap<String, u32>`, `output`, `tick_cost` |
| `QuestData` | `quest_id`, `progress`, `completed` |

### Systems

All implement the `System` trait:

```rust
pub trait System {
    fn run(&mut self, world: &mut World, tick: u64);
}
```

#### `MovementSystem`

Adds velocity to position each tick for entities with both `Position` and `Velocity`:

```
pos += vel   (component-wise)
```

#### `VibePropagationSystem`

Spreads the `Vibe` component between nearby entities that also have `Position`:

| Field | Default | Meaning |
|---|---|---|
| `radius` | 10.0 | Max distance for coupling. |
| `coupling` | 0.1 | Fraction of vibe difference transferred per tick. |

```
flow = (vibe_a − vibe_b) × coupling    (if distance ≤ radius)
vibe_a -= flow
vibe_b += flow
```

This conserves total vibe (flow is symmetric).

#### `CollisionSystem`

Brute-force O(n²) pairwise collision detection among entities with `Position + Collider`.

| Shape pair | Algorithm |
|---|---|
| Circle–Circle | Distance ≤ r₁ + r₂ |
| Box–Box | AABB overlap on both axes |
| Circle–Box | Closest-point-on-AABB to circle center, distance ≤ r |
| Point–* | Treat point as infinitesimal circle |

Call `.detect_collisions(&world)` → `Vec<(EntityId, EntityId)>`.

#### `ConservationSystem`

Utility to verify that total `Vibe` is conserved across a simulation:

```rust
let baseline = cons.total_vibe(&world);
// ... run vibe-propagating systems ...
cons.check_conservation(&world, baseline)?;  // Ok(()) if within threshold
```

---

## How It Works

### Architecture

```
┌──────────────────────────────────────────┐
│                  World                    │
│  entities: HashMap<EntityId, Mask>        │
│  positions:   HashMap<EntityId, (f64×3)> │
│  velocities:  HashMap<EntityId, (f64×3)> │
│  vibes:       HashMap<EntityId, f64>     │
│  healths:     HashMap<EntityId, f64>     │
│  ... (one map per component type)        │
└──────────────────────────────────────────┘
```

Each component type lives in its own `HashMap<EntityId, Data>`. This is **sparse-set–adjacent** storage: absent components cost zero memory. The bitmask on each entity is the authoritative record of which maps to check.

### Spawn → Despawn lifecycle

1. `spawn()` allocates a new `EntityId` and inserts `EntityId → ComponentMask(0)` into `entities`.
2. `add_component(id, comp)` inserts into the relevant map **and** sets the mask bit.
3. `despawn(id)` reads the mask, removes from every set map, then removes the entity entry.

### Query mechanism

`query(mask)` iterates all `(EntityId, ComponentMask)` pairs and keeps those where `entity_mask.matches(&query_mask)`. This is:

- **O(n)** in entity count (scans all entities)
- **O(1)** per entity (single bitwise AND + comparison)
- Returns `Vec<EntityId>` (owned)

For the intended scale (hundreds of entities), this is faster and simpler than maintaining archetypes.

### System execution pattern

```rust
let mask = ComponentMask::new()
    .with(ComponentType::Position)
    .with(ComponentType::Velocity);
let ids = world.query(mask);
for id in ids {
    // read/write components via world.positions[&id], etc.
}
```

Systems first query for matching entities, then iterate. Writes happen directly on the sparse maps.

### Serialisation

Every type derives `Serialize` + `Deserialize`. A full `World` serialises to JSON and restores perfectly, including all component data and the entity ID counter. After deserialisation, systems continue to work correctly (tested).

---

## The Math

### Bitmask subset test

An entity's mask `M` matches a query mask `Q` iff:

```
M & Q == Q
```

This is the set-containment test for bitmasks. If bit *i* is set in `Q`, it must also be set in `M`.

### Movement integration (Euler)

```
x(t+dt) = x(t) + v · dt
```

With `dt = 1` (one tick), this simplifies to `pos += vel`.

### Vibe propagation (diffusive coupling)

For two entities *a* and *b* within distance `radius`:

```
Δ = (vibe_a − vibe_b) × coupling

vibe_a ← vibe_a − Δ
vibe_b ← vibe_b + Δ
```

Total vibe is conserved because `−Δ + Δ = 0`. This is a discrete approximation of the diffusion equation:

```
∂u/∂t = D · ∇²u
```

where `coupling` plays the role of the diffusion coefficient and the "spatial discretisation" is just the pairwise distance check.

### Collision detection

**Circle–Circle**: Two circles with radii *r₁*, *r₂* and centres *c₁*, *c₂* overlap when:

```
‖c₁ − c₂‖ ≤ r₁ + r₂
```

Squaring both sides avoids the `sqrt`:

```
Δx² + Δy² + Δz² ≤ (r₁ + r₂)²
```

**AABB–AABB**: Two axis-aligned boxes overlap iff they overlap on **every** axis:

```
|x₁ − x₂| < w₁/2 + w₂/2   AND   |y₁ − y₂| < h₁/2 + h₂/2
```

**Circle–AABB**: Find the closest point on the box to the circle centre, then check if that distance is within the radius:

```
closest_x = clamp(circle_x, box_x − w/2, box_x + w/2)
closest_y = clamp(circle_y, box_y − h/2, box_y + h/2)
dist² = (circle_x − closest_x)² + (circle_y − closest_y)²
overlap ⟺  dist² ≤ r²
```

---

## License

MIT
