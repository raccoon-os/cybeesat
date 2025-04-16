#!/usr/bin/env python3.13 
import sys
import os

from yamcs.pymdb import *
# from application import *
# from service import *

from rccn_gen import *

root_system = System("BiX1")
app = Application(system=root_system, name="BiX1_Cntrl_App", apid=77)

service = Service(name="Telemetry", system=app)
service_type_id = 77
service.service_id = service_type_id

base_cmd = Command(
    system=service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": service_type_id},
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

def enum_parameter(service, name, choices, short_description=None, encoding=uint8_t):
    return ParameterEntry(
            EnumeratedParameter(
                system=service,
                name=name,
                encoding=encoding,
                choices=choices,
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
    
def multi_int_parameter(service, base_name, count, units, short_description_base = None, description_list = None, encoding=uint16_t):
    return [int_parameter(service, base_name + str(i), units, short_description=short_description_base + description_list[i] if description_list != None else short_description_base + " " + str(i), encoding=encoding)
            for i in range(count)]


RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 6},
    name="RQ_OBC_INFO",
    short_description="Request OBC_INFO Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OBC_INFO",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 6),
    ),
    entries=[
        bool_parameter(service, "ACTIVE_OBC", "Currently active OBC"),
        int_parameter(service, "OBC_UPTIME", "seconds", "Time since boot OBC", uint32_t),
        int_parameter(service, "OBC_SYSMEM", "mb", "Ram usage", uint8_t),
        int_parameter(service, "OBC_USERMEM", "gb", "Total avail. Storage space", uint8_t),
        int_parameter(service, "OBC_CPU_UTIL", "Percent 0 - 100", "Cpu usage", uint8_t),
        int_parameter(service, "OBC_ONBOARD_UTC", "seconds", "Onboard time UTC", uint32_t),
        int_parameter(service, "LAST_SESSION_UTC", "seconds", "Timestamp of last/previous contact", uint32_t),
    ]   
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 7},
    name="RQ_user_defined_tm",
    short_description="Request user_defined_tm Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="user_defined_tm",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 7),
    ),
    entries=[
        *multi_int_parameter(service, "user_defined_parameter", 6, "tbd.", "tbd.", encoding=uint32_t)
    ]   
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 8},
    name="RQ_COM",
    short_description="Request COM Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="COM",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 8),
    ),
    entries=[
        enum_parameter(service, "VCOM0_STAT", [[0, "tbd1"], [1, "tbd2"], [2, "tbd3"]], "tbd", uint8_t),
        int_parameter(service, "VCOM0_RSSI", "dB", "Received signal strength indicator", int8_t),
        enum_parameter(service, "VCOM1_STAT", [[0, "tbd1"], [1, "tbd2"], [2, "tbd3"]], "tbd", uint8_t),
        int_parameter(service, "VCOM1_RSSI", "dB", "Received signal strength indicator", int8_t),
    ]   
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 9},
    name="RQ_IMU",
    short_description="Request IMU Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="IMU",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 9),
    ),
    entries=[
        int_parameter(service, "GYRO0_X_SENS", "Deg/s", "Angular rate X-axis Sensor 0", int16_t),
        int_parameter(service, "GYRO0_Y_SENS", "Deg/s", "Angular rate Y-axis Sensor 0", int16_t),
        int_parameter(service, "GYRO0_Z_SENS", "Deg/s", "Angular rate Z-axis Sensor 0", int16_t),
        int_parameter(service, "ACCEL0_X", "g", "Linear acceleration sensor X-axis measurement Sensor 0", int16_t),
        int_parameter(service, "ACCEL0_Y", "g", "Linear acceleration sensor Y-axis measurement Sensor 0", int16_t),
        int_parameter(service, "ACCEL0_Z", "g", "Linear acceleration sensor Z-axis measurement Sensor 0", int16_t),
        int_parameter(service, "MAG0_X", "Micro Tesla", "Magnetic field measurement X-axis Sensor 0", int16_t),
        int_parameter(service, "MAG0_Y", "Micro Tesla", "Magnetic field measurement Y-axis Sensor 0", int16_t),
        int_parameter(service, "MAG0_Z", "Micro Tesla", "Magnetic field measurement Z-axis Sensor 0", int16_t),
        int_parameter(service, "GYRO1_X_SENS", "Deg/s", "Angular rate X-axis Sensor 1", int16_t),
        int_parameter(service, "GYRO1_Y_SENS", "Deg/s", "Angular rate Y-axis Sensor 1", int16_t),
        int_parameter(service, "GYRO1_Z_SENS", "Deg/s", "Angular rate Z-axis Sensor 1", int16_t),
        int_parameter(service, "ACCEL1_X", "g", "Linear acceleration sensor X-axis measurement Sensor 1", int16_t),
        int_parameter(service, "ACCEL1_Y", "g", "Linear acceleration sensor Y-axis measurement Sensor 1", int16_t),
        int_parameter(service, "ACCEL1_Z", "g", "Linear acceleration sensor Z-axis measurement Sensor 1", int16_t),
        int_parameter(service, "MAG1_X", "Micro Tesla", "Magnetic field measurement X-axis Sensor 1", int16_t),
        int_parameter(service, "MAG1_Y", "Micro Tesla", "Magnetic field measurement Y-axis Sensor 1", int16_t),
        int_parameter(service, "MAG1_Z", "Micro Tesla", "Magnetic field measurement Z-axis Sensor 1", int16_t),
    ]   
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 10},
    name="RQ_PAYLOAD",
    short_description="Request PAYLOAD Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="PAYLOAD",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 10),
    ),
    entries=[
        int_parameter(service, "APRS_STAT0", "tbd", "APRS Module operational status 0", uint16_t),
        int_parameter(service, "APRS_STAT1", "tbd", "APRS Module operational status 1", uint16_t),
    ]
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 12},
    name="RQ_OPT_EPS_RTC",
    short_description="Request OPT_EPS_RTC Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_EPS_RTC",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 12),
    ),
    entries=[
        int_parameter(service, "EPS_RTC_DATETIME", "unix timestamp", "Realtime Clock date time", uint32_t),
        *multi_int_parameter(service, "EPS_RTC_CONTROL", 6, "control flags tbc.", "Realtime Clock control settings register")
    ]
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 13},
    name="RQ_OPT_EPS_BATTERY",
    short_description="Request OPT_EPS_BATTERY Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_EPS_BATTERY",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 13),
    ),
    entries=[
        int_parameter(service, "PMIC0_THERM", "Ohm", "Battery charger 0 Thermistor", uint16_t),
        enum_parameter(service, "FG0_STAT", [[0, "charging"], [1, "not_charging"], [2, "charge_terminated"]] , "Fuel gauge 0 status", uint8_t),
        int_parameter(service, "FG0_SOC", "Percent", "Fuel gauge 0 battery state of charge", uint8_t),
        int_parameter(service, "FG0_THERM", "Ohm", "Fuel gauge 0 Thermistor", uint16_t),
        bool_parameter(service, "PASS_SW0_STAT", "Passivation mode 0 state, active/inactive"),
        int_parameter(service, "PMIC1_THERM", "Ohm", "Battery charger 1 Thermistor", uint16_t),
        enum_parameter(service, "FG1_STAT", [[0, "charging"], [1, "not_charging"], [2, "charge_terminated"]] , "Fuel gauge 1 status", uint8_t),
        int_parameter(service, "FG1_SOC", "Percent", "Fuel gauge 1 battery state of charge", uint8_t),
        int_parameter(service, "FG1_THERM", "Ohm", "Fuel gauge 1 Thermistor", uint16_t),
        bool_parameter(service, "PASS_SW1_STAT", "Passivation mode 1 state, active/inactive"),
    ]
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 14},
    name="RQ_OPT_EPS_BUS",
    short_description="Request OPT_EPS_BUS Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_EPS_BUS",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 14),
    ),
    entries=[
        int_parameter(service, "V_UNREG_P",  "mW", "Satellite bus main unregulated voltage power measurement",                int16_t),
        int_parameter(service, "V3_3_BUS0_P", "mW", "Bus 0 3.3V power measurement",                int16_t),
        int_parameter(service, "V3_3_BUS1_P", "mW", "Bus 1 3.3V power measurement",                int16_t),
        int_parameter(service, "V5_BUS0_P", "mW", "Bus 0 5V power measurement",                int16_t),
        int_parameter(service, "V5_BUS1_P", "mW", "Bus 1 5V power measurement",                int16_t),
        int_parameter(service, "UNREG_BUS_P", "mW", "Bus unregulated voltage power measurement",                int16_t),
        int_parameter(service, "V3_3_USER0_P", "mW", "User 0 3.3V power measurement",                int16_t),
        int_parameter(service, "V3_3_USER1_P", "mW", "User 1 3.3V power measurement",                int16_t),
        int_parameter(service, "V3_3_USER2_P", "mW", "User 2 3.3V power measurement",                int16_t),
        int_parameter(service, "V5_USER0_P", "mW", "User 0 5V power measurement",                int16_t),
        int_parameter(service, "V5_USER1_P", "mW", "User 1 5V power measurement",                int16_t),
        int_parameter(service, "UNREG_USER_P", "mW", "User unregulated power measurement",                int16_t),
    ]
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 15},
    name="RQ_OPT_OBC_CERT",
    short_description="Request OPT_OBC_CERT Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_OBC_CERT",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 15),
    ),
    entries=[
        *multi_int_parameter(service, "SW_CERT0", 5,  "", "authorized sw. updater Certificate", encoding= uint32_t),

    ]
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 16},
    name="RQ_OPT_COM",
    short_description="Request OPT_COM Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_COM",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 16),
    ),
    entries=[
        enum_parameter(service, "VCOM0_MODE", [[0, "tbd0"], [1, "tbd1"], [2, "tbd2"]], "tbd", uint8_t),
        int_parameter(service, "VCOM0_FIFO_INFO", "", "tbd", int8_t),
        enum_parameter(service, "VCOM0_INT_STAT", [[0, "tbd0"], [1, "tbd1"], [2, "tbd2"]], "tbd", uint8_t),
        enum_parameter(service, "VCOM1_MODE", [[0, "tbd0"], [1, "tbd1"], [2, "tbd2"]], "tbd", uint8_t),
        int_parameter(service, "VCOM1_FIFO_INFO", "", "tbd", int8_t),
        enum_parameter(service, "VCOM1_INT_STAT", [[0, "tbd0"], [1, "tbd1"], [2, "tbd2"]], "tbd", uint8_t),
    ]   
)

RCCNCommand(
    system=service,
    base=base_cmd,
    assignments={"subtype": 17},
    name="RQ_OPT_PAYLOAD",
    short_description="Request OPT_PAYLOAD Telemetry",
)

Container(
    system=service,
    base="/PUS/pus-tm",
    name="OPT_PAYLOAD",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 17),
    ),
    entries=[
        int_parameter(service, "APRS_STAT2", "tbd", "APRS Module operational status 2", uint16_t),
        int_parameter(service, "APRS_STAT3", "tbd", "APRS Module operational status 3", uint16_t),
        int_parameter(service, "APRS_STAT4", "tbd", "APRS Module operational status 4", uint16_t),
    ]   
)

eps_service = Service(name="EPS_Controll", system=app)
eps_service_type_id = 78
eps_service.service_id = eps_service_type_id

eps_base_cmd = Command(
    system=eps_service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": eps_service_type_id},
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 1},
    name="POWER_VCOM",
    short_description="Power VCOM unreg bus",
    arguments=[
        BooleanArgument(
            name="Power_VCOM_ARG",
            encoding=bool_t,
        ),
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 2},
    name="POWER_ANT_DEPLOY",
    short_description="Power Deploy Antenna Mechanism",
    arguments=[
        BooleanArgument(
            name="Power_ANT_DEPLOY_ARG",
            encoding=bool_t,
        ),
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 3},
    name="RQ_EPS_Battery_Status",
    short_description="Request EPS Battery Status Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_Battery_Status",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 3),
    ),
    entries=[
        int_parameter(eps_service, "PMIC0_VBUS",  "mV", "EPS digital PCB temperature",                uint16_t),
        int_parameter(eps_service, "PMIC0_ICHG",  "mA", "Battery charger 0 charge/discharge current", int16_t),
        int_parameter(eps_service, "PMIC0_VBAT",  "mV", "Battery charger 0 battery voltage",          uint16_t),
        enum_parameter(eps_service, "PMIC0_STAT", [[0, "not_charging"], [1, "pre_charge"], [2, "fast_charge"], [3, "charge_terminated"], [4, "read_out_error"]], "Battery charger 0 status", uint8_t),
        int_parameter(eps_service, "FG0_VBAT",    "mV", "Fuel gauge 0 battery voltage",               uint16_t),
        int_parameter(eps_service, "FG0_CURRENT", "mA", "EPS digital PCB temperature",                int16_t),
        int_parameter(eps_service, "FG0_PWR",     "mW", "EPS digital PCB temperature",                int16_t),
        int_parameter(eps_service, "PMIC1_VBUS",  "mV", "EPS digital PCB temperature",                uint16_t),
        int_parameter(eps_service, "PMIC1_ICHG",  "mA", "Battery charger 1 charge/discharge current", int16_t),
        int_parameter(eps_service, "PMIC1_VBAT",  "mV", "Battery charger 1 battery voltage",          uint16_t),
        enum_parameter(eps_service, "PMIC1_STAT", [[0, "not_charging"], [1, "pre_charge"], [2, "fast_charge"], [3, "charge_terminated"], [4, "read_out_error"]], "Battery charger 1 status", uint8_t),
        int_parameter(eps_service, "FG1_VBAT",    "mV", "Fuel gauge 1 battery voltage",               uint16_t),
        int_parameter(eps_service, "FG1_CURRENT", "mA", "EPS digital PCB temperature",                int16_t),
        int_parameter(eps_service, "FG1_PWR",     "mW", "EPS digital PCB temperature",                int16_t),
    ]
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 4},
    name="RQ_EPS_Bus_Power_Status",
    short_description="Request EPS_BUS Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_Bus_Power_Status",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 4),
    ),
    entries=[
        int_parameter(eps_service, "V_UNREG_V",  "mV", "Satellite bus main unreg. power voltage",                uint16_t),
        int_parameter(eps_service, "V_UNREG_I",  "mA", "Satellite bus main unreg. power current",                int16_t),
        int_parameter(eps_service, "V3_3_BUS0_V","mV", "Bus 0 3.3V voltage measurement",                         uint16_t),
        int_parameter(eps_service, "V3_3_BUS0_I","mA", "Bus 0 3.3V current measurement",                         int16_t),
        int_parameter(eps_service, "V3_3_BUS1_V","mV", "Bus 1 3.3V voltage measurement",                         uint16_t),
        int_parameter(eps_service, "V3_3_BUS1_I","mA", "Bus 1 3.3V current measurement",                         int16_t),
        int_parameter(eps_service, "V5_BUS0_V"  ,"mV", "Bus 0 5V voltage measurement",                           uint16_t),
        int_parameter(eps_service, "V5_BUS0_I"  ,"mA", "Bus 0 5V current measurement",                           int16_t),
        int_parameter(eps_service, "V5_BUS1_V"  ,"mV", "Bus 1 5V voltage measurement",                           uint16_t),
        int_parameter(eps_service, "V5_BUS1_I"  ,"mA", "Bus 1 5V current measurement",                           int16_t),
        int_parameter(eps_service, "UNREG_BUS_V","mV", "Bus unregulated voltage measurement",                    uint16_t),
        int_parameter(eps_service, "UNREG_BUS_I","mA", "Bus unregulated voltage current measurement",            int16_t),
    ]
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 5},
    name="RQ_EPS_User_Power_Status",
    short_description="Request EPS_USER Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_User_Power_Status",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 5),
    ),
    entries=[
        bool_parameter(eps_service, "V3_3_USER_SW", "User 0 3.3V switch state (true=0n, false=Off)"), 
        int_parameter(eps_service, "V3_3_USER_V", "mV", "User 0 3.3V voltage measurement", uint16_t),
        int_parameter(eps_service, "V3_3_USER_I", "mA", "User 0 3.3V current measurement", int16_t),
        bool_parameter(eps_service, "V5_USER_SW", "User 0 5V switch state (true=0n, false=Off)"), 
        int_parameter(eps_service, "V5_USER_V", "mV", "User 0 5V voltage measurement", uint16_t),
        int_parameter(eps_service, "V5_USER_I", "mA", "User 0 5V current measurement", int16_t),
        bool_parameter(eps_service, "UNREG_USER_SW", "Satellite user unregulated voltage switch state (true=0n, false=Off)"), 
        int_parameter(eps_service, "UNREG_USER_V", "mV", "User unregulated voltage measurement", uint16_t),
        int_parameter(eps_service, "UNREG_USER_I", "mA", "User unregulated current measurement", int16_t),
    ]
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 6},
    name="RQ_EPS_Temperatur",
    short_description="Request EPS Temperature Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_Temperature",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 6),
    ),
    entries=[
        int_parameter(eps_service, "PCB_DTEMP", "Celcius", "EPS digital PCB temperature", int8_t),
        *multi_int_parameter(eps_service, "PCB_ATEMP", 4, "Celcius", "OBC PCB Temperature")
    ]   
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 7},
    name="POWER_Pl_APRS",
    short_description="Power Payload APRS",
    arguments=[
        BooleanArgument(
            name="POWER_PL_APRS_ARGS",
            encoding=bool_t,
        ),
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 8},
    name="Set_Power_Sensor_Register",
    short_description="Set a Power Sensor Register",
    arguments=[
        IntegerArgument(
            name="Adress",
            encoding=uint16_t,
        ),
        IntegerArgument(
            name="Register",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Value",
            encoding=uint16_t,
        ),
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 9},
    name="Get_Power_Sensor_Register",
    short_description="Get a Power Sensor Register",
    arguments=[
        IntegerArgument(
            name="Adress",
            encoding=uint16_t,
        ),
        IntegerArgument(
            name="Register",
            encoding=uint8_t,
        ),
    ],
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="Power_Sensor_Register_Value",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 9),
    ),
    entries=[
        int_parameter(eps_service, "Power_Sensor_Address", "", "Power Sensor I2C Adress", uint16_t),
        int_parameter(eps_service, "Power_Sensor_Register", "", "Power Sensor Register", uint8_t),
        int_parameter(eps_service, "Power_Sensor_Value", "", "Power Register Content", uint16_t),  
    ]   
)


pmic_select_argument = EnumeratedArgument(
            name="PMIC_Select",
            choices= [[0, "PMIC0"], [1, "PMIC1"]],
            encoding=uint8_t,
        )

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 10},
    name="PMIC_Set_I_Charge_Limit",
    short_description="Set the PMIC I Charge Limit",
    arguments=[
        pmic_select_argument,
        # IntegerArgument(
        #     name="IChargeLimit",
        #     encoding=uint8_t,
        # ),
        EnumeratedArgument(
            name="I_Charge_Limit_Select",
            choices= [[0b0000100, "Limit_256mA"], [0b0001000, "Limit_512mA"], [0b0010000, "Limit_1024mA"], [0b0011000, "Limit_1536mA"], [0b0100000, "Limit_2048mA"]],
            # Reg04 Val:    0000100                 0001000             0010000             0011000                 0100000      
            encoding=uint8_t,
        )
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 11},
    name="PMIC_Set_I_Input_Limit",
    short_description="Set the PMIC I Input Limit",
    arguments=[
        pmic_select_argument,
        # IntegerArgument(
        #     name="InputLimit",
        #     encoding=uint8_t,
        # ),
        EnumeratedArgument(
            name="I_Input_Limit_Select",
            choices= [[0b001000, "Limit_400mA"], [0b010000, "Limit_800mA"], [0b011100, "Limit_1400mA"], [0b101000, "Limit_2000mA"], [0b110000, "Limit_2400mA"], [0b111000, "Limit_2800mA"], [0b111111, "Limit_3250mA"]],
            #  REG00 Val:       001000                  010000                   011100                               101000              110000                  111000              111111
            encoding=uint8_t,
        )
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 12},
    name="PMIC_Set_V_Charge_Limit",
    short_description="Set the PMIC V Charge Limit",
    arguments=[
        pmic_select_argument,
        # IntegerArgument(
        #     name="VChargeLimit",
        #     encoding=uint8_t,
        # ),
        EnumeratedArgument(
            name="V_Charge_Limit_Select",
            choices= [[0b000000, "Limit_3V840"], [0b000100, "Limit_3V904"], [0b001100, "Limit_4V032"], [0b010010, "Limit_4V128"], [0b010111, "Limit_4V208"], [0b100000, "Limit_4V352"], [0b100100, "Limit_4V416"], [0b101010, "Limit_4V511"], [0b110000, "Limit_4V608"]],
            # REG06       # 0b000000              0b000100                       0b001100               0b010010                       0b011010            0b100000                       0b100100                  0b101010                      0b110000
            encoding=uint8_t,
        )
    ],
)


pmic_register_argument = IntegerArgument(
            name="PMIC_Register",
            encoding=uint8_t,
        )

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 13},
    name="PMIT_Set_Register",
    short_description="Set a PMIC Register",
    arguments=[
        pmic_select_argument,
        pmic_register_argument,
        IntegerArgument(
            name="PMIC_Value",
            encoding=uint8_t,
        ),
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 14},
    name="PMIC_Get_Register",
    short_description="Get a PMIC Register",
    arguments=[
        pmic_select_argument,
        pmic_register_argument,
    ],
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="PMIC_Register_Value",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 14),
    ),
    entries=[
        enum_parameter(eps_service, "PMIC", [[0, "PMIC0"], [1, "PMIC1"]], "PMIC Selection", uint8_t),
        int_parameter(eps_service, "PMIC_Register", "", "PMIC Register", uint8_t),
        int_parameter(eps_service, "PMIC_Value", "", "PMIC Register Content", uint8_t),  
    ]   
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 15},
    name="RQ_EPS_Battery_Config",
    short_description="Get EPS Battery Config Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_Battery_Config",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 15),
    ),
    entries=[
        bool_parameter(eps_service, "PASS_SW0_Passivation_state", "Passivation mode 0 state, active/inactive"),
        bool_parameter(eps_service, "PASS_SW0_Persitant", "Passivation mode 0 Persitant"),
        bool_parameter(eps_service, "PASS_SW1_Passivation_state", "Passivation mode 1 state, active/inactive"),
        bool_parameter(eps_service, "PASS_SW1_Persitant", "Passivation mode 1 Persitant"),
        enum_parameter(eps_service, "PMIC0_I_Charge_Limit_Select", [[0xFF, "Undefined"], [0b0000100, "Limit_256mA"], [0b0001000, "Limit_512mA"], [0b0010000, "Limit_1024mA"], [0b0011000, "Limit_1536mA"], [0b0100000, "Limit_2048mA"]], "PMIC0 Selected Charge Current", uint8_t),
        enum_parameter(eps_service, "PMIC0_I_Input_Limit_Select",  [[0xFF, "Undefined"], [0b001000, "Limit_400mA"],  [0b010000, "Limit_800mA"], [0b011100, "Limit_1400mA"], [0b101000, "Limit_2000mA"], [0b110000, "Limit_2400mA"], [0b111000, "Limit_2800mA"], [0b111111, "Limit_3250mA"]], "PMIC0 Selected Input Limit", uint8_t),
        enum_parameter(eps_service, "PMIC0_V_Charge_Limit_Select", [[0xFF, "Undefined"], [0b000000, "Limit_3V840"],  [0b000100, "Limit_3V904"], [0b001100, "Limit_4V032"], [0b010010, "Limit_4V128"], [0b010111, "Limit_4V208"], [0b100000, "Limit_4V352"], [0b100100, "Limit_4V416"], [0b101010, "Limit_4V511"], [0b110000, "Limit_4V608"]], "PMIC0 Selected Charge Voltage", uint8_t),
        enum_parameter(eps_service, "PMIC1_I_Charge_Limit_Select", [[0xFF, "Undefined"], [0b0000100, "Limit_256mA"], [0b0001000, "Limit_512mA"], [0b0010000, "Limit_1024mA"], [0b0011000, "Limit_1536mA"], [0b0100000, "Limit_2048mA"]], "PMIC1 Selected Charge Current", uint8_t),
        enum_parameter(eps_service, "PMIC1_I_Input_Limit_Select",  [[0xFF, "Undefined"], [0b001000, "Limit_400mA"],  [0b010000, "Limit_800mA"], [0b011100, "Limit_1400mA"], [0b101000, "Limit_2000mA"], [0b110000, "Limit_2400mA"], [0b111000, "Limit_2800mA"], [0b111111, "Limit_3250mA"]], "PMIC1 Selected Input Limit", uint8_t),
        enum_parameter(eps_service, "PMIC1_V_Charge_Limit_Select", [[0xFF, "Undefined"], [0b000000, "Limit_3V840"],  [0b000100, "Limit_3V904"], [0b001100, "Limit_4V032"], [0b010010, "Limit_4V128"], [0b010111, "Limit_4V208"], [0b100000, "Limit_4V352"], [0b100100, "Limit_4V416"], [0b101010, "Limit_4V511"], [0b110000, "Limit_4V608"]], "PMIC1 Selected Charge Voltage", uint8_t),
    ]   
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 16},
    name="Set_Passivation_Sw_State",
    short_description="Set the Passivation Switch State",
    arguments=[
        EnumeratedArgument(
            name="Switch_Select",
            choices=  [[0, "Switch_0"], [1, "Switch_1"]],
            encoding=uint8_t,
        ),
        BooleanArgument(
            name="Passivation_State",
            encoding=bool_t,
            short_description="False: Battery charging active (NO EoL-PASSIVATION), \nTrue: Battery charging inactive (EoL-PASSIVATION)"
        ),
        BooleanArgument(
            name="Persitant",
            encoding=bool_t,
        ),
    ]
)

integer_type = IntegerDataType(
    encoding=uint8_t,
    signed=True,
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 17},
    name="Set_Register",
    short_description="Set a Register",
    arguments=[
        IntegerArgument(
            name="Address",
            encoding=uint16_t
        ),
        IntegerArgument(
            name="Register",
            encoding=uint8_t
        ),
        IntegerArgument(
            name="Length",
            encoding=uint8_t
        ),
        IntegerArgument(name="Byte00", encoding=uint8_t),
        IntegerArgument(name="Byte01", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte02", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte03", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte04", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte05", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte06", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte07", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte08", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte09", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte10", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte11", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte12", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte13", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte14", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte15", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte16", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte17", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte18", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte19", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte20", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte21", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte22", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte23", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte24", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte25", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte26", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte27", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte28", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte29", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte30", encoding=uint8_t, default=0),
        IntegerArgument(name="Byte31", encoding=uint8_t, default=0),
        
    ],
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 18},
    name="Get_Register",
    short_description="Get a Register",
    arguments=[
        IntegerArgument(
            name="Adress",
            encoding=uint16_t,
        ),
        IntegerArgument(
            name="Register",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Length",
            encoding=uint8_t,
        ),
    ],
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="Register_Value_TM",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 18),
    ),
    entries=[
        int_parameter(eps_service, "Address", "", "I2C Adress", uint16_t),
        int_parameter(eps_service, "Register", "", "Register", uint8_t),
        # ParameterEntry(IntegerParameter(system=service, name="n", encoding=uint8_t)),
        ParameterEntry(
            IntegerParameter(
                system=eps_service,
                name="n",
                encoding=uint8_t
            )
        ),
        ParameterEntry(
            ArrayParameter(
                system=eps_service,
                name="ReadRegisterValues",
                length=DynamicInteger("n"),
                data_type=IntegerDataType(
                    encoding=uint8_t,
                    signed=True,
                ),
            )
        )
    ]   
)

solar_panels = ["x-", "z+", "y-", "x+", "x+", "x+", "x+", "x+"]


RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 19},
    name="RQ_EPS_CSA_SOL",
    short_description="Request EPS_CSA_SOL Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_CSA_SOL",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 19),
    ),
    entries=[
        *multi_int_parameter(eps_service, "CSA_SOL", 8, "mA", short_description_base = "Analog solar input current - Direction (TBC): ", description_list = solar_panels),
    ]
)

RCCNCommand(
    system=eps_service,
    base=eps_base_cmd,
    assignments={"subtype": 20},
    name="RQ_EPS_TEMP_ALX_SOL",
    short_description="Request OPT_EPS_SOL Telemetry",
)

Container(
    system=eps_service,
    base="/PUS/pus-tm",
    name="EPS_TEMP_ALX_SOL",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", eps_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 20),
    ),
    entries=[
        *multi_int_parameter(eps_service, "ATEMP_SOL", 7, "Deci Celcius", "Analog temperature of solar panel - Direction (TBC):", solar_panels, int16_t),
        *multi_int_parameter(eps_service, "ALX_SOL", 7, "light intensity", "Analog solar panel illuminance - Direction (TBC):", solar_panels, int16_t)
    ]
)

RTC_service = Service(name="RTC", system=app)
RTC_service_type_id = 79
RTC_service.service_id = RTC_service_type_id

rtc_base_cmd = Command(
    system=RTC_service,
    name="base",
    abstract=True,
    base="/PUS/pus-tc",
    assignments={"type": RTC_service_type_id},
)

RCCNCommand(
    system=RTC_service,
    base=rtc_base_cmd,
    assignments={"subtype": 1},
    name="RTC_Software_Reset",
    short_description="Perform RTC Software Reset"
)

RCCNCommand(
    system=RTC_service,
    base=rtc_base_cmd,
    assignments={"subtype": 2},
    name="RTC_Set_Time",
    short_description="Set RTC Time",
    arguments=[
        IntegerArgument(
            name="SecondFrac100th",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Seconds",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Minutes",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Hours",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Day",
            encoding=uint8_t,
        ),
        EnumeratedArgument(
            name="Weekday",
            choices= [[0b000, "Sunday"], [0b001, "Monday"], [0b010, "Tuesday"], [0b011, "Wednesday"], [0b100, "Thursday"], [0b101, "Friday"], [0b110, "Saturday"]],
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Month",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Year",
            encoding=uint8_t,
        ),
    ],
)


RCCNCommand(
    system=RTC_service,
    base=rtc_base_cmd,
    assignments={"subtype": 3},
    name="RTC_Read_Time",
    short_description="Read current RTC Time"
)

Container(
    system=RTC_service,
    base="/PUS/pus-tm",
    name="RTC_Time",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", RTC_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 3),
    ),
    entries=[
        int_parameter(RTC_service, "SecondFrac100th",  "", "100th Fraction of a second",                uint8_t),
        int_parameter(RTC_service, "Seconds",  "", "Current Set Seconds", uint8_t),
        int_parameter(RTC_service, "Minutes",  "", "Current Set Minutes", uint8_t),
        int_parameter(RTC_service, "Hours",  "", "Current Set Hours", uint8_t),
        int_parameter(RTC_service, "Day",  "", "Current Set Days", uint8_t),
        enum_parameter(RTC_service, "Weekday", [[0b000, "Sunday"], [0b001, "Monday"], [0b010, "Tuesday"], [0b011, "Wednesday"], [0b100, "Thursday"], [0b101, "Friday"], [0b110, "Saturday"]], "Current set day of the week", uint8_t),
        int_parameter(RTC_service, "Month",  "", "Current Set Month", uint8_t),
        int_parameter(RTC_service, "Year",  "", "Current Set Year", uint8_t),
    ]
)

RCCNCommand(
    system=RTC_service,
    base=rtc_base_cmd,
    assignments={"subtype": 4},
    name="RTC_Set_Register",
    short_description="Set RTC Register",
    arguments=[
        IntegerArgument(
            name="Register",
            encoding=uint8_t,
        ),
        IntegerArgument(
            name="Value",
            encoding=uint8_t,
        ),
    ],
)

RCCNCommand(
    system=RTC_service,
    base=rtc_base_cmd,
    assignments={"subtype": 5},
    name="RTC_Read_Register",
    short_description="Read RTC Register",
    arguments=[
        IntegerArgument(
            name="Register",
            encoding=uint8_t,
        ),
    ],
)

Container(
    system=RTC_service,
    base="/PUS/pus-tm",
    name="RTC_Register",
    condition=AndExpression(
        EqExpression("/PUS/pus-tm/type", RTC_service_type_id),
        EqExpression("/PUS/pus-tm/subtype", 5),
    ),
    entries=[
        int_parameter(RTC_service, "Register",  "", "Register ID", uint8_t),
        int_parameter(RTC_service, "Value",  "", "Register Value", uint8_t),
    ]
)




# app.generate_rccn_code()
with open("bix1_tmtc.xml", "wt") as f:
  app.dump(f)
  
