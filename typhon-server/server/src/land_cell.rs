use uuid::Uuid;

enum Biome {}
enum Terrain {}

struct LandCell {
    id: Uuid,
    biome: Biome,
    terrain: Terrain,
}
