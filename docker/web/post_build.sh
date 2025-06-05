#!/bin/bash

echo "alias ll='ls -all'" >> /home/appuser/.bashrc

tail -f /dev/null # keep the container running after build