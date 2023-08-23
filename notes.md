## Client:
- [x] Simulate locally with Turtle
- [x] Convert turtle program into move set
- [x] Connect to web UI and submit moveset 

## Frontend Server:
- [x] Web Interface
    - [ ] Show title, show server status, etc
    - [ ] Admin page to cancel, clear queue
    - [ ] Make CSS a tiny bit less barebones
- [ ] Authenticate with session key
- [x] Receive gcode from clients, queue for execution
- [ ] Send gcode jobs to server
- [ ] Get machine details from server

## GCode Host:
- [x] Read machine details from config
- [x] GCode type system
- [ ] Serial port communicator
    - [ ] GCode Sending
    - [ ] Pausing
- [ ] Convert uploaded movements into GCode
- [ ] Simulate move set
- [ ] HTTP Endpoints:
    - [x] Send Gcode
    - [ ] Run Gcode
    - [ ] Pause
    - [ ] Start simulation
    - [ ] Get simulation results
    - [ ] Get status
    - [ ] Get machine details
