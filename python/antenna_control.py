#!/usr/bin/env python3
import sys
from time import sleep
from smbus import SMBus

# Create a new I2C bus
i2c_bus = SMBus(1)

io_expander = 0x20


def set_register_bit(register, bit, level):
    """
    Sets or clears a specific bit in an 8-bit register.

    Parameters:
        register (int): The current register value (0–255).
        bit (int): The bit position to modify (0–7).
        level (str): "high" to set the bit, "low" to clear the bit.

    Returns:
        int: The updated register value.
    """
    if not (0 <= bit <= 7):
        raise ValueError("Bit must be in the range 0 to 7.")
    
    if level.lower() == "high":
        # Set the specified bit to 1
        register = register | (1 << bit)
    elif level.lower() == "low":
        # Clear the specified bit to 0
        register = register & ~(1 << bit)
    else:
        raise ValueError("Level must be 'high' or 'low'.")

    return register

def read_output_port(port):
    port0output = i2c_bus.read_i2c_block_data(io_expander, 0x02, 1)
    if port == 0:
        return port0output[0]
    elif port == 1:
        return port0output[1]

def write_gpio(port, pin, level):
    if port == 0:
        port0output = read_output_port(0)
        new_port0output = set_register_bit(port0output, pin, level)
        i2c_bus.write_i2c_block_data(io_expander, 0x02, [new_port0output])

def main():
    # ina_user_3v3 0x4a
    # send to reg 0x14
    # value 0x6060
    # power off 0x4040

    # power on antenna deployment mechanism
    i2c_bus.write_i2c_block_data(0x4a, 0x14, [0x60, 0x60])


    if len(sys.argv) != 3:
        print("Usage: python deploy_antenna.py <deploy|retract> <antenna_number>")
        sys.exit(1)

    action = sys.argv[1].lower()
    antenna_number = int(sys.argv[2])

    if action not in ["deploy", "retract"]:
        print("Invalid action. Use 'deploy' or 'retract'.")
        sys.exit(1)

    if antenna_number not in [1, 2, 3, 4]:
        print("Invalid antenna number. Use 1, 2, 3, or 4.")
        sys.exit(1)

    # read port0 config
    port0config = i2c_bus.read_i2c_block_data(io_expander, 0x06, 1)
    
    # Set the corresponding IO pins to output
    i2c_bus.write_i2c_block_data(io_expander, 0x06, [0x00])

    # clear all IO
    i2c_bus.write_i2c_block_data(io_expander, 0x02, [0x00])

    if action == "deploy":
        if antenna_number == 1:
            write_gpio(0, 0, "high")
            write_gpio(0, 1, "low")
        elif antenna_number == 2:
            write_gpio(0, 2, "high")
            write_gpio(0, 3, "low")
        elif antenna_number == 3:
            write_gpio(0, 4, "high")
            write_gpio(0, 5, "low")
        elif antenna_number == 4:
            write_gpio(0, 6, "high")
            write_gpio(0, 7, "low")
        sleep(1)
        if antenna_number == 1:
            write_gpio(0, 0, "low")
            write_gpio(0, 1, "low")
        elif antenna_number == 2:
            write_gpio(0, 2, "low")
            write_gpio(0, 3, "low")
        elif antenna_number == 3:
            write_gpio(0, 4, "low")
            write_gpio(0, 5, "low")
        elif antenna_number == 4:
            write_gpio(0, 6, "low")
            write_gpio(0, 7, "low")
    elif action == "retract":
        if antenna_number == 1:
            write_gpio(0, 0, "high")
            write_gpio(0, 1, "high")
        elif antenna_number == 2:
            write_gpio(0, 2, "high")
            write_gpio(0, 3, "high")
        elif antenna_number == 3:
            write_gpio(0, 4, "high")
            write_gpio(0, 5, "high")
        elif antenna_number == 4:
            write_gpio(0, 6, "high")
            write_gpio(0, 7, "high")
        sleep(1)
        if antenna_number == 1:
            write_gpio(0, 0, "low")
            write_gpio(0, 1, "low")
        elif antenna_number == 2:
            write_gpio(0, 2, "low")
            write_gpio(0, 3, "low")
        elif antenna_number == 3:
            write_gpio(0, 4, "low")
            write_gpio(0, 5, "low")
        elif antenna_number == 4:
            write_gpio(0, 6, "low")
            write_gpio(0, 7, "low")

    # power off antenna deployment mechanism
    i2c_bus.write_i2c_block_data(0x4a, 0x14, [0x40, 0x40])


if __name__ == "__main__":
    main()