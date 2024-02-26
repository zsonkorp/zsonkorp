#!/bin/bash

#pipe to dev null for now, need to hook up tp cloud watch
nohup /home/ec2-user/bin/zsonkorp > /dev/null 2>&1 &