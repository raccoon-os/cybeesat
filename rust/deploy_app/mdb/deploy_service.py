# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *

service = System("DeployService")
service_type_id = 136

base_cmd = Command(
    system=service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": service_type_id},
)

antenna_argument = IntegerArgument(name="antenna_number", minimum=1, maximum=4, encoding=uint8_t)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 1},
    name="DeployAntenna",
    arguments=[antenna_argument],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 2},
    name="RetractAntenna",
    arguments=[antenna_argument],
)

print(service.dumps())