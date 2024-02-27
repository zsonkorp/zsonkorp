#!/bin/bash

aws s3 cp s3://arn:aws:s3:::zsonkorp-game-artifact/zsonkorp /home/ec2-user/app/bin/zsonkorp
chmod -x /home/ec2-user/app/bin/zsonkorp