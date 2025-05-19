/// A file containing an enum
/// that include a list of commonly used ElementIds.
///
/// These element id's are from the Element table
/// located in the NHTSA local database and used
/// to retrieve specific information about an ELEMENT
/// from the Patterns table alongside a key and VinSchemaId.
use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u16)] // Max Element id is 203 - Future proof with u16
pub enum ElementId {
    BodyClass = 5,
    ManufacturerCountry = 8,
    EngineCylinderCount = 9,
    DriveType = 15,
    EngineDisplacement = 13,
    VehicleDoorCount = 14,
    EngineModel = 18,
    FuelType = 24,
    VehicleWeightRating = 25,
    VehicleModel = 28,
    PlantCity = 31,
    NumberOfSeats = 33,
    SteeringLocation = 36,
    TransmissionStyle = 37,
    Trim = 38,
    Windows = 40,
    AxleCount = 41,
    BrakeSystem = 42,
    AirbagLocationsCurtain = 55,
    AirbagLocationsSeatCushion = 56,
    NumberOfRows = 61,
    ValveTrainDesign = 62,
    TransmissionSpeeds = 63,
    AirbagLocationsFront = 65,
    FuelDeliveryType = 67,
    AirbagLocationsKnee = 69,
    PlantCountry = 75,
    PlantCompanyName = 76,
    PlantState = 77,
    SeatbeltType = 79,
    ABS = 86,
    ElectronicStabilityControl = 99,
    TractionControl = 100,
    BackupCamera = 104,
    AirbagLocationsSide = 107,
    WheelSizeFront = 119,
    WheelSizeRear = 120,
    TopSpeedMPH = 139,
    EngineManufacturer = 146,
    VehicleBasePrice = 136,
    HasTurbo = 135,
    DynamicBrakeSupport = 170,
    ACN = 174,
    AutoReverseSystem = 172,
    DaytimeRunningLight = 177,
    SemiAutoHeadlampBeamSwitching = 179,
    AdaptiveDrivingBeam = 180,
    KeylessIgnition = 176,
}

impl ElementId {
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
}
