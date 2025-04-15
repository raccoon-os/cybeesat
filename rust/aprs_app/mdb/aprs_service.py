# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *

service = System("APRSService")
service_type_id = 142

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
    name="SetBeaconInterval",
    arguments=[
        IntegerArgument(name="Interval", encoding=uint16_t),
    ],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 2},
    name="SetPowerMode",
    arguments=[
        BooleanArgument(name="DIGI1_ENABLE", encoding=uint8_t),
        BooleanArgument(name="DIGI2_ENABLE", encoding=uint8_t),
        BooleanArgument(name="HIGH_POWER_MODE", encoding=uint8_t),
    ],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 3},
    name="SetBeaconMessage",
    arguments=[
        StringArgument(name="MESSAGE", encoding=StringEncoding())
    ],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 4},
    name="SetCallsign",
    arguments=[
        StringArgument(name="CALLSIGN", encoding=StringEncoding())
    ],
)

Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 5},
    name="GetTelemetry",
    arguments=[],
)

def str_parameter(service, name, short_description=None):
    return ParameterEntry(
        StringParameter(
            system=service,
            name=name,
            short_description=short_description,
            encoding=StringEncoding()
        )
    )

def int_parameter(service, name, units, short_description=None, encoding=uint16_t):
    return ParameterEntry(
        IntegerParameter(
            system=service,
            name=name,
            encoding=encoding,
            units=units,
            short_description=short_description
        )
    )

def bool_parameter(service, name, short_description=None):
    return ParameterEntry(
        BooleanParameter(
            system=service,
            name=name,
            short_description=short_description,
            encoding=bool_t
        )
    )

Container(
    name="APRSTelemetry",
    base="/PUS/pus-tm",
    system=service,
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 1),
    ),
    entries=[
        str_parameter(service, "CALLSIGN"),
        int_parameter(service, "BEACON_TIME", "seconds"),
        str_parameter(service, "BEACON_MESSAGE"),
        bool_parameter(service, "HIGH_POWER"),
        bool_parameter(service, "DIGI1_ENABLED"),
    ],
)

print(service.dumps())