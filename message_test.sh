#!/bin/bash

set -euo pipefail

set -x

curl --verbose -X PUT -H 'X-Braid-Signature: foobar' --data-binary @test-message2.msgpack localhost:9999/message
