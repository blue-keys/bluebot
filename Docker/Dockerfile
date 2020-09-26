# Exécuter spartiate sur le port 9080
#sudo docker run -d -p 9080:9080 spartiate:1.0.2
#
#Permet de voir la liste des container :
#sudo docker ps -a
#
#Supprime un container par id
#sudo docker rm
#
#Liste des images
#sudo docker images -a
#
#Liste des container
#sudo docker ps -a
#
#création de l’image depuis le dockerfile
#sudo docker build -t spartiate .

# docker stop $(docker ps -a -q)
# docker rm $(docker ps -a -q)
# docker system prune

FROM ubuntu:latest

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# log initial packages list
RUN dpkg -l | sed "s/^/dpkg-list-before /g"

# ubuntu packages installation
RUN apt-get update && \
    apt upgrade -y

RUN apt-get install -y --no-install-recommends \
    curl=* \
    bzip2=* \
    git=1:2.* \
    jq=1.* \
    libxml2-utils=2.* \
    unzip=* \
    python3-pip=* \
    nodejs=* \
    npm=* \
    rustc=*

ENV PATH="/root/.cargo/bin:${PATH}"

# configure packages
RUN ln -snf /usr/share/zoneinfo/Europe/Paris /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata && \
    ln -s /usr/games/cowsay /usr/bin/cowsay

RUN npm i -g envinfo && rustc --version && cargo --version

# clean image
RUN apt-get autoremove -y && \
    apt-get clean && \
    rm -r /var/lib/apt/lists/

RUN cargo install --path .

# select default home dir
WORKDIR /usr/src/discord-bot-rust

# select default shell
CMD ["/usr/local/cargo/bin/discord-bot-rust"]
