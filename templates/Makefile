.PHONY: build start
build:
	wash reg push --insecure localhost:5000/v2/thread:0.1.0 ../test/cap/game-provider.par.gz
	wash reg push --insecure localhost:5000/v2/nats_messaging:0.1.0 ../test/cap/nats_messaging.par.gz
	(cd actors/game_logic && make clean_wasm && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS configs/manifest.yaml
	sleep 8
start:
	(cd actors/game_logic && make start-thread)