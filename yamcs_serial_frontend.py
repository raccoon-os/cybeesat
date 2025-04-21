import socket
import threading
import time
import serial

# Serial port configuration
COM_PORT = '/dev/ttyUSB0'
BAUD_RATE = 115200

# Yamcs UDP interface configuration
UDP_IP = '127.0.0.1'
UDP_PORT_TM = 10066
UDP_PORT_TC = 10056

def serial_to_udp(ser, udp_ip, udp_port, stop_event):
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    print("Serial to UDP thread started")

    while not stop_event.is_set():
        try:
            tm = ser.read(525)
            if tm:
                udp_socket.sendto(tm, (udp_ip, udp_port))
                print(f"Sent to UDP: {tm}")
        except serial.SerialException as e:
            print(f"Serial error: {e}")
            stop_event.set()
        time.sleep(0.01)

    print("Serial to UDP thread stopped")

def udp_to_serial(ser, udp_ip, udp_port, stop_event):
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        udp_socket.bind((udp_ip, udp_port))
        print(f"UDP to Serial thread started, listening on {udp_ip}:{udp_port}")
    except PermissionError as e:
        print(f"PermissionError: {e}")
        stop_event.set()
        return

    while not stop_event.is_set():
        try:
            udp_socket.settimeout(0.1)
            data, addr = udp_socket.recvfrom(1024)
            if data:
                ser.write(data)
                print(f"Sent to Serial: {data}")
        except socket.timeout:
            continue
        except (socket.error, serial.SerialException) as e:
            print(f"Error: {e}")
            stop_event.set()
            break

    print("UDP to Serial thread stopped")

if __name__ == "__main__":
    try:
        ser = serial.Serial(COM_PORT, BAUD_RATE, timeout=1)
        print(f"Serial port opened on {COM_PORT}")
    except Exception as e:
        print(f"Failed to open serial port on {COM_PORT}: {e}")
        exit(1)

    stop_event = threading.Event()

    serial_to_udp_thread = threading.Thread(
        target=serial_to_udp,
        args=(ser, UDP_IP, UDP_PORT_TM, stop_event)
    )

    udp_to_serial_thread = threading.Thread(
        target=udp_to_serial,
        args=(ser, UDP_IP, UDP_PORT_TC, stop_event)
    )

    serial_to_udp_thread.start()
    udp_to_serial_thread.start()

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("Interrupted by user. Exiting...")
        stop_event.set()
        serial_to_udp_thread.join()
        udp_to_serial_thread.join()
    finally:
        ser.close()
        print("Serial port closed")

