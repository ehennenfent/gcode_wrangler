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
- [x] Serial port communicator
    - [x] GCode Sending
    - [x] Pausing
- [x] Convert uploaded movements into GCode
- [x] Simulate move set
- [x] HTTP Endpoints:
    - [x] Send Gcode
    - [x] Run Gcode
    - [x] Pause
    - [x] Get simulation results
    - [x] Get status
    - [x] Get machine details
