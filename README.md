# rocket-backend-template
 A simple mvc template that uses Rocket(Rust) for the backend + mongodb with JWTs for authentication. All of this is wrapped in a dockerfile, with a docker-compose file to spin up the backend server and the mongodb server. This is expecting you to have a MongoDB docker container already set up, which can be done as easy as running `docker pull mongo`

## Environment Vars
The secret for the JWT is defined in the docker-compose file, though this should be regularly changed/cycled for security. And the mongo uri is also defined in that same YAML file.

## How to Run
run `docker build -t rocket-backend-template .` for the docker image to be made

then run `docker-compose up --build` to run the docker image, which will be exposed on port 3000 to your local machine