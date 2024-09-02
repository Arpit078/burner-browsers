FROM burner-browser-base
ARG PASSWORD
RUN mkdir -p /home/alpine/.vnc && x11vnc -storepasswd ${PASSWORD} /home/alpine/.vnc/passwd
COPY entry.sh /entry.sh

CMD [ "/bin/bash", "/entry.sh" ]