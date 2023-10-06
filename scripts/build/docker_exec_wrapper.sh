#!/bin/bash
CI_HOSTNAME=ip-172-31-63-27
IMAGE=duvisor/build-env:v2

if [ $(hostname)1 == ${CI_HOSTNAME}1 ]; then
    # for CI environment
    IT=""
    EXTRA_V=" -v /home/ubuntu/prepare:/home/ubuntu/prepare "
else
    IT=" -it "
fi

docker run ${IT} --rm \
	-v $(pwd):/home/$(id -u -n)/duvisor \
	-w /home/$(id -u -n)/duvisor \
	-u root \
	--network=host \
	-e PATH="/root/.cargo/bin:${PATH}" \
	${EXTRA_V} \
	${IMAGE} $1
