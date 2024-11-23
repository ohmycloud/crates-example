curl --location --request POST 'localhost:5000' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        {
              "calculation": "perimeter",
              "shape": "circle",
              "radius": 2.3
         }
}'
