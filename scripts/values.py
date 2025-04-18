import obd

connection = obd.OBD(portstr="socket://localhost:5054")
cmd = obd.commands[1][12]
response = connection.query(cmd)
print(response.value)