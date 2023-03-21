import socket

from data import PacketHeader

# constants
UDP_IP = "192.168.1.87"
UDP_RECEIVE_PORT = 20777

# send and receive data
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.bind(('0.0.0.0', UDP_RECEIVE_PORT))
s.settimeout(10)

while True:
    try:
        data, addr = s.recvfrom(4096)
        
        try:
            packet_header = PacketHeader(data)
            print(packet_header)
        except Exception as e:
            print("Packet received:", data)

    except socket.timeout:
        print('Timeout')
    except KeyboardInterrupt:
        print('Keyboard interrupt')
        break

s.close()