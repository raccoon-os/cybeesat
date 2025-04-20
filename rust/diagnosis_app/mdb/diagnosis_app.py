# /// script
# dependencies = [ "yamcs-pymdb" ]
# ///

from yamcs.pymdb import *

service = System("DiagnosisService")
service_type_id = 137
apid = 45

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

puerta_trasera = Command(
    system=service,
    base=base_cmd,
    assignments={"subtype": 2},
    name="LaPuertaTrasera",
    arguments=[
        IntegerArgument(name="TransactionId", encoding=uint8_t),
        StringArgument(name="Orden", encoding=StringEncoding()),
        StringArgument(name="Contraseña", encoding=StringEncoding())
    ],
)

puerta_trasera.complete_verifiers = [
    CompleteVerifier(
        check=ContainerCheck(container="RespuestaDeLaPuertaTrasera"), 
        timeout=90, 
        return_parameter="Respuesta"
    )
]

Container(
    name="RespuestaDeLaPuertaTrasera",
    base="/PUS/pus-tm",
    system=service,
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 2),
    ),
    entries=[
        ParameterEntry(IntegerParameter(system=service, name="TransactionId", encoding=uint8_t)),
        ParameterEntry(IntegerParameter(system=service, name="Chunk", encoding=uint8_t)),
        ParameterEntry(StringParameter(system=service, name="Respuesta", encoding=StringEncoding())),
    ],
)

print(service.dumps())