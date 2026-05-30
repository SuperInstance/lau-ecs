use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Entity ID
// ---------------------------------------------------------------------------

/// A newtype wrapper around `u64` representing a unique entity identifier.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntityId(pub u64);

// ---------------------------------------------------------------------------
// Component types
// ---------------------------------------------------------------------------

/// Bitmask tracking which components an entity possesses.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct ComponentMask(u64);

impl ComponentMask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn with(mut self, ty: ComponentType) -> Self {
        self.insert(ty);
        self
    }

    pub fn insert(&mut self, ty: ComponentType) {
        self.0 |= 1u64 << (ty as u8);
    }

    pub fn remove(&mut self, ty: ComponentType) {
        self.0 &= !(1u64 << (ty as u8));
    }

    pub fn has(&self, ty: ComponentType) -> bool {
        (self.0 & (1u64 << (ty as u8))) != 0
    }

    /// Returns `true` if *all* bits in `other` are present in this mask.
    pub fn matches(&self, other: &ComponentMask) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

/// The 13 component types supported by the ECS.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    Position = 0,
    Velocity = 1,
    Vibe = 2,
    Health = 3,
    AgentAI = 4,
    Renderable = 5,
    Collider = 6,
    Inventory = 7,
    Pet = 8,
    Dialogue = 9,
    BiomeAffinity = 10,
    Craftable = 11,
    QuestTarget = 12,
}

// ---------------------------------------------------------------------------
// Component data types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub program_counter: usize,
    pub stack: Vec<f64>,
    pub state: String,
}

impl Default for AgentState {
    fn default() -> Self {
        Self {
            program_counter: 0,
            stack: Vec::new(),
            state: "idle".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderInfo {
    pub sprite: String,
    pub scale: f64,
    pub layer: u32,
}

impl Default for RenderInfo {
    fn default() -> Self {
        Self {
            sprite: "default".to_string(),
            scale: 1.0,
            layer: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColliderShape {
    Box { w: f64, h: f64 },
    Circle { r: f64 },
    Point,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryData {
    pub items: HashMap<String, u32>,
    pub capacity: u32,
}

impl Default for InventoryData {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            capacity: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetData {
    pub species: String,
    pub happiness: f64,
    pub evolution_stage: u32,
}

impl Default for PetData {
    fn default() -> Self {
        Self {
            species: "unknown".to_string(),
            happiness: 50.0,
            evolution_stage: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueState {
    pub current_node: String,
    pub mood: f64,
    pub history: Vec<String>,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            current_node: "start".to_string(),
            mood: 0.0,
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftRecipe {
    pub inputs: HashMap<String, u32>,
    pub output: String,
    pub tick_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub quest_id: String,
    pub progress: f64,
    pub completed: bool,
}

impl Default for QuestData {
    fn default() -> Self {
        Self {
            quest_id: String::new(),
            progress: 0.0,
            completed: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Component enum
// ---------------------------------------------------------------------------

/// Runtime representation of any component value.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Component {
    pub fn component_type(&self) -> ComponentType {
        match self {
            Self::Position(..) => ComponentType::Position,
            Self::Velocity(..) => ComponentType::Velocity,
            Self::Vibe(..) => ComponentType::Vibe,
            Self::Health(..) => ComponentType::Health,
            Self::AgentAI(..) => ComponentType::AgentAI,
            Self::Renderable(..) => ComponentType::Renderable,
            Self::Collider(..) => ComponentType::Collider,
            Self::Inventory(..) => ComponentType::Inventory,
            Self::Pet(..) => ComponentType::Pet,
            Self::Dialogue(..) => ComponentType::Dialogue,
            Self::BiomeAffinity(..) => ComponentType::BiomeAffinity,
            Self::Craftable(..) => ComponentType::Craftable,
            Self::QuestTarget(..) => ComponentType::QuestTarget,
        }
    }
}

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------

/// The central ECS world storing entities and their components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    // Entity tracking
    pub entities: HashMap<EntityId, ComponentMask>,
    next_id: u64,

    // Per-component sparse storage
    pub positions: HashMap<EntityId, (f64, f64, f64)>,
    pub velocities: HashMap<EntityId, (f64, f64, f64)>,
    pub vibes: HashMap<EntityId, f64>,
    pub healths: HashMap<EntityId, f64>,
    pub agent_ais: HashMap<EntityId, AgentState>,
    pub renderables: HashMap<EntityId, RenderInfo>,
    pub colliders: HashMap<EntityId, ColliderShape>,
    pub inventories: HashMap<EntityId, InventoryData>,
    pub pets: HashMap<EntityId, PetData>,
    pub dialogues: HashMap<EntityId, DialogueState>,
    pub biome_affinities: HashMap<EntityId, String>,
    pub craftables: HashMap<EntityId, CraftRecipe>,
    pub quest_targets: HashMap<EntityId, QuestData>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Create an empty world.
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
            positions: HashMap::new(),
            velocities: HashMap::new(),
            vibes: HashMap::new(),
            healths: HashMap::new(),
            agent_ais: HashMap::new(),
            renderables: HashMap::new(),
            colliders: HashMap::new(),
            inventories: HashMap::new(),
            pets: HashMap::new(),
            dialogues: HashMap::new(),
            biome_affinities: HashMap::new(),
            craftables: HashMap::new(),
            quest_targets: HashMap::new(),
        }
    }

    /// Spawn a new entity and return its ID.
    pub fn spawn(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.entities.insert(id, ComponentMask::new());
        id
    }

    /// Despawn an entity and remove all its components.
    pub fn despawn(&mut self, id: EntityId) {
        let mask = match self.entities.remove(&id) {
            Some(m) => m,
            None => return,
        };
        // Remove every component this entity had
        if mask.has(ComponentType::Position) {
            self.positions.remove(&id);
        }
        if mask.has(ComponentType::Velocity) {
            self.velocities.remove(&id);
        }
        if mask.has(ComponentType::Vibe) {
            self.vibes.remove(&id);
        }
        if mask.has(ComponentType::Health) {
            self.healths.remove(&id);
        }
        if mask.has(ComponentType::AgentAI) {
            self.agent_ais.remove(&id);
        }
        if mask.has(ComponentType::Renderable) {
            self.renderables.remove(&id);
        }
        if mask.has(ComponentType::Collider) {
            self.colliders.remove(&id);
        }
        if mask.has(ComponentType::Inventory) {
            self.inventories.remove(&id);
        }
        if mask.has(ComponentType::Pet) {
            self.pets.remove(&id);
        }
        if mask.has(ComponentType::Dialogue) {
            self.dialogues.remove(&id);
        }
        if mask.has(ComponentType::BiomeAffinity) {
            self.biome_affinities.remove(&id);
        }
        if mask.has(ComponentType::Craftable) {
            self.craftables.remove(&id);
        }
        if mask.has(ComponentType::QuestTarget) {
            self.quest_targets.remove(&id);
        }
    }

    /// Add a component to an entity, updating the mask.
    pub fn add_component(&mut self, id: EntityId, component: Component) {
        let ty = component.component_type();
        self.insert_component_inner(id, component);
        self.entities
            .entry(id)
            .and_modify(|mask| mask.insert(ty));
    }

    fn insert_component_inner(&mut self, id: EntityId, component: Component) {
        match component {
            Component::Position(x, y, z) => drop(self.positions.insert(id, (x, y, z))),
            Component::Velocity(x, y, z) => drop(self.velocities.insert(id, (x, y, z))),
            Component::Vibe(v) => drop(self.vibes.insert(id, v)),
            Component::Health(h) => drop(self.healths.insert(id, h)),
            Component::AgentAI(a) => drop(self.agent_ais.insert(id, a)),
            Component::Renderable(r) => drop(self.renderables.insert(id, r)),
            Component::Collider(c) => drop(self.colliders.insert(id, c)),
            Component::Inventory(i) => drop(self.inventories.insert(id, i)),
            Component::Pet(p) => drop(self.pets.insert(id, p)),
            Component::Dialogue(d) => drop(self.dialogues.insert(id, d)),
            Component::BiomeAffinity(b) => drop(self.biome_affinities.insert(id, b)),
            Component::Craftable(c) => drop(self.craftables.insert(id, c)),
            Component::QuestTarget(q) => drop(self.quest_targets.insert(id, q)),
        };
    }

    /// Remove a component from an entity.
    pub fn remove_component(&mut self, id: EntityId, comp_type: ComponentType) {
        match comp_type {
            ComponentType::Position => drop(self.positions.remove(&id)),
            ComponentType::Velocity => drop(self.velocities.remove(&id)),
            ComponentType::Vibe => drop(self.vibes.remove(&id)),
            ComponentType::Health => drop(self.healths.remove(&id)),
            ComponentType::AgentAI => drop(self.agent_ais.remove(&id)),
            ComponentType::Renderable => drop(self.renderables.remove(&id)),
            ComponentType::Collider => drop(self.colliders.remove(&id)),
            ComponentType::Inventory => drop(self.inventories.remove(&id)),
            ComponentType::Pet => drop(self.pets.remove(&id)),
            ComponentType::Dialogue => drop(self.dialogues.remove(&id)),
            ComponentType::BiomeAffinity => drop(self.biome_affinities.remove(&id)),
            ComponentType::Craftable => drop(self.craftables.remove(&id)),
            ComponentType::QuestTarget => drop(self.quest_targets.remove(&id)),
        };

        self.entities
            .entry(id)
            .and_modify(|mask| mask.remove(comp_type));
    }

    /// Check if an entity has the given component type.
    pub fn has_component(&self, id: EntityId, comp_type: ComponentType) -> bool {
        self.entities
            .get(&id)
            .is_some_and(|mask| mask.has(comp_type))
    }

    /// Get a copy of the component value for an entity, if present.
    pub fn get_component(&self, id: EntityId, comp_type: ComponentType) -> Option<Component> {
        match comp_type {
            ComponentType::Position => {
                self.positions.get(&id).map(|&(x, y, z)| Component::Position(x, y, z))
            }
            ComponentType::Velocity => {
                self.velocities.get(&id).map(|&(x, y, z)| Component::Velocity(x, y, z))
            }
            ComponentType::Vibe => {
                self.vibes.get(&id).map(|&v| Component::Vibe(v))
            }
            ComponentType::Health => self.healths.get(&id).map(|&h| Component::Health(h)),
            ComponentType::AgentAI => self.agent_ais.get(&id).map(|a| Component::AgentAI(a.clone())),
            ComponentType::Renderable => {
                self.renderables.get(&id).map(|r| Component::Renderable(r.clone()))
            }
            ComponentType::Collider => {
                self.colliders.get(&id).map(|c| Component::Collider(c.clone()))
            }
            ComponentType::Inventory => {
                self.inventories.get(&id).map(|i| Component::Inventory(i.clone()))
            }
            ComponentType::Pet => self.pets.get(&id).map(|p| Component::Pet(p.clone())),
            ComponentType::Dialogue => {
                self.dialogues.get(&id).map(|d| Component::Dialogue(d.clone()))
            }
            ComponentType::BiomeAffinity => {
                self.biome_affinities.get(&id).map(|b| Component::BiomeAffinity(b.clone()))
            }
            ComponentType::Craftable => {
                self.craftables.get(&id).map(|c| Component::Craftable(c.clone()))
            }
            ComponentType::QuestTarget => {
                self.quest_targets.get(&id).map(|q| Component::QuestTarget(q.clone()))
            }
        }
    }

    /// Query all entities whose component mask matches the given mask.
    pub fn query(&self, mask: ComponentMask) -> Vec<EntityId> {
        self.entities
            .iter()
            .filter(|(_, entity_mask)| entity_mask.matches(&mask))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Return the next entity ID counter (useful for tests / inspection).
    pub fn next_id(&self) -> u64 {
        self.next_id
    }
}

// ---------------------------------------------------------------------------
// System trait
// ---------------------------------------------------------------------------

/// A system that operates on the world each tick.
pub trait System {
    fn run(&mut self, world: &mut World, tick: u64);
}

// ---------------------------------------------------------------------------
// MovementSystem
// ---------------------------------------------------------------------------

/// Updates position of entities with both Position and Velocity.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, world: &mut World, _tick: u64) {
        let mask =
            ComponentMask::new().with(ComponentType::Position).with(ComponentType::Velocity);
        let ids: Vec<EntityId> = world.query(mask);

        for id in ids {
            if let Some(&(x, y, z)) = world.velocities.get(&id) {
                if let Some(pos) = world.positions.get_mut(&id) {
                    pos.0 += x;
                    pos.1 += y;
                    pos.2 += z;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// VibePropagationSystem
// ---------------------------------------------------------------------------

/// Spreads Vibe to nearby entities that also have Position + Vibe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibePropagationSystem {
    pub radius: f64,
    pub coupling: f64,
}

impl Default for VibePropagationSystem {
    fn default() -> Self {
        Self {
            radius: 10.0,
            coupling: 0.1,
        }
    }
}

impl System for VibePropagationSystem {
    fn run(&mut self, world: &mut World, _tick: u64) {
        let mask = ComponentMask::new()
            .with(ComponentType::Position)
            .with(ComponentType::Vibe);
        let ids: Vec<EntityId> = world.query(mask);

        // Gather positions and vibes
        let mut data: Vec<(EntityId, (f64, f64, f64), f64)> = Vec::with_capacity(ids.len());
        for &id in &ids {
            if let Some(&pos) = world.positions.get(&id) {
                if let Some(&vibe) = world.vibes.get(&id) {
                    data.push((id, pos, vibe));
                }
            }
        }

        let sq_radius = self.radius * self.radius;
        let mut transfers: Vec<(EntityId, f64)> = Vec::new();

        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                let (id_a, pos_a, vibe_a) = data[i];
                let (id_b, pos_b, vibe_b) = data[j];
                let dx = pos_a.0 - pos_b.0;
                let dy = pos_a.1 - pos_b.1;
                let dz = pos_a.2 - pos_b.2;
                let dist_sq = dx * dx + dy * dy + dz * dz;
                if dist_sq <= sq_radius {
                    let diff = vibe_a - vibe_b;
                    let flow = diff * self.coupling;
                    transfers.push((id_a, -flow));
                    transfers.push((id_b, flow));
                }
            }
        }

        for (id, delta) in transfers {
            if let Some(v) = world.vibes.get_mut(&id) {
                *v += delta;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CollisionSystem
// ---------------------------------------------------------------------------

/// Detects overlapping colliders among entities with Position + Collider.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CollisionSystem;

impl CollisionSystem {
    /// Returns pairs of entity IDs whose colliders are overlapping.
    pub fn detect_collisions(&self, world: &World) -> Vec<(EntityId, EntityId)> {
        let mask = ComponentMask::new()
            .with(ComponentType::Position)
            .with(ComponentType::Collider);
        let ids: Vec<EntityId> = world.query(mask);
        let mut collisions = Vec::new();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = ids[i];
                let b = ids[j];
                if let (Some(&pos_a), Some(&pos_b)) =
                    (world.positions.get(&a), world.positions.get(&b))
                {
                    if let (Some(shape_a), Some(shape_b)) =
                        (world.colliders.get(&a), world.colliders.get(&b))
                    {
                        if self.overlap(&pos_a, shape_a, &pos_b, shape_b) {
                            collisions.push((a, b));
                        }
                    }
                }
            }
        }

        collisions
    }

    fn overlap(
        &self,
        pos_a: &(f64, f64, f64),
        shape_a: &ColliderShape,
        pos_b: &(f64, f64, f64),
        shape_b: &ColliderShape,
    ) -> bool {
        match (shape_a, shape_b) {
            (ColliderShape::Point, ColliderShape::Point) => {
                (pos_a.0 - pos_b.0).abs() < f64::EPSILON
                    && (pos_a.1 - pos_b.1).abs() < f64::EPSILON
            }
            (ColliderShape::Circle { r: ra }, ColliderShape::Circle { r: rb }) => {
                let dx = pos_a.0 - pos_b.0;
                let dy = pos_a.1 - pos_b.1;
                let dist_sq = dx * dx + dy * dy;
                let sum_r = ra + rb;
                dist_sq <= sum_r * sum_r
            }
            (ColliderShape::Box { w: w1, h: h1 }, ColliderShape::Box { w: w2, h: h2 }) => {
                let a_half_w = w1 / 2.0;
                let a_half_h = h1 / 2.0;
                let b_half_w = w2 / 2.0;
                let b_half_h = h2 / 2.0;
                let overlap_x = (pos_a.0 - pos_b.0).abs() < a_half_w + b_half_w;
                let overlap_y = (pos_a.1 - pos_b.1).abs() < a_half_h + b_half_h;
                overlap_x && overlap_y
            }
            // Mixed shapes: treat Circle vs Box as AABB-circle
            (ColliderShape::Circle { r }, ColliderShape::Box { w, h })
            | (ColliderShape::Box { w, h }, ColliderShape::Circle { r }) => {
                // Simple AABB check against the circle's bounding box
                let (box_pos, circle_pos, circle_r, bw, bh) = if matches!(shape_a, ColliderShape::Circle { .. })
                {
                    (pos_b, pos_a, *r, *w, *h)
                } else {
                    (pos_a, pos_b, *r, *w, *h)
                };
                let half_w = bw / 2.0;
                let half_h = bh / 2.0;

                let closest_x = (circle_pos.0).clamp(box_pos.0 - half_w, box_pos.0 + half_w);
                let closest_y = (circle_pos.1).clamp(box_pos.1 - half_h, box_pos.1 + half_h);
                let dx = circle_pos.0 - closest_x;
                let dy = circle_pos.1 - closest_y;
                dx * dx + dy * dy <= circle_r * circle_r
            }
            (ColliderShape::Point, _) | (_, ColliderShape::Point) => {
                // Point vs anything: treat Point as small circle
                let r = f64::EPSILON.sqrt();
                let (pt, other_pos, other_shape) = if matches!(shape_a, ColliderShape::Point) {
                    (pos_a, pos_b, shape_b)
                } else {
                    (pos_b, pos_a, shape_a)
                };
                match other_shape {
                    ColliderShape::Circle { r: or } => {
                        let dx = pt.0 - other_pos.0;
                        let dy = pt.1 - other_pos.1;
                        dx * dx + dy * dy <= (r + or) * (r + or)
                    }
                    ColliderShape::Box { w, h } => {
                        let half_w = w / 2.0;
                        let half_h = h / 2.0;
                        let closest_x = pt.0.clamp(other_pos.0 - half_w, other_pos.0 + half_w);
                        let closest_y = pt.1.clamp(other_pos.1 - half_h, other_pos.1 + half_h);
                        let dx = pt.0 - closest_x;
                        let dy = pt.1 - closest_y;
                        dx * dx + dy * dy <= r * r
                    }
                    ColliderShape::Point => {
                        (pt.0 - other_pos.0).abs() < f64::EPSILON
                            && (pt.1 - other_pos.1).abs() < f64::EPSILON
                    }
                }
            }
        }
    }
}

impl System for CollisionSystem {
    fn run(&mut self, world: &mut World, _tick: u64) {
        // Detection is done via detect_collisions, but the System trait
        // just runs the detection. For now we do nothing destructive.
        let _ = self.detect_collisions(world);
    }
}

// ---------------------------------------------------------------------------
// ConservationSystem
// ---------------------------------------------------------------------------

/// Checks that the total Vibe in the world is conserved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationSystem {
    pub error_threshold: f64,
}

impl Default for ConservationSystem {
    fn default() -> Self {
        Self {
            error_threshold: 1e-9,
        }
    }
}

impl ConservationSystem {
    /// Sum all Vibe values and return the total with the number of entities.
    pub fn total_vibe(&self, world: &World) -> f64 {
        world.vibes.values().sum()
    }

    /// Returns `Ok(())` if the conservation error (difference from stored baseline)
    /// is within the threshold, or `Err(delta)` otherwise.
    pub fn check_conservation(&self, world: &World, baseline: f64) -> Result<(), f64> {
        let current = self.total_vibe(world);
        let delta = (current - baseline).abs();
        if delta <= self.error_threshold {
            Ok(())
        } else {
            Err(current - baseline)
        }
    }
}

impl System for ConservationSystem {
    fn run(&mut self, world: &mut World, _tick: u64) {
        let _ = self.total_vibe(world);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // EntityId
    // -----------------------------------------------------------------------

    #[test]
    fn entity_id_newtype() {
        let a = EntityId(1);
        let b = EntityId(1);
        let c = EntityId(2);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn entity_id_copy_clone() {
        let a = EntityId(42);
        let b = a; // Copy
        let c = a; // Copy again
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn entity_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EntityId(1));
        set.insert(EntityId(2));
        set.insert(EntityId(1));
        assert_eq!(set.len(), 2);
    }

    // -----------------------------------------------------------------------
    // ComponentMask
    // -----------------------------------------------------------------------

    #[test]
    fn component_mask_insert_remove_has() {
        let mut mask = ComponentMask::new();
        assert!(!mask.has(ComponentType::Position));
        mask.insert(ComponentType::Position);
        assert!(mask.has(ComponentType::Position));
        mask.remove(ComponentType::Position);
        assert!(!mask.has(ComponentType::Position));
    }

    #[test]
    fn component_mask_multiple_bits() {
        let mut mask = ComponentMask::new()
            .with(ComponentType::Position)
            .with(ComponentType::Vibe)
            .with(ComponentType::Health);
        assert!(mask.has(ComponentType::Position));
        assert!(mask.has(ComponentType::Vibe));
        assert!(mask.has(ComponentType::Health));
        assert!(!mask.has(ComponentType::Velocity));

        mask.remove(ComponentType::Vibe);
        assert!(!mask.has(ComponentType::Vibe));
        assert!(mask.has(ComponentType::Position));
    }

    #[test]
    fn component_mask_matches() {
        let query = ComponentMask::new()
            .with(ComponentType::Position)
            .with(ComponentType::Velocity);
        let entity = ComponentMask::new()
            .with(ComponentType::Position)
            .with(ComponentType::Velocity)
            .with(ComponentType::Vibe);
        assert!(entity.matches(&query));

        let partial = ComponentMask::new().with(ComponentType::Position);
        assert!(!partial.matches(&query));
    }

    // -----------------------------------------------------------------------
    // World: spawn / despawn / add / remove components
    // -----------------------------------------------------------------------

    #[test]
    fn world_spawn_increments_id() {
        let mut world = World::new();
        let id1 = world.spawn();
        let id2 = world.spawn();
        assert_eq!(id1, EntityId(1));
        assert_eq!(id2, EntityId(2));
    }

    #[test]
    fn world_despawn_removes_entity_and_components() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(1.0, 2.0, 3.0));
        world.add_component(id, Component::Vibe(10.0));
        assert!(world.entities.contains_key(&id));
        assert!(world.positions.contains_key(&id));

        world.despawn(id);
        assert!(!world.entities.contains_key(&id));
        assert!(!world.positions.contains_key(&id));
        assert!(!world.vibes.contains_key(&id));
    }

    #[test]
    fn world_despawn_nonexistent_is_noop() {
        let mut world = World::new();
        world.despawn(EntityId(999)); // should not panic
    }

    #[test]
    fn world_add_component_updates_mask() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(0.0, 0.0, 0.0));
        let mask = world.entities.get(&id).unwrap();
        assert!(mask.has(ComponentType::Position));
        assert!(!mask.has(ComponentType::Velocity));
    }

    #[test]
    fn world_remove_component() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Health(100.0));
        assert!(world.has_component(id, ComponentType::Health));
        world.remove_component(id, ComponentType::Health);
        assert!(!world.has_component(id, ComponentType::Health));
        assert!(!world.healths.contains_key(&id));
    }

    #[test]
    fn world_has_component_nonexistent_entity() {
        let world = World::new();
        assert!(!world.has_component(EntityId(999), ComponentType::Position));
    }

    #[test]
    fn world_get_component_nonexistent() {
        let world = World::new();
        assert!(world.get_component(EntityId(1), ComponentType::Position).is_none());
    }

    #[test]
    fn world_get_component_returns_value() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Velocity(0.5, 1.0, -0.5));
        let comp = world.get_component(id, ComponentType::Velocity);
        assert!(comp.is_some());
        match comp.unwrap() {
            Component::Velocity(x, y, z) => {
                assert!((x - 0.5).abs() < 1e-10);
                assert!((y - 1.0).abs() < 1e-10);
                assert!((z + 0.5).abs() < 1e-10);
            }
            _ => panic!("wrong component type"),
        }
    }

    // -----------------------------------------------------------------------
    // World: query
    // -----------------------------------------------------------------------

    #[test]
    fn world_query_matches_entities() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();

        world.add_component(e1, Component::Position(0.0, 0.0, 0.0));
        world.add_component(e1, Component::Velocity(1.0, 0.0, 0.0));
        world.add_component(e2, Component::Position(5.0, 5.0, 5.0));
        world.add_component(e3, Component::Position(0.0, 0.0, 0.0));
        world.add_component(e3, Component::Velocity(0.0, 1.0, 0.0));

        let query_mask =
            ComponentMask::new().with(ComponentType::Position).with(ComponentType::Velocity);
        let results = world.query(query_mask);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&e1));
        assert!(results.contains(&e3));
        assert!(!results.contains(&e2));
    }

    #[test]
    fn world_query_empty_when_no_match() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(1.0, 2.0, 3.0));
        let results = world.query(ComponentMask::new().with(ComponentType::AgentAI));
        assert!(results.is_empty());
    }

    // -----------------------------------------------------------------------
    // ComponentType enum ordering
    // -----------------------------------------------------------------------

    #[test]
    fn component_type_discriminants() {
        assert_eq!(ComponentType::Position as u8, 0);
        assert_eq!(ComponentType::Velocity as u8, 1);
        assert_eq!(ComponentType::Vibe as u8, 2);
        assert_eq!(ComponentType::Health as u8, 3);
        assert_eq!(ComponentType::AgentAI as u8, 4);
        assert_eq!(ComponentType::Renderable as u8, 5);
        assert_eq!(ComponentType::Collider as u8, 6);
        assert_eq!(ComponentType::Inventory as u8, 7);
        assert_eq!(ComponentType::Pet as u8, 8);
        assert_eq!(ComponentType::Dialogue as u8, 9);
        assert_eq!(ComponentType::BiomeAffinity as u8, 10);
        assert_eq!(ComponentType::Craftable as u8, 11);
        assert_eq!(ComponentType::QuestTarget as u8, 12);
    }

    // -----------------------------------------------------------------------
    // MovementSystem
    // -----------------------------------------------------------------------

    #[test]
    fn movement_system_updates_position() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(10.0, 20.0, 30.0));
        world.add_component(id, Component::Velocity(1.0, 2.0, 3.0));

        let mut system = MovementSystem;
        system.run(&mut world, 1);

        let pos = world.positions.get(&id).unwrap();
        assert!((pos.0 - 11.0).abs() < 1e-10);
        assert!((pos.1 - 22.0).abs() < 1e-10);
        assert!((pos.2 - 33.0).abs() < 1e-10);
    }

    #[test]
    fn movement_system_ignores_entities_without_velocity() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(0.0, 0.0, 0.0));
        // No velocity

        let mut system = MovementSystem;
        system.run(&mut world, 1);

        let pos = world.positions.get(&id).unwrap();
        assert!((pos.0).abs() < 1e-10);
        assert!((pos.1).abs() < 1e-10);
        assert!((pos.2).abs() < 1e-10);
    }

    // -----------------------------------------------------------------------
    // VibePropagationSystem
    // -----------------------------------------------------------------------

    #[test]
    fn vibe_propagation_system_spreads_vibe() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Vibe(100.0));
        world.add_component(b, Component::Position(1.0, 0.0, 0.0));
        world.add_component(b, Component::Vibe(0.0));

        let mut system = VibePropagationSystem {
            radius: 10.0,
            coupling: 0.5,
        };
        system.run(&mut world, 1);

        let vibe_a = world.vibes.get(&a).unwrap();
        let vibe_b = world.vibes.get(&b).unwrap();
        // Vibe transfers from a (100) to b (0): delta = (100-0)*0.5 = 50
        assert!((*vibe_a - 50.0).abs() < 1e-10);
        assert!((*vibe_b - 50.0).abs() < 1e-10);
    }

    #[test]
    fn vibe_propagation_no_spread_outside_radius() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Vibe(100.0));
        world.add_component(b, Component::Position(100.0, 0.0, 0.0));
        world.add_component(b, Component::Vibe(0.0));

        let mut system = VibePropagationSystem::default();
        system.run(&mut world, 1);

        let vibe_a = world.vibes.get(&a).unwrap();
        let vibe_b = world.vibes.get(&b).unwrap();
        assert!((*vibe_a - 100.0).abs() < 1e-10);
        assert!((*vibe_b - 0.0).abs() < 1e-10);
    }

    // -----------------------------------------------------------------------
    // CollisionSystem
    // -----------------------------------------------------------------------

    #[test]
    fn collision_detects_circle_overlap() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Circle { r: 5.0 }));
        world.add_component(b, Component::Position(3.0, 4.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Circle { r: 5.0 }));

        let system = CollisionSystem;
        let collisions = system.detect_collisions(&world);
        // distance = 5, sum r = 10 -> overlap
        assert_eq!(collisions.len(), 1);
        assert!(
            collisions[0] == (a, b) || collisions[0] == (b, a)
        );
    }

    #[test]
    fn collision_no_overlap_when_far_apart() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Circle { r: 1.0 }));
        world.add_component(b, Component::Position(10.0, 10.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Circle { r: 1.0 }));

        let system = CollisionSystem;
        let collisions = system.detect_collisions(&world);
        assert!(collisions.is_empty());
    }

    #[test]
    fn collision_point_and_box() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Box { w: 4.0, h: 4.0 }));
        world.add_component(b, Component::Position(1.0, 1.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Point));

        let system = CollisionSystem;
        let collisions = system.detect_collisions(&world);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn collision_circle_and_box() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Circle { r: 3.0 }));
        world.add_component(b, Component::Position(4.0, 0.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Box { w: 2.0, h: 2.0 }));

        let system = CollisionSystem;
        let collisions = system.detect_collisions(&world);
        // Box centered at (4,0) half-width 1, so left edge at 3. Circle radius 3 centered at (0,0)
        // touches x=3: distance from circle center to closest point on box = 1. 1 <= 3 -> overlap
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn collision_box_no_overlap() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Box { w: 2.0, h: 2.0 }));
        world.add_component(b, Component::Position(10.0, 10.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Box { w: 2.0, h: 2.0 }));

        let system = CollisionSystem;
        let collisions = system.detect_collisions(&world);
        assert!(collisions.is_empty());
    }

    // -----------------------------------------------------------------------
    // ConservationSystem
    // -----------------------------------------------------------------------

    #[test]
    fn conservation_system_totals_vibes() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        let c = world.spawn();
        world.add_component(a, Component::Vibe(10.0));
        world.add_component(b, Component::Vibe(20.0));
        world.add_component(c, Component::Vibe(30.0));

        let system = ConservationSystem::default();
        let total = system.total_vibe(&world);
        assert!((total - 60.0).abs() < 1e-10);
    }

    #[test]
    fn conservation_system_within_threshold() {
        let mut world = World::new();
        let a = world.spawn();
        world.add_component(a, Component::Vibe(100.0));

        let system = ConservationSystem {
            error_threshold: 1e-9,
        };
        let baseline = system.total_vibe(&world);

        // Add and remove vibes to verify conservation
        let b = world.spawn();
        world.add_component(b, Component::Vibe(50.0));
        // total is now 150
        let result = system.check_conservation(&world, baseline);
        assert!(result.is_err());
        assert!((result.unwrap_err() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn conservation_system_exact() {
        let mut world = World::new();
        let a = world.spawn();
        world.add_component(a, Component::Vibe(42.0));

        let system = ConservationSystem {
            error_threshold: 1e-9,
        };
        let baseline = system.total_vibe(&world);
        let result = system.check_conservation(&world, baseline);
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // Component type defaults / construction
    // -----------------------------------------------------------------------

    #[test]
    fn agent_state_default() {
        let state = AgentState::default();
        assert_eq!(state.program_counter, 0);
        assert!(state.stack.is_empty());
        assert_eq!(state.state, "idle");
    }

    #[test]
    fn render_info_default() {
        let info = RenderInfo::default();
        assert_eq!(info.sprite, "default");
        assert!((info.scale - 1.0).abs() < 1e-10);
        assert_eq!(info.layer, 0);
    }

    #[test]
    fn inventory_default() {
        let inv = InventoryData::default();
        assert!(inv.items.is_empty());
        assert_eq!(inv.capacity, 10);
    }

    #[test]
    fn pet_data_default() {
        let pet = PetData::default();
        assert_eq!(pet.species, "unknown");
        assert!((pet.happiness - 50.0).abs() < 1e-10);
        assert_eq!(pet.evolution_stage, 1);
    }

    #[test]
    fn dialogue_state_default() {
        let d = DialogueState::default();
        assert_eq!(d.current_node, "start");
        assert!((d.mood - 0.0).abs() < 1e-10);
        assert!(d.history.is_empty());
    }

    #[test]
    fn quest_data_default() {
        let q = QuestData::default();
        assert!(q.quest_id.is_empty());
        assert!((q.progress - 0.0).abs() < 1e-10);
        assert!(!q.completed);
    }

    #[test]
    fn component_type_roundtrip() {
        let cases: Vec<Component> = vec![
            Component::Position(1.0, 2.0, 3.0),
            Component::Velocity(4.0, 5.0, 6.0),
            Component::Vibe(7.0),
            Component::Health(8.0),
            Component::AgentAI(AgentState::default()),
            Component::Renderable(RenderInfo::default()),
            Component::Collider(ColliderShape::Circle { r: 1.0 }),
            Component::Inventory(InventoryData {
                items: [("rock".into(), 3)].into(),
                capacity: 20,
            }),
            Component::Pet(PetData {
                species: "cat".into(),
                happiness: 90.0,
                evolution_stage: 2,
            }),
            Component::Dialogue(DialogueState {
                current_node: "greeting".into(),
                mood: 0.5,
                history: vec!["hello".into()],
            }),
            Component::BiomeAffinity("forest".into()),
            Component::Craftable(CraftRecipe {
                inputs: [("wood".into(), 3)].into(),
                output: "plank".into(),
                tick_cost: 5,
            }),
            Component::QuestTarget(QuestData {
                quest_id: "q001".into(),
                progress: 0.5,
                completed: false,
            }),
        ];

        for comp in &cases {
            let ty = comp.component_type();
            match comp {
                Component::Position(..) => assert_eq!(ty, ComponentType::Position),
                Component::Velocity(..) => assert_eq!(ty, ComponentType::Velocity),
                Component::Vibe(..) => assert_eq!(ty, ComponentType::Vibe),
                Component::Health(..) => assert_eq!(ty, ComponentType::Health),
                Component::AgentAI(..) => assert_eq!(ty, ComponentType::AgentAI),
                Component::Renderable(..) => assert_eq!(ty, ComponentType::Renderable),
                Component::Collider(..) => assert_eq!(ty, ComponentType::Collider),
                Component::Inventory(..) => assert_eq!(ty, ComponentType::Inventory),
                Component::Pet(..) => assert_eq!(ty, ComponentType::Pet),
                Component::Dialogue(..) => assert_eq!(ty, ComponentType::Dialogue),
                Component::BiomeAffinity(..) => assert_eq!(ty, ComponentType::BiomeAffinity),
                Component::Craftable(..) => assert_eq!(ty, ComponentType::Craftable),
                Component::QuestTarget(..) => assert_eq!(ty, ComponentType::QuestTarget),
            }
        }
    }

    // -----------------------------------------------------------------------
    // Serde roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn entity_id_serde_roundtrip() {
        let e = EntityId(42);
        let json = serde_json::to_string(&e).unwrap();
        let back: EntityId = serde_json::from_str(&json).unwrap();
        assert_eq!(e, back);
    }

    #[test]
    fn component_serde_roundtrip() {
        let comp = Component::Position(1.0, 2.0, 3.0);
        let json = serde_json::to_string(&comp).unwrap();
        let back: Component = serde_json::from_str(&json).unwrap();
        match back {
            Component::Position(x, y, z) => {
                assert!((x - 1.0).abs() < 1e-10);
                assert!((y - 2.0).abs() < 1e-10);
                assert!((z - 3.0).abs() < 1e-10);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn world_serde_roundtrip() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Velocity(1.0, 2.0, 3.0));
        world.add_component(b, Component::Vibe(99.0));

        let json = serde_json::to_string(&world).unwrap();
        let mut restored: World = serde_json::from_str(&json).unwrap();

        assert!(restored.has_component(a, ComponentType::Position));
        assert!(restored.has_component(a, ComponentType::Velocity));
        assert!(!restored.has_component(a, ComponentType::Vibe));
        assert!(restored.has_component(b, ComponentType::Vibe));

        // Verify movement still works after deserialisation
        let mut system = MovementSystem;
        system.run(&mut restored, 1);
        let pos = restored.positions.get(&a).unwrap();
        assert!((pos.0 - 1.0).abs() < 1e-10);
        assert!((pos.1 - 2.0).abs() < 1e-10);
        assert!((pos.2 - 3.0).abs() < 1e-10);
    }

    // -----------------------------------------------------------------------
    // ColliderShape constructors
    // -----------------------------------------------------------------------

    #[test]
    fn collider_shapes() {
        let box_s = ColliderShape::Box { w: 10.0, h: 20.0 };
        if let ColliderShape::Box { w, h } = box_s {
            assert!((w - 10.0).abs() < 1e-10);
            assert!((h - 20.0).abs() < 1e-10);
        } else {
            panic!("not box");
        }

        let circle = ColliderShape::Circle { r: 5.0 };
        if let ColliderShape::Circle { r } = circle {
            assert!((r - 5.0).abs() < 1e-10);
        } else {
            panic!("not circle");
        }

        assert!(matches!(ColliderShape::Point, ColliderShape::Point));
    }

    // -----------------------------------------------------------------------
    // System trait implementations
    // -----------------------------------------------------------------------

    #[test]
    fn system_trait_movement() {
        let mut sys: Box<dyn System> = Box::new(MovementSystem);
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Position(5.0, 5.0, 5.0));
        world.add_component(id, Component::Velocity(1.0, 2.0, 3.0));
        sys.run(&mut world, 0);
        let pos = world.positions.get(&id).unwrap();
        assert!((pos.0 - 6.0).abs() < 1e-10);
        assert!((pos.1 - 7.0).abs() < 1e-10);
        assert!((pos.2 - 8.0).abs() < 1e-10);
    }

    #[test]
    fn system_trait_vibe_propagation() {
        let mut sys: Box<dyn System> = Box::new(VibePropagationSystem {
            radius: 5.0,
            coupling: 0.2,
        });
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Vibe(100.0));
        world.add_component(b, Component::Position(2.0, 0.0, 0.0));
        world.add_component(b, Component::Vibe(0.0));

        sys.run(&mut world, 1);
        let vibe_a = world.vibes.get(&a).unwrap();
        let vibe_b = world.vibes.get(&b).unwrap();
        assert!((*vibe_a - 80.0).abs() < 1e-10);
        assert!((*vibe_b - 20.0).abs() < 1e-10);
    }

    #[test]
    fn system_trait_collision() {
        let mut sys: Box<dyn System> = Box::new(CollisionSystem);
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Collider(ColliderShape::Circle { r: 2.0 }));
        world.add_component(b, Component::Position(1.0, 0.0, 0.0));
        world.add_component(b, Component::Collider(ColliderShape::Circle { r: 2.0 }));

        // CollisionSystem::run doesn't collect results, but doesn't crash
        sys.run(&mut world, 1);
        // Verify world still in good state
        assert!(world.entities.contains_key(&a));
        assert!(world.entities.contains_key(&b));
    }

    // -----------------------------------------------------------------------
    // CraftRecipe and component interaction
    // -----------------------------------------------------------------------

    #[test]
    fn craft_recipe_fields() {
        let recipe = CraftRecipe {
            inputs: [("iron_ore".into(), 3)].into(),
            output: "iron_ingot".into(),
            tick_cost: 10,
        };
        assert_eq!(recipe.inputs.get("iron_ore"), Some(&3));
        assert_eq!(recipe.output, "iron_ingot");
        assert_eq!(recipe.tick_cost, 10);
    }

    // -----------------------------------------------------------------------
    // Inventory / Pet / Dialogue / Quest full roundtrips
    // -----------------------------------------------------------------------

    #[test]
    fn inventory_add_items() {
        let mut inv = InventoryData::default();
        inv.items.insert("apple".into(), 5);
        inv.items.insert("bread".into(), 2);
        assert_eq!(inv.items.len(), 2);
        assert_eq!(*inv.items.get("apple").unwrap(), 5);
    }

    #[test]
    fn pet_data_happiness() {
        let pet = PetData {
            happiness: 100.0,
            evolution_stage: 3,
            ..PetData::default()
        };
        assert_eq!(pet.species, "unknown");
        assert!((pet.happiness - 100.0).abs() < 1e-10);
        assert_eq!(pet.evolution_stage, 3);
        assert_eq!(pet.species, "unknown");
    }

    #[test]
    fn dialogue_history() {
        let d = DialogueState {
            mood: -1.0,
            current_node: "farewell".into(),
            history: vec!["hello".into(), "goodbye".into()],
        };
        assert_eq!(d.history.len(), 2);
        assert_eq!(d.current_node, "farewell");
        assert!(!d.history.is_empty());
        assert!((d.mood - -1.0).abs() < 1e-10);
    }

    #[test]
    fn quest_target_progress() {
        let mut quest = QuestData {
            quest_id: "q002".into(),
            progress: 0.0,
            completed: false,
        };
        quest.progress = 1.0;
        quest.completed = true;
        assert!(quest.completed);
        assert!((quest.progress - 1.0).abs() < 1e-10);
    }

    // -----------------------------------------------------------------------
    // BiomeAffinity string component
    // -----------------------------------------------------------------------

    #[test]
    fn biome_affinity() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::BiomeAffinity("tundra".into()));
        assert!(world.has_component(id, ComponentType::BiomeAffinity));
        let comp = world.get_component(id, ComponentType::BiomeAffinity);
        assert!(comp.is_some());
        match comp.unwrap() {
            Component::BiomeAffinity(b) => assert_eq!(b, "tundra"),
            _ => panic!("wrong type"),
        }
    }

    // -----------------------------------------------------------------------
    // World next_id
    // -----------------------------------------------------------------------

    #[test]
    fn world_next_id_progression() {
        let mut world = World::new();
        assert_eq!(world.next_id(), 1);
        world.spawn();
        assert_eq!(world.next_id(), 2);
        world.spawn();
        world.spawn();
        assert_eq!(world.next_id(), 4);
    }

    // -----------------------------------------------------------------------
    // Multiple systems run in sequence
    // -----------------------------------------------------------------------

    #[test]
    fn multiple_systems_sequence() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();

        world.add_component(a, Component::Position(0.0, 0.0, 0.0));
        world.add_component(a, Component::Velocity(1.0, 1.0, 1.0));
        world.add_component(a, Component::Vibe(50.0));
        world.add_component(b, Component::Position(5.0, 0.0, 0.0));
        world.add_component(b, Component::Vibe(0.0));

        let mut systems: Vec<Box<dyn System>> = vec![
            Box::new(MovementSystem),
            Box::new(VibePropagationSystem {
                radius: 10.0,
                coupling: 0.1,
            }),
        ];

        for (i, sys) in systems.iter_mut().enumerate() {
            sys.run(&mut world, i as u64);
        }

        // a moved by (1,1,1)
        let pos_a = world.positions.get(&a).unwrap();
        assert!((pos_a.0 - 1.0).abs() < 1e-10);
        assert!((pos_a.1 - 1.0).abs() < 1e-10);
        assert!((pos_a.2 - 1.0).abs() < 1e-10);

        // a's vibe partially moved to b: delta = (50-0)*0.1 = 5
        let vibe_a = world.vibes.get(&a).unwrap();
        let vibe_b = world.vibes.get(&b).unwrap();
        assert!((*vibe_a - 45.0).abs() < 1e-10);
        assert!((*vibe_b - 5.0).abs() < 1e-10);
    }

    // -----------------------------------------------------------------------
    // Despawn + re-spawn
    // -----------------------------------------------------------------------

    #[test]
    fn respawn_after_despawn() {
        let mut world = World::new();
        let id = world.spawn();
        world.add_component(id, Component::Health(100.0));
        world.despawn(id);
        let id2 = world.spawn();
        assert_eq!(id2, EntityId(2));
        world.add_component(id2, Component::Health(50.0));
        assert_eq!(world.healths[&id2], 50.0);
        assert!(!world.healths.contains_key(&id));
    }

    // -----------------------------------------------------------------------
    // 30th+ test: verify that has_component works after remove on non-existing entity
    // -----------------------------------------------------------------------

    #[test]
    fn remove_component_from_entity_without_it_is_safe() {
        let mut world = World::new();
        let id = world.spawn();
        // No component added yet
        world.remove_component(id, ComponentType::Position); // should not panic
        assert!(!world.has_component(id, ComponentType::Position));
    }

    #[test]
    fn default_world_is_empty() {
        let world = World::default();
        assert_eq!(world.next_id(), 1);
        assert!(world.entities.is_empty());
        assert!(world.positions.is_empty());
        assert!(world.velocities.is_empty());
        assert!(world.vibes.is_empty());
        assert!(world.healths.is_empty());
        assert!(world.agent_ais.is_empty());
        assert!(world.renderables.is_empty());
        assert!(world.colliders.is_empty());
        assert!(world.inventories.is_empty());
        assert!(world.pets.is_empty());
        assert!(world.dialogues.is_empty());
        assert!(world.biome_affinities.is_empty());
        assert!(world.craftables.is_empty());
        assert!(world.quest_targets.is_empty());
    }
}
