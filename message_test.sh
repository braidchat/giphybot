#!/bin/bash

set -euo pipefail

set -x

curl --verbose -X PUT -H 'X-Braid-Signature: foobar' --data-binary @test_giphy_msg.msgpack localhost:9999/message
