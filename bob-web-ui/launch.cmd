docker build --tag python-docker .
docker run -d -p 5000:5000 python-docker
