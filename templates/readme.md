nats-server -c configs/websocket_lite.conf -js -D -VV

WASMCLOUD_HOST_SEED=SNAKDMBLB7TPIL4K3YXDGLUDXYFEEB2UAUXSAJYFBUKAWXBT6VPSTSE34Y WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000 WASMCLOUD_CLUSTER_SEED=SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E wasmcloud_host start

WASMCLOUD_HOST_SEED=SNAKDMBLB7TPIL4K3YXDGLUDXYFEEB2UAUXSAJYFBUKAWXBT6VPSTSE34Y WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000 WASMCLOUD_CLUSTER_SEED=SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E nohup wasmcloud_host start &