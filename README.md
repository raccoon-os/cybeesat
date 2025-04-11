# CyBEEsat Software

This repository contains the onboard software and ground segment for the CyBEEsat mission.

It consists of:

- A [configuration file](config.yaml) to build the RACCOON OS / Yocto Linux image
- Rust applications specific to the CyBEEsat platform:
    - `rust/boot_app`: Manages the boot counter. Provides functions to read it and reset it.
    - `rust/deploy_app`: Provides a TC interface to deploy the VHF antennas. Uses the `python/antenna_control.py` script.
    - `rust/diagnosis_app`: Provides onboard hardware diagnostics. At the moment, it only performs I2C device scans.
    - `rust/vcom_interface`: A driver for the VCOM transceiver on the CyBEEsat bus. Manages transmission and reception of air frames.

- Standard applications from the [RACCOON Userspace](https://gitlab.com/rccn/rccn-userspace/ws) library:
    - `rccn_usr_comm`: Manages CCSDS communication. Receives TCs from the `vcom_interface` and publishes Space Packets to Zenoh keys. Vice versa for TC.
    - `rccn_usr_launch`: Starts all of the applications, handles logging and provides a TC/TM inteface to monitor/restart processes.
    - `rccn_usr_cfdp`: Enables file upload / download using the CCSDS File Delivery Protocol (CFDP).
    - `rccn_usr_update`: Manages software updates using OSTree.

# Building

## RACCOON OS

To build the base RACCOON OS image for CyBEEsat, run:

    kas build kas.yml

## Applications

To build the Rust applications for CyBEEsat, run:

    make deploy

This will install the binaries of the Rust applications in the `install/usr/bin` directory.
The `install` directory is intended to be copied to the root filesystem of the CyBEEsat OBC.

To create a delta upgrade package, run:

    make delta DELTA_FROM=<current_onboard_revision>

Where `<current_onboard_revision>` is the current OSTree commit that is running on the OBC.

## Ground Segment

... TBD!

# Acknowledgements

Work on this repository is partially sponsored by [Quantum Galactics GmbH](https://quantumgalactics.com/).
