/// A file containing an enum
/// that include a list of commonly used ElementIds.
///
/// These element id's are from the Element table
/// located in the NHTSA local database and used
/// to retrieve specific information about an ELEMENT
/// from the Patterns table alongside a key and VinSchemaId.

#[derive(Copy, Clone)]
pub enum ElementId {
    EngineModel = 18,
    EngineManufacturer = 146,
    EngineDisplacement = 13,
    EngineCylinderCount = 9,
    ValveTrainDesign = 62,
    HasTurbo = 135,
    FuelType = 24,
    FuelDeliveryType = 67,
    VehicleDoorCount = 14,
    VehicleModel = 28,

    ManufacturerCountry = 8,
    PlantCity = 31,
    NumberOfSeats = 33,
    Windows = 40,
    BodyClass = 5,
    DriveType = 15,
}

impl ElementId {
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
}
