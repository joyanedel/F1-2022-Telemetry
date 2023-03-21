import struct

class PacketHeader:
    def __init__(self, data) -> None:
        self.m_packetFormat = data[0] + (data[1] << 8)
        self.m_gameMajorVersion = data[2]
        self.m_gameMinorVersion = data[3]
        self.m_packetVersion = data[4]
        self.m_packetId = data[5]
        self.m_sessionUID = data[6] + (data[7] << 8) + (data[8] << 16) + (data[9] << 24) + (data[10] << 32) + (data[11] << 40) + (data[12] << 48) + (data[13] << 56)
        self.m_sessionTime = struct.unpack('f', bytes(data[14:18]))[0]
        self.m_frameIdentifier = data[18] + (data[19] << 8) + (data[20] << 16) + (data[21] << 24)
        self.m_playerCarIndex = data[22]
        self.m_secondaryPlayerCarIndex = data[23]
    
    def __str__(self) -> str:
        return f'''
        PacketHeader(
            m_packetFormat={self.m_packetFormat},
            m_gameMajorVersion={self.m_gameMajorVersion},
            m_gameMinorVersion={self.m_gameMinorVersion},
            m_packetVersion={self.m_packetVersion},
            m_packetId={self.m_packetId},
            m_sessionUID={self.m_sessionUID},
            m_sessionTime={self.m_sessionTime},
            m_frameIdentifier={self.m_frameIdentifier},
            m_playerCarIndex={self.m_playerCarIndex},
            m_secondaryPlayerCarIndex={self.m_secondaryPlayerCarIndex}
        )'''

class CarMotionData:
    def __init__(self, data) -> None:
        self.m_worldPositionX = struct.unpack('f', bytes(data[0:4]))[0]
        self.m_worldPositionY = struct.unpack('f', bytes(data[4:8]))[0]
        self.m_worldPositionZ = struct.unpack('f', bytes(data[8:12]))[0]
        self.m_worldVelocityX = struct.unpack('f', bytes(data[12:16]))[0]
        self.m_worldVelocityY = struct.unpack('f', bytes(data[16:20]))[0]
        self.m_worldVelocityZ = struct.unpack('f', bytes(data[20:24]))[0]
        self.m_worldForwardDirX = struct.unpack('h', bytes(data[24:26]))[0]
        self.m_worldForwardDirY = struct.unpack('h', bytes(data[26:28]))[0]
        self.m_worldForwardDirZ = struct.unpack('h', bytes(data[28:30]))[0]
        self.m_worldRightDirX = struct.unpack('h', bytes(data[30:32]))[0]
        self.m_worldRightDirY = struct.unpack('h', bytes(data[32:34]))[0]
        self.m_worldRightDirZ = struct.unpack('h', bytes(data[34:36]))[0]
        self.m_gForceLateral = struct.unpack('f', bytes(data[36:40]))[0]
        self.m_gForceLongitudinal = struct.unpack('f', bytes(data[40:44]))[0]
        self.m_gForceVertical = struct.unpack('f', bytes(data[44:48]))[0]
        self.m_yaw = struct.unpack('f', bytes(data[48:52]))[0]
        self.m_pitch = struct.unpack('f', bytes(data[52:56]))[0]
        self.m_roll = struct.unpack('f', bytes(data[56:60]))[0]

class PacketMotionData:
    def __init__(self, data) -> None:
        self.m_header = PacketHeader(data[0:24])
        self.m_carMotionData = [CarMotionData(data[24 + 60 * i:84 + 60 * i]) for i in range(22)]

    def __str__(self) -> str:
        return f'''
        PacketMotionData(
            m_header={self.m_header},
            m_carMotionData={self.m_carMotionData}
        )'''


class Session:
    def __init__(self, data) -> None:
        pass

class LapData:
    def __init__(self, data) -> None:
        pass

class Event:
    def __init__(self, data) -> None:
        pass

class Participants:
    def __init__(self, data) -> None:
        pass

class CarSetups:
    def __init__(self, data) -> None:
        pass

class CarTelemetry:
    def __init__(self, data) -> None:
        pass

class CarStatus:
    def __init__(self, data) -> None:
        pass

class FinalClassification:
    def __init__(self, data) -> None:
        pass

class LobbyInfo:
    def __init__(self, data) -> None:
        pass

class CarDamage:
    def __init__(self, data) -> None:
        pass

class SessionHistory:
    def __init__(self, data) -> None:
        pass