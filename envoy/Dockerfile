FROM envoyproxy/envoy:v1.22.0

COPY ./envoy-template.yaml /etc/envoy/envoy-template.yaml
RUN apt -qq update
RUN apt install -y gettext
RUN chmod -R 777 /etc/envoy

CMD /usr/local/bin/envoy -c /etc/envoy/envoy-template.yaml -l trace --log-path /tmp/envoy_info.log