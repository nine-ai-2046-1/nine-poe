# POE_API_KEY="Suppose to read from global env variable"
MODEL="Mistral-Small-4"
PROMPT="Hello world"

curl "https://api.poe.com/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $POE_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "$MODEL",
    "messages": [
      {
        "role": "user",
        "content": "$PROMPT"
      }
    ]
  }'