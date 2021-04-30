#!/bin/bash
curl -i \
    -H "Content-Type: application/json" \
    --request 'POST' \
    --data '{
        "slug": "whiskey-old-fashioned",
        "name": "Whiskey Old-Fashioned",
        "ingredients": [
            {
                "name": "Whiskey"
            },
            {
                "name": "Sugar Syrup"
            },
            {
                "name": "Angostura Bitters"
            }
        ],
        "instructions": [
            "Stir the ingredients over ice.",
            "Express the orange peel over the drink."
        ]
    }' \
    'http://localhost:5000/cocktails/'
