FROM postgres

RUN apt-get update
RUN apt-get -y install git make gcc
RUN ls -la
RUN git clone https://github.com/theory/pg-semver.git
RUN ls -la 
WORKDIR pg-semver
RUN ls -la 
RUN make && make install && make installcheck

WORKDIR ..

RUN psql -d $POSTGRES_DB -c 'CREATE EXTENSION semver;'