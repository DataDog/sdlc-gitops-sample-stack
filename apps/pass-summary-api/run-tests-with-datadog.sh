#!/bin/bash

curl -L https://dtdg.co/latest-java-tracer -o $HOME/dd-java-agent.jar

export DD_CIVISIBILITY_AGENTLESS_ENABLED=true
export DD_CIVISIBILITY_ENABLED=true
export DD_SITE=datadoghq.eu
export DD_ENV=ci
export DD_SERVICE=downstream-service

if [ -n "$DD_API_KEY" ]; then
    export MAVEN_OPTS=-javaagent:$HOME/dd-java-agent.jar
else
    echo "No DD_API_KEY - can't submit results to Datadog"
fi

mvn clean verify
