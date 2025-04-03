FROM grafana/k6:latest

# Create app directory
WORKDIR /app

# Copy our test scripts and configuration
COPY scripts/ /app/scripts/

# Set the entrypoint to run our start script with the shell
ENTRYPOINT ["/bin/sh", "/app/scripts/start.sh"]

# By default, run with no arguments (which will use main.js)
CMD [] 