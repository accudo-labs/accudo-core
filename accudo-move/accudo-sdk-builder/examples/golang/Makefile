#Copyright © Accudo Foundation
# SPDX-License-Identifier: Apache-2.0

clean:
	- rm -rf go.mod go.sum accudostdlib accudotypes

test:
	cargo run -p accudo-framework release
	cargo run -p accudo-sdk-builder -- \
		--language go --module-name accudostdlib \
		--package-name testing \
		--with-accudo-types "../../../../testsuite/generate-format/tests/staged/accudo.yaml" \
		--target-source-dir . \
		"../../../framework/accudo-framework/build/AccudoFramework"
	- go mod init testing
	go mod edit -replace testing=`pwd`
	go get github.com/accudo-labs/serde-reflection/serde-generate/runtime/golang/bcs
	go get github.com/accudo-labs/serde-reflection/serde-generate/runtime/golang/serde
	go run stdlib_demo.go

.PHONY: clean test
