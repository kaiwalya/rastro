FROM ubuntu:22.04
RUN apt-get update && apt-get install -y software-properties-common locales && apt-get update
RUN localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8
ENV LANG en_US.utf8

RUN apt-add-repository ppa:mutlaqja/ppa && apt-get update && apt-get install -y indi-full gsc

CMD indiserver -v indi_simulator_ccd indi_simulator_telescope
