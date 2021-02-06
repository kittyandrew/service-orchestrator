curl -X POST -d '{"data": 1}' -H "Content-Type: application/json" -H "X-TOKEN: secrettoken" -H "X-SERVICE: cool_thing" -H "X-URL: https://coolstuff.service.dev" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward
echo -e

result=$(curl -H "Content-Type: application/json" -H "X-TOKEN: secrettoken" -H "X-SERVICE: cool_thing" -H "X-URL: https://google.com"  http://0.0.0.0:8000/subscription/new)

token=$(echo $result | jq -r .new_token)
echo -e
echo -e "Token: $token\n"

out=$(curl -X POST -d '{"data": 1}' -H "Content-Type: application/json" -H "X-TOKEN: $token" -H "X-SERVICE: cool_thing" -H "X-TARGET-SERVICE: cool_thing" http://0.0.0.0:8000/subscription/forward)
echo $out
echo -e
