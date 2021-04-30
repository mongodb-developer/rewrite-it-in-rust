#!/bin/bash
curl -i \
    -H "Content-Type: application/json" \
    --request 'PUT' \
    --data '{
        "slug": "whiskey-new-fashioned",
        "name": "Whiskey New-Fashioned",
        "ingredients": [
            {
                "name": "New Whiskey"
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
    'http://localhost:5000/cocktails/whiskey-old-fashioned'