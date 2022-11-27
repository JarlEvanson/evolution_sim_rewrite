pub const SENSOR_COUNT: usize = Sensor::NumSensory as usize;
pub const ACTION_COUNT: usize = Action::NumAction as usize;

const INITIAL_VALUE: f32 = 0.5;

/// I -> Indivual Data
/// W -> Environment Data

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sensor {
    LocX = 0,        // I distance from left edge
    LocY,            // I distance from bottom
    BoundaryDistX,   // I X distance to nearest edge of world
    BoundaryDistY,   // I distance to nearest edge of world
    BoundaryDist,    // I Y distance to nearest edge of world
    GeneticSimFwd,   // I genetic similarity forward
    LastMoveDirX,    // I +- amount of X movement in last movement
    LastMoveDirY,    // I +- amount of Y movement in last movement
    LongProbePopFwd, // W long look for population forward
    LongProbeBarFwd, // W long look for barriers forward
    Population,      // W population density in neighborhood
    PopulationFwd,   // W population density in the forward-reverse axis
    PopulationLr,    // W population density in the left-right axis
    Oscillator,      // I oscillator +-value
    Age,             // I
    BarrierFwd,      // W neighborhood barrier distance forward-reverse axis
    BarrierLr,       // W neighborhood barrier distance left-right axis
    Random,          //   random sensor value, uniform distribution
    Signal,          // W strength of signal0 in neighborhood
    SignalFwd,       // W strength of signal0 in the forward-reverse axis
    SignalLr,        // W strength of signal0 in the left-right axis
    NumSensory,      // <<------------------ END OF ACTIVE SENSES MARKER
}

/// I -> Indivual Effects
/// W -> Environment Effects

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    MoveX = 0,         // W +- X component of movement
    MoveY,             // W +- Y component of movement
    MoveForward,       // W continue last direction
    MoveRl,            // W +- component of movement
    MoveRandom,        // W
    SetOscillator,     // I
    SetLongProbeDist,  // I
    SetResponsiveness, // I
    EmitSignal,        // W
    MoveEast,          // W
    MoveWest,          // W
    MoveNorth,         // W
    MoveSouth,         // W
    MoveLeft,          // W
    MoveRight,         // W
    MoveReverse,       // W
    KillForward,       // W
    NumAction,
}
