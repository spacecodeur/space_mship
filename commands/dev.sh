#!/bin/bash

## build containers

./commands.sh commands/services/app/container-management/build.sh &
./commands.sh commands/services/db/container-management/build.sh &
wait

## run some scripts like migrations, wasm building, ...

./commands.sh commands/services/app/in-container/migrate/up-all.sh
./commands.sh commands/services/app/in-container/wasm-build.sh

## finish !

echo
echo "All set!"
echo "Next step : ./commands.sh commands/services/app/in-container/server-start.sh"
echo "Happy coding! :D"