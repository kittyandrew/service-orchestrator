
out=$(curl -s -H "Content-Type: application/json" -H "X-SERVICE: cool_thing" -H "X-TOKEN: secrettoken" http://0.0.0.0:8000)
echo -e
if [ -z $out ]; then
    echo "Could not connect to the server! Start it first."
    echo -e
    exit 1
else
    echo $out
fi

out=$(curl -s -X POST -d '{"data": 1}' -H "Content-Type: application/json" -H "X-TOKEN: secrettoken" -H "X-SERVICE: cool_thing" -H "X-URL: https://coolstuff.service.dev" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo -e
echo $out

result=$(curl -s -H "Content-Type: application/json" -H "X-TOKEN: secrettoken" -H "X-SERVICE: cool_thing" -H "X-URL: https://google.com"  http://0.0.0.0:8000/subscription/new)

token=$(echo $result | jq -r .new_token)
echo -e
echo -e "Token: $token\n"

out=$(curl -s -X POST -d '"cool_long"' -H "Content-Type: application/json" -H "X-TOKEN: $token" -H "X-SERVICE: cool_thing" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo $out
echo -e

out=$(curl -s -X POST -d '"cool"' -H "Content-Type: application/json" -H "X-TOKEN: $token" -H "X-SERVICE: cool_thing" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo $out
echo -e

