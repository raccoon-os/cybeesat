-- create stream cfdp_in as select substring(packet, 6) as pdu from tm_realtime where extract_short(packet, 0) = 222
create stream cfdp_out (gentime TIMESTAMP, entityId long, seqNum int, pdu  binary)
create stream cfdp_in as select substring(packet, 6) as pdu from cfdp_udp_tm

-- insert into tc_realtime select gentime, 'cfdp-service' as origin, seqNum, '/yamcs/cfdp/upload' as cmdName, unhex('17FDC0000000') + pdu as binary from cfdp_out
insert into cfdp_udp_tx select gentime, 'cfdp-service' as origin, seqNum, '/yamcs/cfdp/upload' as cmdName, pdu as binary from cfdp_out
