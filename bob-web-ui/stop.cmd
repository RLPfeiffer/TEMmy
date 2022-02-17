FOR /F "tokens=*" %%a in ('docker ps -a -q  --filter "ancestor=python-docker"') do SET OUTPUT=%%a
docker stop %OUTPUT%
docker container prune -f