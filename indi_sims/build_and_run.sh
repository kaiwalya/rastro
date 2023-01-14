docker build -t indi_sims .
docker run --rm -it --name indi_sims -p 7624:7624 indi_sims indiserver -vvv indi_simulator_ccd indi_simulator_telescope
