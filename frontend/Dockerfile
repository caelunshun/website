FROM node:lts-alpine

WORKDIR /usr/src/website/frontend
COPY . .

RUN npm install
run npm build:ssr

CMD ["npm run serve"]