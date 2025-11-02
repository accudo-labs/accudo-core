This is a small example of using the new `accudo` dependency. This shall be removed once we have
documentation/tests.

`pack2` contains a package which is used by `pack1` as follows:

```
[dependencies]
Pack2 = { accudo = "http://localhost:8080", address = "default" }
```

To see it working:

```shell
# Start a node with an account
accudo node run-local-testnet &
accudo account create --account default --use-faucet 
# Compile and publish pack2
cd pack2
accudo move compile --named-addresses project=default     
accudo move publish --named-addresses project=default
# Compile pack1 agains the published pack2
cd ../pack1
accudo move compile --named-addresses project=default     
```
