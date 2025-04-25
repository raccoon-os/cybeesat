.PHONY: build install commit debug-symbols

INSTALL_DIR ?= install

OSTREE_REPO ?= build/tmp/deploy/images/sama5d27-som1-ek-sd/ostree_repo
DELTA_FROM  ?= e7ddfddf9456988c57e725730aca412d041b9a67bea7bc2fa7c834d6a786fd01 
BRANCH_NAME ?= cybeesat
DELTA_TO    ?= cybeesat

BINARIES := rccn_usr_comm \
            rccn_usr_launch \
            rccn_usr_cfdp \
            rccn_usr_update \
            rccn_usr_fec \
            serial_bridge \
            boot_app \
            deploy_app \
            diagnosis_app \
            aprs_app \
            bix1_ops_app \
            vcom_interface

build:
	cross build \
		$(foreach bin,$(BINARIES),--bin $(bin)) \
		--target armv7-unknown-linux-musleabihf \
		--release


install: build
	cp target/armv7-unknown-linux-musleabihf/release/{$(shell echo $(BINARIES) | tr ' ' ',')} \
		install/usr/bin

	cp python/antenna_control.py install/usr/bin
	cp python/leop.py install/usr/bin

debug-symbols:
	mkdir -p debug
	$(foreach bin,$(BINARIES),\
		arm-none-eabi-objcopy --only-keep-debug install/usr/bin/$(bin) debug/$(bin).dbg && \
		arm-none-eabi-strip install/usr/bin/$(bin) ; \
	)

commit: deploy 
	ostree commit --repo=${OSTREE_REPO} --branch="${BRANCH_NAME}" --tree=dir=./install

delta: commit
	ostree commit \
		--repo=${OSTREE_REPO} \
		--tree=ref=${DELTA_FROM} \
		--tree=ref=${BRANCH_NAME} \
		--branch=${BRANCH_NAME}

	ostree static-delta generate \
		--repo=${OSTREE_REPO} \
		--from=${DELTA_FROM} \
		--to=${DELTA_TO} \
		--min-fallback-size=0 

	tar -czvf delta.tar.gz --transform "s,.*/,," ${OSTREE_REPO}/deltas/**/*
	rm -rf ${OSTREE_REPO}/deltas


clean:
	rm -rf debug
