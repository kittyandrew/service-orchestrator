TOKEN="testtoken" cargo test

: '
out=$(curl -s -H "Content-Type: application/json" -H "X-TOKEN: secrettoken" http://0.0.0.0:8000)
echo -e
if [ -z "$out" ]; then
    echo "Could not connect to the server! Start it first."
    echo -e
    exit 1
else
    echo -e "Hello message:"
    echo $out
fi

out=$(curl -s -X POST -d '{"data": 1}' -H "Content-Type: application/json" -H "X-SERVICE: cool_thing" -H "X-TOKEN: secrettoken" -H "X-URL: https://coolstuff.service.dev" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo -e
echo -e "Expected to fail:"
echo $out

result=$(curl -s -H "X-EXPECTED-SCHEMA: schema" -H "X-SERVICE: cool_thing" -H "X-TOKEN: secrettoken" -H "X-URL: https://google.com" http://0.0.0.0:8000/subscription/new)
token=$(echo $result | jq -r .new_token)
echo -e
echo -e "Registered:"
echo -e "New Token: $token"

out=$(curl -s -X POST -d '"cool_long"' -H "Content-Type: application/json" -H "X-SERVICE: cool_thing" -H "X-TOKEN: $token" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo -e
echo -e "Expected to fail validation:"
echo $out

out=$(curl -s -X POST -d '"cool"' -H "Content-Type: application/json" -H "X-SERVICE: cool_thing" -H "X-TOKEN: $token" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo -e
echo -e "Success:"
echo $out
echo -e
'
