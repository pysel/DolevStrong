FROM cargo:latest

WORKDIR /app 

COPY . .

ARG CONFIG_INDEX
ARG CONFIG_PATH

CMD [ "cargo", "run" , $CONFIG_PATH $CONFIG_INDEX]