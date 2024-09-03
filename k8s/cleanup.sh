#!/bin/bash

multipass delete --purge k3s-master
multipass delete --purge worker01
multipass delete --purge worker02
