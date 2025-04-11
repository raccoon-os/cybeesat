import socket
import threading
import time
from pySerialTransfer import pySerialTransfer as txfer

# Serial port configuration
COM_PORT = '/dev/ttyUSB1'
BAUD_RATE = 115200

# Yamcs UDP interface configuration
UDP_IP = '192.168.188.86'
UDP_PORT_TM = 10017
UDP_PORT_TC = 10018

def serial_to_udp(link, udp_ip, udp_port, stop_event):
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    print("Serial to UDP thread started")
    while not stop_event.is_set():
        if link.available():
            try:
                tm = bytearray()
                for index in range(link.bytesRead):
                    tm.append(link.rxBuff[index])
                udp_socket.sendto(tm, (udp_ip, udp_port))
                print(f"Sent to UDP: {tm}")
            except Exception as e:
                print(f"Error reading from serial link: {e}")
        elif link.status < 0:
            print('ERROR: {}'.format(link.status))
        time.sleep(0.1)  # Add a small sleep to allow checking the stop_event
    print("Serial to UDP thread stopped")

def udp_to_serial(link, udp_ip, udp_port, stop_event):
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
            udp_socket.settimeout(0.1)  # Set a timeout to allow checking the stop_event
            data, addr = udp_socket.recvfrom(1024)
            print(f"Received from UDP: {data}")
            try:
                # Clear the transmit buffer
                link.txBuff = [0] * len(data)
                # Load the data into the transmit buffer
                for i in range(len(data)):
                    link.txBuff[i] = data[i]
                # Send the data
                link.send(len(data))
                print(f"Sent to Serial: {data}")
            except Exception as e:
                print(f"Error sending to serial link: {e}")
        except socket.timeout:
            continue
        except socket.error as e:
            print(f"Socket error: {e}")
            stop_event.set()
            break
    print("UDP to Serial thread stopped")

if __name__ == "__main__":
    try:
        link = txfer.SerialTransfer(COM_PORT)
        link.open()
        print(f"Serial link opened on {COM_PORT}")
    except Exception as e:
        print(f"Failed to open serial link on {COM_PORT}: {e}")
        exit(1)

    stop_event = threading.Event()

    serial_to_udp_thread = threading.Thread(target=serial_to_udp, args=(link, UDP_IP, UDP_PORT_TM, stop_event))
    udp_to_serial_thread = threading.Thread(target=udp_to_serial, args=(link, UDP_IP, UDP_PORT_TC, stop_event))

    serial_to_udp_thread.start()
    udp_to_serial_thread.start()

    try:
        while True:
            time.sleep(1)  # Add a small sleep to reduce CPU usage
    except KeyboardInterrupt:
        print("Interrupted by user. Exiting...")
        stop_event.set()
        serial_to_udp_thread.join()
        udp_to_serial_thread.join()
    finally:
        link.close()
        print("Serial link closed")
