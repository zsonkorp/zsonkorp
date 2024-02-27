#!/bin/bash

aws s3 cp s3://zsonkorp-game-artifact/zsonkorp /home/ec2-user/app/bin/
chmod +x /home/ec2-user/app/bin/zsonkorp