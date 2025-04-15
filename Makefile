.PHONY: deploy install commit

DEPLOY_DIR ?= deploy
INSTALL_DIR ?= install

OSTREE_REPO ?= build/tmp/deploy/images/sama5d27-som1-ek-sd/ostree_repo
DELTA_FROM  ?= e7ddfddf9456988c57e725730aca412d041b9a67bea7bc2fa7c834d6a786fd01 
BRANCH_NAME ?= cybeesat
DELTA_TO    ?= cybeesat

deploy:
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_comm rccn_usr_comm
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_launch rccn_usr_launch
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_example_app rccn_usr_example_app
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_cfdp rccn_usr_cfdp
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_update rccn_usr_update
	./cross.sh cargo install --path rust/rccn-usr/src/rccn_usr_fec rccn_usr_fec
	./cross.sh cargo install --path rust/serial_bridge serial_bridge
	./cross.sh cargo install --path rust/boot_app boot_app
	./cross.sh cargo install --path rust/deploy_app deploy_app
	./cross.sh cargo install --path rust/diagnosis_app diagnosis_app
	./cross.sh cargo install --path rust/aprs_app aprs_app
	# ./cross.sh cargo install --path rust/health_app health_app

	cp python/antenna_control.py install/usr/bin

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
	rm -rf ${DEPLOY_DIR}

