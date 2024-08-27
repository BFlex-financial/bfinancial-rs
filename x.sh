curl --request POST \
  --url http://127.0.0.1:8080/payment/create \
  --header 'Authorization-key: admin' \
  --header 'Content-Type: application/json' \
  --data '{
	"amount": 0.21,
	"method": "Pix",
	"payer_email": "xaxuxogasas@gmail.com"
}'

read 