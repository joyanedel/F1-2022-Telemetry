/* PACKET HEADER */

// Packet Header
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketHeader {
    pub m_packetFormat: u16, // 2022
    pub m_gameMajorVersion: u8, // Game major version - "X.00"
    pub m_gameMinorVersion: u8, // Game minor version - "1.XX"
    pub m_packetVersion: u8, // Version of this packet type, all start from 1
    pub m_packetId: u8, // Identifier for the packet type, see below
    pub m_sessionUID: u64, // Unique identifier for the session
    pub m_sessionTime: f32, // Session timestamp
    pub m_frameIdentifier: u32, // Identifier for the frame the data was retrieved on
    pub m_playerCarIndex: u8, // Index of player's car in the array
    pub m_secondaryPlayerCarIndex: u8, // Index of secondary player's car in the array (splitscreen) // 255 if no second player
}

/* 
    PACKET ID
    Packet Name    Value    Description
    ------------------------------
    Motion          0        Data about all cars on track
    Session         1        Details about the current session
    Lap Data        2        Timings for all cars on track
    Event           3        Various notable events that happen during a session
    Participants    4        List of participants in the session, mostly relevant for multiplayer
    Car Setups      5        Packet detailing the car setups for each vehicle in the session
    Car Telemetry   6        Telemetry data for all cars
    Car Status      7        Status data for all cars such as damage
    Final Classification 8  Final classification confirmation of a session
    Lobby Info      9        Lobby information for a multiplayer session
    Car Damage      10       Damage status of all cars
    Session History 11       Session history data
*/

/* MOTION PACKET */

// Car Motion Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarMotionData {
    pub m_worldPositionX: f32, // World space X position
    pub m_worldPositionY: f32, // World space Y position
    pub m_worldPositionZ: f32, // World space Z position
    pub m_worldVelocityX: f32, // Velocity in world space X
    pub m_worldVelocityY: f32, // Velocity in world space Y
    pub m_worldVelocityZ: f32, // Velocity in world space Z
    pub m_worldForwardDirX: i16, // World space forward X direction (normalised)
    pub m_worldForwardDirY: i16, // World space forward Y direction (normalised)
    pub m_worldForwardDirZ: i16, // World space forward Z direction (normalised)
    pub m_worldRightDirX: i16, // World space right X direction (normalised)
    pub m_worldRightDirY: i16, // World space right Y direction (normalised)
    pub m_worldRightDirZ: i16, // World space right Z direction (normalised)
    pub m_gForceLateral: f32, // Lateral G-Force component
    pub m_gForceLongitudinal: f32, // Longitudinal G-Force component
    pub m_gForceVertical: f32, // Vertical G-Force component
    pub m_yaw: f32, // Yaw angle in radians
    pub m_pitch: f32, // Pitch angle in radians
    pub m_roll: f32, // Roll angle in radians
}

// Packet Motion Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketMotionData {
    pub m_header: PacketHeader, // Header
    pub m_carMotionData: [CarMotionData; 22], // Data for all cars on track
    pub m_suspensionPosition: [f32; 4], // Note: All wheel arrays have the following order:
    pub m_suspensionVelocity: [f32; 4], // RL, RR, FL, FR
    pub m_suspensionAcceleration: [f32; 4], // RL, RR, FL, FR
    pub m_wheelSpeed: [f32; 4], // Speed of each wheel
    pub m_wheelSlip: [f32; 4], // Slip ratio for each wheel
    pub m_localVelocityX: f32, // Velocity in local space
    pub m_localVelocityY: f32, // Velocity in local space
    pub m_localVelocityZ: f32, // Velocity in local space
    pub m_angularVelocityX: f32, // Angular velocity x-component
    pub m_angularVelocityY: f32, // Angular velocity y-component
    pub m_angularVelocityZ: f32, // Angular velocity z-component
    pub m_angularAccelerationX: f32, // Angular velocity x-component
    pub m_angularAccelerationY: f32, // Angular velocity y-component
    pub m_angularAccelerationZ: f32, // Angular velocity z-component
    pub m_frontWheelsAngle: f32, // Current front wheels angle in radians
}

/* SESSION PACKET */
// Marshal Zone
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarshalZone {
    pub m_zoneStart: f32,
    pub m_zoneFlag: i8,
}

// Weather Forecast Sample
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeatherForecastSample {
    pub m_sessionType: u8,
    pub m_timeOffset: u8,
    pub m_weather: u8,
    pub m_trackTemperature: i8,
    pub m_trackTemperatureChange: i8,
    pub m_airTemperature: i8,
    pub m_airTemperatureChange: i8,
    pub m_rainPercentage: u8,
}

// Packet Session Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketSessionData {
    pub m_header: PacketHeader,
    pub m_weather: u8,
    pub m_trackTemperature: i8,
    pub m_airTemperature: i8,
    pub m_totalLaps: u8,
    pub m_trackLength: u16,
    pub m_sessionType: u8,
    pub m_trackId: i8,
    pub m_formula: u8,
    pub m_sessionTimeLeft: u16,
    pub m_sessionDuration: u16,
    pub m_pitSpeedLimit: u8,
    pub m_gamePaused: u8,
    pub m_isSpectating: u8,
    pub m_spectatorCarIndex: u8,
    pub m_sliProNativeSupport: u8,
    pub m_numMarshalZones: u8,
    pub m_marshalZones: [MarshalZone; 21],
    pub m_safetyCarStatus: u8,
    pub m_networkGame: u8,
    pub m_numWeatherForecastSamples: u8,
    pub m_weatherForecastSamples: [WeatherForecastSample; 56],
    pub m_forecastAccuracy: u8,
    pub m_aiDifficulty: u8,
    pub m_seasonLinkIdentifier: u32,
    pub m_weekendLinkIdentifier: u32,
    pub m_sessionLinkIdentifier: u32,
    pub m_pitStopWindowIdealLap: u8,
    pub m_pitStopWindowLatestLap: u8,
    pub m_pitStopRejoinPosition: u8,
    pub m_steeringAssist: u8,
    pub m_brakingAssist: u8,
    pub m_gearboxAssist: u8,
    pub m_pitAssist: u8,
    pub m_pitReleaseAssist: u8,
    pub m_ersAssist: u8,
    pub m_drsAssist: u8,
    pub m_dynamicRacingLine: u8,
    pub m_dynamicRacingLineType: u8,
    pub m_gameMode: u8,
    pub m_ruleSet: u8,
    pub m_timeOfDay: u32,
    pub m_sessionLength: u8,
}

/* LAP DATA PACKET */
// Lap Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LapData {
    pub m_lastLapTimeInMS: u32,
    pub m_currentLapTimeInMS: u32,
    pub m_sector1TimeInMS: u16,
    pub m_sector2TimeInMS: u16,
    pub m_lapDistance: f32,
    pub m_totalDistance: f32,
    pub m_safetyCarDelta: f32,
    pub m_carPosition: u8,
    pub m_currentLapNum: u8,
    pub m_pitStatus: u8,
    pub m_numPitStops: u8,
    pub m_sector: u8,
    pub m_currentLapInvalid: u8,
    pub m_penalties: u8,
    pub m_warnings: u8,
    pub m_numUnservedDriveThroughPens: u8,
    pub m_numUnservedStopGoPens: u8,
    pub m_gridPosition: u8,
    pub m_driverStatus: u8,
    pub m_resultStatus: u8,
    pub m_pitLaneTimerActive: u8,
    pub m_pitLaneTimeInLaneInMS: u16,
    pub m_pitStopTimerInMS: u16,
    pub m_pitStopShouldServePen: u8,
}

// Packet Lap Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketLapData {
    pub m_header: PacketHeader,
    pub m_lapData: [LapData; 22],

    pub m_timeTrialPBCarIdx: u8,
    pub m_timeTrialRivalCarIdx: u8,
}

/* EVENT PACKET */
// !Fastest Lap
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FastestLap {
    pub vehicleIdx: u8,
    pub lapTime: f32,
}

// !Retirement
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Retirement {
    pub vehicleIdx: u8,
}

// !TeamMateInPits
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TeamMateInPits {
    pub vehicleIdx: u8,
}

// !RaceWinner
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RaceWinner {
    pub vehicleIdx: u8,
}

// !Penalty
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Penalty {
    pub penaltyType: u8,
    pub infringementType: u8,
    pub vehicleIdx: u8,
    pub otherVehicleIdx: u8,
    pub time: u8,
    pub lapNum: u8,
    pub placesGained: u8,
}

// !SpeedTrap
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpeedTrap {
    pub vehicleIdx: u8,
    pub speed: f32,
    pub isOverallFastestInSession: u8,
    pub isDriverFastestInSession: u8,
    pub fastestVehicleIdxInSession: u8,
    pub fastestSpeedInSession: f32,
}

// !StartLights
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartLights {
    pub numLights: u8,
}

// !DriveThroughPenaltyServe
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DriveThroughPenaltyServed {
    pub vehicleIdx: u8,
}

// !StopGoPenaltyServed
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StopGoPenaltyServed {
    pub vehicleIdx: u8,
}

// !Flashback
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flashback {
    pub flashbackFrameIdentifier: u32,
    pub flashbackSessionTime: f32,
}

// !Buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buttons {
    pub m_buttonStatus: u32,
}

// Event Data Details
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub union EventDataDetails {
    pub m_fastestLap: FastestLap,
    pub m_retirement: Retirement,
    pub m_teamMateInPits: TeamMateInPits,
    pub m_raceWinner: RaceWinner,
    pub m_penalty: Penalty,
    pub m_speedTrap: SpeedTrap,
    pub m_startLights: StartLights,
    pub m_driveThroughPenaltyServed: DriveThroughPenaltyServed,
    pub m_stopGoPenaltyServed: StopGoPenaltyServed,
    pub m_flashback: Flashback,
    pub m_buttons: ButtonStatus,
}

/*
    Event String Codes

    Event              Code                 Description
    ---------------------------------------------------
    Session Started    "SSTA"               Session Started
    Session Ended      "SEND"               Session Ended
    Fastest Lap        "FTLP"               Fastest Lap
    Retirement         "RTMT"               Retirement
    DRS Enabled        "DRSE"               DRS Enabled
    DRS Disabled       "DRSD"               DRS Disabled
    Team Mate In Pits  "TMPT"               Team Mate In Pits
    Chequered Flag     "CHQF"               Chequered Flag has been waved
    ...
    
*/

// Packet Event Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub m_eventStringCode: [u8; 4],
    pub m_eventDetails: EventDataDetails,
}

/* PARTICIPANTS PACKET */
// Participant Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantData {
    pub m_aiControlled: u8,
    pub m_driverId: u8,
    pub m_networkId: u8,
    pub m_teamId: u8,
    pub m_myTeam: u8,
    pub m_raceNumber: u8,
    pub m_nationality: u8,
    pub m_name: [u8; 48],
    pub m_yourTelemetry: u8,
}

// Packet Participants Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketParticipantsData {
    pub m_header: PacketHeader,
    pub m_numActiveCars: u8,
    pub m_participants: [ParticipantData; 22],
}

/* CAR SETUPS PACKET */
// Car Setup Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarSetupData {
    pub m_frontWing: u8,
    pub m_rearWing: u8,
    pub m_onThrottle: u8,
    pub m_offThrottle: u8,
    pub m_frontCamber: f32,
    pub m_rearCamber: f32,
    pub m_frontToe: f32,
    pub m_rearToe: f32,
    pub m_frontSuspension: u8,
    pub m_rearSuspension: u8,
    pub m_frontAntiRollBar: u8,
    pub m_rearAntiRollBar: u8,
    pub m_frontSuspensionHeight: u8,
    pub m_rearSuspensionHeight: u8,
    pub m_brakePressure: u8,
    pub m_brakeBias: u8,
    pub m_rearLeftTyrePressure: f32,
    pub m_rearRightTyrePressure: f32,
    pub m_frontLeftTyrePressure: f32,
    pub m_frontRightTyrePressure: f32,
    pub m_ballast: u8,
    pub m_fuelLoad: f32,
}

// Packet Car Setup Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketCarSetupData {
    pub m_header: PacketHeader,
    pub m_carSetups: [CarSetupData; 22],
}

/* CAR TELEMETRY PACKET */
// Car Telemetry Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarTelemetryData {
    pub m_speed: u16,
    pub m_throttle: f32,
    pub m_steer: f32,
    pub m_brake: f32,
    pub m_clutch: u8,
    pub m_gear: i8,
    pub m_engineRPM: u16,
    pub m_drs: u8,
    pub m_revLightsPercent: u8,
    pub m_brakesTemperature: [u16; 4],
    pub m_tyresSurfaceTemperature: [u16; 4],
    pub m_tyresInnerTemperature: [u16; 4],
    pub m_engineTemperature: u16,
    pub m_tyresPressure: [f32; 4],
    pub m_surfaceType: [u8; 4],
}

// Packet Car Telemetry Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader,
    pub m_carTelemetryData: [CarTelemetryData; 22],
    pub m_mfdPanelIndex: u8,
    pub m_mfdPanelIndexSecondaryPlayer: u8,
    pub m_suggestedGear: i8,
}

/* CAR STATUS PACKET */
// Car Status Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarStatusData {
    pub m_tractionControl: u8,
    pub m_antiLockBrakes: u8,
    pub m_fuelMix: u8,
    pub m_frontBrakeBias: u8,
    pub m_pitLimiterStatus: u8,
    pub m_fuelInTank: f32,
    pub m_fuelCapacity: f32,
    pub m_fuelRemainingLaps: f32,
    pub m_maxRPM: u16,
    pub m_idleRPM: u16,
    pub m_maxGears: u8,
    pub m_drsAllowed: u8,
    pub m_drsActivationDistance: u16,
    pub m_actualTyreCompound: u8,
    pub m_visualTyreCompound: u8,
    pub m_tyresAgeLaps: u8,
    pub m_vehicleFiaFlags: i8,
    pub m_ersStoreEnergy: f32,
    pub m_ersDeployMode: u8,
    pub m_ersHarvestedThisLapMGUK: f32,
    pub m_ersHarvestedThisLapMGUH: f32,
    pub m_ersDeployedThisLap: f32,
    pub m_networkPaused: u8,
}

// Packet Car Status Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader,
    pub m_carStatusData: [CarStatusData; 22],
}

/* FINAL CLASSIFICATION PACKET */
// Final Classification Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FinalClassificationData {
    pub m_position: u8,
    pub m_numLaps: u8,
    pub m_gridPosition: u8,
    pub m_points: u8,
    pub m_numPitStops: u8,
    pub m_resultStatus: u8,
    pub m_bestLapTimeInMS: u32,
    pub m_totalRaceTime: f64,
    pub m_penaltiesTime: u8,
    pub m_numPenalties: u8,
    pub m_numTyreStints: u8,
    pub m_tyreStintsActual: [u8; 8],
    pub m_tyreStintsVisual: [u8; 8],
    pub m_tyreStintEndLaps: [u8; 8],
}

// Packet Final Classification Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketFinalClassificationData {
    pub m_header: PacketHeader,
    pub m_numCars: u8,
    pub m_classificationData: [FinalClassificationData; 22],
}

/* LOBBY INFO PACKET */
// Lobby Info Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LobbyInfoData {
    pub m_aiControlled: u8,
    pub m_teamId: u8,
    pub m_nationality: u8,
    pub m_name: [u8; 48],
    pub m_carNumber: u8,
    pub m_readyStatus: u8,
}

// Packet Lobby Info Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketLobbyInfoData {
    pub m_header: PacketHeader,
    pub m_numPlayers: u8,
    pub m_lobbyPlayers: [LobbyInfoData; 22],
}

/* CAR DAMAGE PACKET */
// Car Damage Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarDamageData {
    pub m_tyresWear: [u8; 4],
    pub m_tyreDamage: [u8; 4],
    pub m_brakesDamage: [u8; 4],
    pub m_frontLeftWingDamage: u8,
    pub m_frontRightWingDamage: u8,
    pub m_rearWingDamage: u8,
    pub m_floorDamage: u8,
    pub m_diffuserDamage: u8,
    pub m_sidepodDamage: u8,
    pub m_drsFault: u8,
    pub m_ersFault: u8,
    pub m_gearBoxDamage: u8,
    pub m_engineDamage: u8,
    pub m_engineMGUHWear: u8,
    pub m_engineESWear: u8,
    pub m_engineCEWear: u8,
    pub m_engineICEWear: u8,
    pub m_engineMGUKWear: u8,
    pub m_engineTCWear: u8,
    pub m_engineBlown: u8,
    pub m_engineSeized: u8,
}

// Packet Car Damage Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketCarDamageData {
    pub m_header: PacketHeader,
    pub m_carDamageData: [CarDamageData; 22],
}

/* SESSION HISTORY PACKET */
// Lap History Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LapHistoryData {
    pub m_lapTimeInMS: u32,
    pub m_sector1TimeInMS: u16,
    pub m_sector2TimeInMS: u16,
    pub m_sector3TimeInMS: u16,
    pub m_lapValidBitFlags: u8,
}

// Tyre Stints History Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TyreStintsHistoryData {
    pub m_endLap: u8,
    pub m_tyreActualCompound: u8,
    pub m_tyreVisualCompound: u8,
}

// Packet Session History Data
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketSessionHistoryData {
    pub m_header: PacketHeader,
    pub m_numCars: u8,
    pub m_numTyreStints: u8,

    pub m_bestLapTimeNum: u8,
    pub m_bestSector1TimeNum: u8,
    pub m_bestSector2TimeNum: u8,
    pub m_bestSector3TimeNum: u8,

    pub m_lapHistoryData: [LapHistoryData; 100],
    pub m_tyreStintsHistoryData: [TyreStintsHistoryData; 8],
}