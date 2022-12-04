#!/bin/bash
npx -p @apollo/rover rover graph introspect https://api.sorare.com/graphql > schema.graphql
#npx -p @apollo/rover rover graph introspect https://api.sorare.com/sports/graphql > schema-us-sports.graphql
