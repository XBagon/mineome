use super::criteria_fragment::*;
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "condition", rename_all = "snake_case")]
pub enum Predicate {
    Alternative {
        terms: Vec<Predicate>,
    },
    BlockStateProperty {
        block: NamespacedId,
        properties: HashMap<String, String>,
    },
    DamageSourceProperties {
        predicate: DamageTypeCriteriaFragment,
    },
    EntityProperties {
        entity: EntityPredicateSubject,
        predicate: EntityCriteriaFragment,
    },
    EntityScores {
        entity: EntityPredicateSubject,
        scores: HashMap<String, RangeOrNumber<i32>>,
    },
    Inverted {
        term: Box<Predicate>,
    },
    KilledByPlayer {
        inverted: bool,
    },
    #[serde(rename_all = "camelCase")]
    LocationCheck {
        offset_x: i32,
        offset_y: i32,
        offset_z: i32,
        predicate: LocationCriteriaFragment,
    },
    MatchTool {
        predicate: ItemCriteriaFragment,
    },
    RandomChance {
        chance: f32,
    },
    RandomChanceWithLooting {
        chance: f32,
        looting_multiplier: i32,
    },
    Reference {
        name: NamespacedId,
    },
    SurvivesExplosion,
    TableBonus {
        enchantment: i32,
        chances: Vec<f32>,
    },
    TimeCheck {
        value: RangeOrNumber<i32>,
        period: Option<i32>,
    },
    WeatherCheck {
        raining: Option<bool>,
        thundering: Option<bool>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityPredicateSubject {
    This,
    Killer,
    KillerPlayer,
}
