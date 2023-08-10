#!/bin/bash
npx -p @apollo/rover rover graph introspect https://api.sorare.com/federation/graphql > core/graphql/schema.graphql