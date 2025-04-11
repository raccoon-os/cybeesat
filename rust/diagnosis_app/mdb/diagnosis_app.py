# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *

service = System("DiagnosisService")
service_type_id = 137

base_cmd = Command(
    system=service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": service_type_id},
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 1},
    name="ScanI2C",
    arguments=[IntegerArgument(name="bus", minimum=0, maximum=2, encoding=uint8_t)],
)

Container(
    name="ScanI2CResponse",
    base="/PUS/pus-tm",
    system=service,
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 1),
    ),
    entries=[
        ParameterEntry(IntegerParameter(system=service, name="bus", encoding=uint8_t)),
        ParameterEntry(IntegerParameter(system=service, name="n", encoding=uint16_t)),
        ParameterEntry(
            ArrayParameter(
                system=service,
                name="i2c_devices",
                length=DynamicInteger("n"),
                data_type=IntegerDataType(bits=8, encoding=uint8_t)
            )
        ),
    ],
)

print(service.dumps())