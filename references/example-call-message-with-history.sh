# POE_API_KEY="Suppose to read from global env variable"
MODEL="Mistral-Small-4"
PROMPT="Let's continue our conversation from the previous messages."

curl "https://api.poe.com/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $POE_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "$MODEL",
    "messages": [
      {
        "role": "user",
        "content": "Good morning"
      },
      {
        "role": "assistant",
        "content": "Hello! How can I assist you today?"
      },
      {
        "role": "user",
        "content": "$PROMPT"
      }
    ]
  }'