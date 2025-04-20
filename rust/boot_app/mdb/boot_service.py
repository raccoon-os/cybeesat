# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *

service = System("BootService")
service_type_id = 135
apid = 44

base_cmd = Command(
    system=service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": service_type_id, "apid": apid},
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 1},
    name="GetBootCounter",
    arguments=[],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 2},
    name="ResetBootCounter",
    arguments=[],
)

Container(
    name="GetBootCounterResponse",
    base="/PUS/pus-tm",
    system=service,
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 1),
    ),
    entries=[
        ParameterEntry(IntegerParameter(system=service, name="boot_counter", encoding=uint16_t)),
    ],
)

print(service.dumps())