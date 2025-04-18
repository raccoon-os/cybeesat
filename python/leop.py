#!/usr/bin/env python3

import json
import os
import sys
import time
import subprocess
from dataclasses import dataclass

MAX_ATTEMPTS = 5
WAIT_TIME_SECONDS = 10
ANTENNA_CONTROL_SCRIPT = "/usr/bin/antenna_control.py"
#ANTENNA_CONTROL_SCRIPT = "/usr/bin/echo"

@dataclass
class LEOPState:
    FILE = "/var/leop.json"

    wait_passed: bool
    attempts: int

    @classmethod
    def read(cls):
        default = cls(wait_passed=False, attempts=0)
        if not os.path.exists(LEOPState.FILE):
            return default
        else:
            try:
                data = json.loads(open(LEOPState.FILE, "r").read())
                return cls(**data)

            except Exception as e:
                print("Exception trying to read LEOP file", e)
                return default
            
    def save(self):
        j = json.dumps({'wait_passed': self.wait_passed, 'attempts': self.attempts})
        open(self.FILE, "w").write(j)

def deploy_antennas():
    for antenna in [1, 3, 2, 4]:
        antenna_arg = str(antenna + 1)

        print("deploy", antenna_arg)
        subprocess.run(["python3", ANTENNA_CONTROL_SCRIPT, "deploy", antenna_arg])
        time.sleep(3)

        print("retract", antenna_arg)
        subprocess.run(["python3", ANTENNA_CONTROL_SCRIPT, "retract", antenna_arg])
        time.sleep(1)

if __name__ == '__main__':
    state = LEOPState.read()
    print("state is", state)

    if not state.wait_passed:
        print("Waiting for", WAIT_TIME_SECONDS, "seconds")
        time.sleep(WAIT_TIME_SECONDS)

        state.wait_passed = True
        state.save()

    print("state is", state)

    while state.attempts < MAX_ATTEMPTS:
        deploy_antennas()
        state.attempts += 1
        state.save()
        
        print("state is", state)

    print("Max attempts exceeded, tschüss!")
    sys.exit(0)

