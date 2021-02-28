FROM node:lts-alpine

WORKDIR /usr/src/website/frontend
COPY . .

RUN npm install
RUN npm run build:app

CMD ["npm run serve"]
