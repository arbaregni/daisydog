FROM node:21-alpine
WORKDIR /app
COPY . .
RUN ["npm", "install"]
RUN ["npm", "run", "build"]
CMD ["npm", "run", "start", "--", "--host", "0.0.0.0", "--port", "8080"]
