#!/bin/sh

rsync -e "ssh -p 38901 -o StrictHostKeyChecking=no" --recursive web/ ryne.moe:/srv/http/auracite.xiv.zone/ &&
rsync -e "ssh -p 38901 -o StrictHostKeyChecking=no" --recursive pkg/ ryne.moe:/srv/http/auracite.xiv.zone/pkg/
